//! Calibration domains and explicit extrapolation records.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::error::ProvenanceError;
use super::identifiers::{validate_finite, validate_finite_map, validate_text, validate_unique};

/// An axis that is outside the calibrated domain of a source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ExtrapolatedInputAxisWire")]
pub struct ExtrapolatedInputAxis {
    /// Name of the input axis, for example `metallicity` or `mass`.
    pub axis: String,
    /// Lower calibrated boundary for this axis.
    pub calibrated_minimum: f64,
    /// Upper calibrated boundary for this axis.
    pub calibrated_maximum: f64,
    /// Value actually supplied to the model.
    pub evaluated_value: f64,
    /// Boundary crossed by the evaluated value.
    pub direction: ExtrapolationDirection,
    /// The reported departure from the calibrated boundary, in the axis' units.
    pub departure: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ExtrapolatedInputAxisWire {
    axis: String,
    calibrated_minimum: f64,
    calibrated_maximum: f64,
    evaluated_value: f64,
    direction: ExtrapolationDirection,
    departure: f64,
}

/// Identifies which side of a calibrated interval was exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtrapolationDirection {
    /// The evaluated value is below the calibrated minimum.
    BelowMinimum,
    /// The evaluated value is above the calibrated maximum.
    AboveMaximum,
}

impl ExtrapolatedInputAxis {
    /// Creates an axis record and verifies that the value lies beyond the named boundary.
    pub fn new(
        axis: impl Into<String>,
        calibrated_minimum: f64,
        calibrated_maximum: f64,
        evaluated_value: f64,
        direction: ExtrapolationDirection,
        departure: f64,
    ) -> Result<Self, ProvenanceError> {
        let axis = Self {
            axis: axis.into(),
            calibrated_minimum,
            calibrated_maximum,
            evaluated_value,
            direction,
            departure,
        };
        axis.validate()?;
        Ok(axis)
    }

    /// Validates the interval, direction, finiteness, and positive departure.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_text(&self.axis, "extrapolated input axis")?;
        validate_finite(self.calibrated_minimum, "calibrated minimum")?;
        validate_finite(self.calibrated_maximum, "calibrated maximum")?;
        validate_finite(self.evaluated_value, "evaluated input")?;
        validate_finite(self.departure, "extrapolation departure")?;
        if self.calibrated_minimum >= self.calibrated_maximum {
            return Err(ProvenanceError::InvalidInterval {
                field: "calibrated input domain",
            });
        }
        if self.departure <= 0.0 {
            return Err(ProvenanceError::NotPositive {
                field: "extrapolation departure",
            });
        }
        let expected_departure = match self.direction {
            ExtrapolationDirection::BelowMinimum
                if self.evaluated_value < self.calibrated_minimum =>
            {
                self.calibrated_minimum - self.evaluated_value
            }
            ExtrapolationDirection::AboveMaximum
                if self.evaluated_value > self.calibrated_maximum =>
            {
                self.evaluated_value - self.calibrated_maximum
            }
            _ => {
                return Err(ProvenanceError::InvalidInterval {
                    field: "extrapolated input axis",
                });
            }
        };
        if !expected_departure.is_finite() || self.departure != expected_departure {
            return Err(ProvenanceError::InvalidInterval {
                field: "extrapolation departure",
            });
        }
        Ok(())
    }
}

impl TryFrom<ExtrapolatedInputAxisWire> for ExtrapolatedInputAxis {
    type Error = ProvenanceError;

    fn try_from(value: ExtrapolatedInputAxisWire) -> Result<Self, Self::Error> {
        Self::new(
            value.axis,
            value.calibrated_minimum,
            value.calibrated_maximum,
            value.evaluated_value,
            value.direction,
            value.departure,
        )
    }
}

/// Applicability of a claim relative to the calibration domain of its source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ClaimApplicabilityWire")]
pub enum ClaimApplicability {
    /// The evaluated inputs are within a named calibration domain.
    InsideDomain {
        /// Name or versioned identity of the calibration domain.
        calibrated_domain: String,
        /// Input values used for this evaluation.
        evaluated_inputs: BTreeMap<String, f64>,
    },
    /// At least one input lies outside the domain and no value was generated.
    OutsideDomain {
        /// Name or versioned identity of the unavailable model domain.
        calibrated_domain: String,
        /// Input values used for the unsupported evaluation.
        evaluated_inputs: BTreeMap<String, f64>,
    },
    /// At least one input lies outside the calibration domain and a proxy value was generated.
    Extrapolated(ClaimExtrapolation),
    /// Presentation-only variation has no scientific calibration domain.
    PresentationOnly,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
enum ClaimApplicabilityWire {
    InsideDomain {
        calibrated_domain: String,
        evaluated_inputs: BTreeMap<String, f64>,
    },
    OutsideDomain {
        calibrated_domain: String,
        evaluated_inputs: BTreeMap<String, f64>,
    },
    Extrapolated(ClaimExtrapolation),
    PresentationOnly,
}

impl ClaimApplicability {
    /// Creates an inside-domain applicability record.
    pub fn inside_domain(
        calibrated_domain: impl Into<String>,
        evaluated_inputs: BTreeMap<String, f64>,
    ) -> Result<Self, ProvenanceError> {
        let applicability = Self::InsideDomain {
            calibrated_domain: calibrated_domain.into(),
            evaluated_inputs,
        };
        applicability.validate()?;
        Ok(applicability)
    }

    /// Creates an outside-domain record for an unsupported evaluation.
    pub fn outside_domain(
        calibrated_domain: impl Into<String>,
        evaluated_inputs: BTreeMap<String, f64>,
    ) -> Result<Self, ProvenanceError> {
        let applicability = Self::OutsideDomain {
            calibrated_domain: calibrated_domain.into(),
            evaluated_inputs,
        };
        applicability.validate()?;
        Ok(applicability)
    }

    /// Creates an applicability record for explicit extrapolation.
    pub fn extrapolated(extrapolation: ClaimExtrapolation) -> Result<Self, ProvenanceError> {
        let applicability = Self::Extrapolated(extrapolation);
        applicability.validate()?;
        Ok(applicability)
    }

    /// Validates the selected applicability variant and all numeric inputs.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        match self {
            Self::InsideDomain {
                calibrated_domain,
                evaluated_inputs,
            } => {
                validate_text(calibrated_domain, "calibrated domain")?;
                validate_finite_map(evaluated_inputs, "evaluated input")?;
            }
            Self::OutsideDomain {
                calibrated_domain,
                evaluated_inputs,
            } => {
                validate_text(calibrated_domain, "calibrated domain")?;
                validate_finite_map(evaluated_inputs, "evaluated input")?;
            }
            Self::Extrapolated(extrapolation) => extrapolation.validate()?,
            Self::PresentationOnly => {}
        }
        Ok(())
    }

    /// Returns `true` when the claim is explicitly inside a calibration domain.
    pub fn is_inside_domain(&self) -> bool {
        matches!(self, Self::InsideDomain { .. })
    }

    /// Returns `true` when the claim records explicit extrapolation.
    pub fn is_extrapolated(&self) -> bool {
        matches!(self, Self::Extrapolated(_))
    }
}

impl TryFrom<ClaimApplicabilityWire> for ClaimApplicability {
    type Error = ProvenanceError;

    fn try_from(value: ClaimApplicabilityWire) -> Result<Self, Self::Error> {
        let applicability = match value {
            ClaimApplicabilityWire::InsideDomain {
                calibrated_domain,
                evaluated_inputs,
            } => Self::InsideDomain {
                calibrated_domain,
                evaluated_inputs,
            },
            ClaimApplicabilityWire::OutsideDomain {
                calibrated_domain,
                evaluated_inputs,
            } => Self::OutsideDomain {
                calibrated_domain,
                evaluated_inputs,
            },
            ClaimApplicabilityWire::Extrapolated(extrapolation) => {
                Self::Extrapolated(extrapolation)
            }
            ClaimApplicabilityWire::PresentationOnly => Self::PresentationOnly,
        };
        applicability.validate()?;
        Ok(applicability)
    }
}

/// Structured record of a claim evaluated beyond a calibrated domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ClaimExtrapolationWire")]
pub struct ClaimExtrapolation {
    /// Name or versioned identity of the calibration domain exceeded.
    pub calibrated_domain: String,
    /// Every input axis that left the calibrated domain.
    pub exceeded_axes: Vec<ExtrapolatedInputAxis>,
    /// Method or policy used to extend the model beyond calibration.
    pub method: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ClaimExtrapolationWire {
    calibrated_domain: String,
    exceeded_axes: Vec<ExtrapolatedInputAxis>,
    method: String,
}

impl ClaimExtrapolation {
    /// Creates and validates an explicit extrapolation record.
    pub fn new(
        calibrated_domain: impl Into<String>,
        exceeded_axes: Vec<ExtrapolatedInputAxis>,
        method: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let extrapolation = Self {
            calibrated_domain: calibrated_domain.into(),
            exceeded_axes,
            method: method.into(),
        };
        extrapolation.validate()?;
        Ok(extrapolation)
    }

    /// Validates the method and requires unique, non-empty exceeded axes.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_text(&self.calibrated_domain, "extrapolation calibrated domain")?;
        validate_text(&self.method, "extrapolation method")?;
        if self.exceeded_axes.is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "exceeded extrapolation axes",
            });
        }
        for axis in &self.exceeded_axes {
            axis.validate()?;
        }
        validate_unique(
            self.exceeded_axes.iter().map(|axis| axis.axis.clone()),
            "extrapolation axis",
        )?;
        Ok(())
    }
}

impl TryFrom<ClaimExtrapolationWire> for ClaimExtrapolation {
    type Error = ProvenanceError;

    fn try_from(value: ClaimExtrapolationWire) -> Result<Self, Self::Error> {
        Self::new(value.calibrated_domain, value.exceeded_axes, value.method)
    }
}
