//! Source-faithful aleatory and epistemic uncertainty.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::error::ProvenanceError;
use super::identifiers::{
    CorrelationGroupId, ModelRealizationId, validate_finite, validate_finite_map,
    validate_probability_level, validate_text,
};

/// The source-native representation of uncertainty.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "UncertaintyRepresentationWire")]
pub enum UncertaintyRepresentation {
    /// Symmetric uncertainty around a realized value.
    SymmetricInterval {
        /// Distance from the center to either interval boundary.
        half_width: f64,
        /// Confidence or credible level, when supplied by the source.
        confidence_or_credible_level: Option<f64>,
    },
    /// Separate lower and upper uncertainty distances.
    AsymmetricInterval {
        /// Lower-side uncertainty distance.
        lower: f64,
        /// Upper-side uncertainty distance.
        upper: f64,
        /// Confidence or credible level, when supplied by the source.
        confidence_or_credible_level: Option<f64>,
    },
    /// One-sided or two-sided uncertainty bound.
    Bound {
        /// Optional lower bound.
        lower: Option<f64>,
        /// Optional upper bound.
        upper: Option<f64>,
        /// Confidence or credible level, when supplied by the source.
        confidence_or_credible_level: Option<f64>,
    },
    /// Named distribution with source-native parameters.
    ParametricDistribution {
        /// Distribution name, such as `Normal` or `LogNormal`.
        name: String,
        /// Distribution parameters keyed by their source-native names.
        parameters: BTreeMap<String, f64>,
    },
    /// External posterior or posterior summary artifact.
    PosteriorArtifact {
        /// Stable reference to the stored artifact.
        reference: String,
        /// Confidence or credible level, when supplied by the source.
        confidence_or_credible_level: Option<f64>,
    },
    /// External covariance or correlation artifact.
    CovarianceArtifact {
        /// Stable reference to the stored artifact.
        reference: String,
    },
    /// Explicitly records that no quantitative uncertainty is available.
    NotQuantified {
        /// Reason the uncertainty could not be quantified.
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
enum UncertaintyRepresentationWire {
    SymmetricInterval {
        half_width: f64,
        confidence_or_credible_level: Option<f64>,
    },
    AsymmetricInterval {
        lower: f64,
        upper: f64,
        confidence_or_credible_level: Option<f64>,
    },
    Bound {
        lower: Option<f64>,
        upper: Option<f64>,
        confidence_or_credible_level: Option<f64>,
    },
    ParametricDistribution {
        name: String,
        parameters: BTreeMap<String, f64>,
    },
    PosteriorArtifact {
        reference: String,
        confidence_or_credible_level: Option<f64>,
    },
    CovarianceArtifact {
        reference: String,
    },
    NotQuantified {
        reason: String,
    },
}

impl UncertaintyRepresentation {
    /// Creates a validated symmetric interval representation.
    pub fn symmetric_interval(
        half_width: f64,
        confidence_or_credible_level: Option<f64>,
    ) -> Result<Self, ProvenanceError> {
        let representation = Self::SymmetricInterval {
            half_width,
            confidence_or_credible_level,
        };
        representation.validate()?;
        Ok(representation)
    }

    /// Creates a validated asymmetric interval representation.
    pub fn asymmetric_interval(
        lower: f64,
        upper: f64,
        confidence_or_credible_level: Option<f64>,
    ) -> Result<Self, ProvenanceError> {
        let representation = Self::AsymmetricInterval {
            lower,
            upper,
            confidence_or_credible_level,
        };
        representation.validate()?;
        Ok(representation)
    }

    /// Creates a validated one-sided or two-sided bound.
    pub fn bound(
        lower: Option<f64>,
        upper: Option<f64>,
        confidence_or_credible_level: Option<f64>,
    ) -> Result<Self, ProvenanceError> {
        let representation = Self::Bound {
            lower,
            upper,
            confidence_or_credible_level,
        };
        representation.validate()?;
        Ok(representation)
    }

    /// Creates a validated named parametric distribution.
    pub fn parametric_distribution(
        name: impl Into<String>,
        parameters: BTreeMap<String, f64>,
    ) -> Result<Self, ProvenanceError> {
        let representation = Self::ParametricDistribution {
            name: name.into(),
            parameters,
        };
        representation.validate()?;
        Ok(representation)
    }

    /// Creates a validated reference to a posterior artifact.
    pub fn posterior_artifact(
        reference: impl Into<String>,
        confidence_or_credible_level: Option<f64>,
    ) -> Result<Self, ProvenanceError> {
        let representation = Self::PosteriorArtifact {
            reference: reference.into(),
            confidence_or_credible_level,
        };
        representation.validate()?;
        Ok(representation)
    }

    /// Creates a validated reference to a covariance artifact.
    pub fn covariance_artifact(reference: impl Into<String>) -> Result<Self, ProvenanceError> {
        let representation = Self::CovarianceArtifact {
            reference: reference.into(),
        };
        representation.validate()?;
        Ok(representation)
    }

    /// Creates an explicit, reason-bearing absence of quantitative uncertainty.
    pub fn not_quantified(reason: impl Into<String>) -> Result<Self, ProvenanceError> {
        let representation = Self::NotQuantified {
            reason: reason.into(),
        };
        representation.validate()?;
        Ok(representation)
    }

    /// Validates numeric ranges, probability levels, and artifact references.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        match self {
            Self::SymmetricInterval {
                half_width,
                confidence_or_credible_level,
            } => {
                validate_finite(*half_width, "uncertainty half width")?;
                if *half_width < 0.0 {
                    return Err(ProvenanceError::Negative {
                        field: "uncertainty half width",
                    });
                }
                validate_probability_level(
                    *confidence_or_credible_level,
                    "confidence or credible level",
                )?;
            }
            Self::AsymmetricInterval {
                lower,
                upper,
                confidence_or_credible_level,
            } => {
                validate_finite(*lower, "lower uncertainty")?;
                validate_finite(*upper, "upper uncertainty")?;
                if *lower < 0.0 || *upper < 0.0 {
                    return Err(ProvenanceError::Negative {
                        field: "asymmetric uncertainty",
                    });
                }
                validate_probability_level(
                    *confidence_or_credible_level,
                    "confidence or credible level",
                )?;
            }
            Self::Bound {
                lower,
                upper,
                confidence_or_credible_level,
            } => {
                if lower.is_none() && upper.is_none() {
                    return Err(ProvenanceError::InvalidInterval {
                        field: "uncertainty bound",
                    });
                }
                if let Some(lower) = lower {
                    validate_finite(*lower, "lower uncertainty bound")?;
                }
                if let Some(upper) = upper {
                    validate_finite(*upper, "upper uncertainty bound")?;
                }
                if let (Some(lower), Some(upper)) = (lower, upper)
                    && lower > upper
                {
                    return Err(ProvenanceError::InvalidInterval {
                        field: "uncertainty bound",
                    });
                }
                validate_probability_level(
                    *confidence_or_credible_level,
                    "confidence or credible level",
                )?;
            }
            Self::ParametricDistribution { name, parameters } => {
                validate_text(name, "uncertainty distribution name")?;
                validate_finite_map(parameters, "uncertainty distribution parameter")?;
            }
            Self::PosteriorArtifact {
                reference,
                confidence_or_credible_level,
            } => {
                validate_text(reference, "posterior artifact reference")?;
                validate_probability_level(
                    *confidence_or_credible_level,
                    "confidence or credible level",
                )?;
            }
            Self::CovarianceArtifact { reference } => {
                validate_text(reference, "covariance artifact reference")?;
            }
            Self::NotQuantified { reason } => {
                validate_text(reason, "unquantified uncertainty reason")?;
            }
        }
        Ok(())
    }

    /// Returns `true` only for [`Self::NotQuantified`].
    pub fn is_not_quantified(&self) -> bool {
        matches!(self, Self::NotQuantified { .. })
    }
}

impl TryFrom<UncertaintyRepresentationWire> for UncertaintyRepresentation {
    type Error = ProvenanceError;

    fn try_from(value: UncertaintyRepresentationWire) -> Result<Self, Self::Error> {
        let representation = match value {
            UncertaintyRepresentationWire::SymmetricInterval {
                half_width,
                confidence_or_credible_level,
            } => Self::SymmetricInterval {
                half_width,
                confidence_or_credible_level,
            },
            UncertaintyRepresentationWire::AsymmetricInterval {
                lower,
                upper,
                confidence_or_credible_level,
            } => Self::AsymmetricInterval {
                lower,
                upper,
                confidence_or_credible_level,
            },
            UncertaintyRepresentationWire::Bound {
                lower,
                upper,
                confidence_or_credible_level,
            } => Self::Bound {
                lower,
                upper,
                confidence_or_credible_level,
            },
            UncertaintyRepresentationWire::ParametricDistribution { name, parameters } => {
                Self::ParametricDistribution { name, parameters }
            }
            UncertaintyRepresentationWire::PosteriorArtifact {
                reference,
                confidence_or_credible_level,
            } => Self::PosteriorArtifact {
                reference,
                confidence_or_credible_level,
            },
            UncertaintyRepresentationWire::CovarianceArtifact { reference } => {
                Self::CovarianceArtifact { reference }
            }
            UncertaintyRepresentationWire::NotQuantified { reason } => {
                Self::NotQuantified { reason }
            }
        };
        representation.validate()?;
        Ok(representation)
    }
}

/// Aleatory variation sampled for the realised claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "AleatoryVariationWire")]
pub struct AleatoryVariation {
    /// Distribution or interval from which the realized variation was sampled.
    pub representation: UncertaintyRepresentation,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct AleatoryVariationWire {
    representation: UncertaintyRepresentation,
}

impl AleatoryVariation {
    /// Creates validated aleatory variation for a realized draw.
    pub fn new(representation: UncertaintyRepresentation) -> Result<Self, ProvenanceError> {
        representation.validate()?;
        Ok(Self { representation })
    }

    /// Validates the underlying uncertainty representation.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.representation.validate()
    }
}

impl TryFrom<AleatoryVariationWire> for AleatoryVariation {
    type Error = ProvenanceError;

    fn try_from(value: AleatoryVariationWire) -> Result<Self, Self::Error> {
        Self::new(value.representation)
    }
}

/// Epistemic uncertainty, optionally tied to one shared model realization and correlation group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "EpistemicUncertaintyWire")]
pub struct EpistemicUncertainty {
    /// Uncertainty about the model, parameters, or calibration.
    pub representation: UncertaintyRepresentation,
    /// Shared model realization, when this uncertainty participates in one.
    pub model_realization_id: Option<ModelRealizationId>,
    /// Shared correlation group, which requires `model_realization_id`.
    pub correlation_group_id: Option<CorrelationGroupId>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct EpistemicUncertaintyWire {
    representation: UncertaintyRepresentation,
    model_realization_id: Option<ModelRealizationId>,
    correlation_group_id: Option<CorrelationGroupId>,
}

impl EpistemicUncertainty {
    /// Creates epistemic uncertainty and checks correlation metadata.
    pub fn new(
        representation: UncertaintyRepresentation,
        model_realization_id: Option<ModelRealizationId>,
        correlation_group_id: Option<CorrelationGroupId>,
    ) -> Result<Self, ProvenanceError> {
        let uncertainty = Self {
            representation,
            model_realization_id,
            correlation_group_id,
        };
        uncertainty.validate()?;
        Ok(uncertainty)
    }

    /// Validates uncertainty and the local correlation-group invariant.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.representation.validate()?;
        if self.correlation_group_id.is_some() && self.model_realization_id.is_none() {
            return Err(ProvenanceError::CorrelationGroupWithoutModelRealization);
        }
        Ok(())
    }
}

impl TryFrom<EpistemicUncertaintyWire> for EpistemicUncertainty {
    type Error = ProvenanceError;

    fn try_from(value: EpistemicUncertaintyWire) -> Result<Self, Self::Error> {
        Self::new(
            value.representation,
            value.model_realization_id,
            value.correlation_group_id,
        )
    }
}

/// Aleatory and epistemic uncertainty are kept as separate fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ClaimUncertaintyWire")]
pub struct ClaimUncertainty {
    /// Variation sampled for this particular realization, if any.
    pub aleatory_variation: Option<AleatoryVariation>,
    /// Model or calibration uncertainty shared across realizations, if any.
    pub epistemic_uncertainty: Option<EpistemicUncertainty>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ClaimUncertaintyWire {
    aleatory_variation: Option<AleatoryVariation>,
    epistemic_uncertainty: Option<EpistemicUncertainty>,
}

impl ClaimUncertainty {
    /// Combines aleatory and epistemic uncertainty, requiring at least one.
    pub fn new(
        aleatory_variation: Option<AleatoryVariation>,
        epistemic_uncertainty: Option<EpistemicUncertainty>,
    ) -> Result<Self, ProvenanceError> {
        let uncertainty = Self {
            aleatory_variation,
            epistemic_uncertainty,
        };
        uncertainty.validate()?;
        Ok(uncertainty)
    }

    /// Creates an epistemic `NotQuantified` record with the supplied reason.
    pub fn not_quantified(reason: impl Into<String>) -> Result<Self, ProvenanceError> {
        Self::new(
            None,
            Some(EpistemicUncertainty::new(
                UncertaintyRepresentation::not_quantified(reason)?,
                None,
                None,
            )?),
        )
    }

    /// Validates both uncertainty channels and requires one to be present.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        if self.aleatory_variation.is_none() && self.epistemic_uncertainty.is_none() {
            return Err(ProvenanceError::MissingUncertainty);
        }
        if let Some(aleatory) = &self.aleatory_variation {
            aleatory.validate()?;
        }
        if let Some(epistemic) = &self.epistemic_uncertainty {
            epistemic.validate()?;
        }
        Ok(())
    }

    /// Returns `true` when either channel explicitly uses `NotQuantified`.
    pub fn is_unquantified(&self) -> bool {
        self.aleatory_variation
            .as_ref()
            .is_some_and(|uncertainty| uncertainty.representation.is_not_quantified())
            || self
                .epistemic_uncertainty
                .as_ref()
                .is_some_and(|uncertainty| uncertainty.representation.is_not_quantified())
    }
}

impl TryFrom<ClaimUncertaintyWire> for ClaimUncertainty {
    type Error = ProvenanceError;

    fn try_from(value: ClaimUncertaintyWire) -> Result<Self, Self::Error> {
        Self::new(value.aleatory_variation, value.epistemic_uncertainty)
    }
}
