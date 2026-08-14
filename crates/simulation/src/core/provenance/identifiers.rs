//! Stable identifiers, evidence levels, and local validation helpers.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use serde::{Deserialize, Serialize};

use super::error::ProvenanceError;

macro_rules! identifier {
    ($name:ident, $kind:literal) => {
        #[doc = concat!("Stable ", $kind, " identifier used in provenance records.")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a validated ", $kind, " identifier." )]
            pub fn new(value: impl Into<String>) -> Result<Self, ProvenanceError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(ProvenanceError::EmptyIdentifier { kind: $kind });
                }
                Ok(Self(value))
            }

            /// Returns the identifier's stable textual representation.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Re-checks the invariant that the identifier is non-empty.
            pub fn validate(&self) -> Result<(), ProvenanceError> {
                if self.0.trim().is_empty() {
                    return Err(ProvenanceError::EmptyIdentifier { kind: $kind });
                }
                Ok(())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

identifier!(ClaimId, "claim");
identifier!(ObjectId, "object");
identifier!(PrescriptionId, "prescription");
identifier!(SourceId, "source");
identifier!(ModelRealizationId, "model-realization");
identifier!(CorrelationGroupId, "correlation-group");

/// Evidence support is ordered from strongest to least supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EvidenceLevel {
    /// Directly supported by empirical evidence inside the source's domain.
    Empirical,
    /// Supported by a calibrated physical proxy, possibly with explicit extrapolation.
    PhysicalProxy,
    /// Used for presentation or flavor and not as physical evidence.
    Decorative,
}

impl fmt::Display for EvidenceLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empirical => "Empirical",
            Self::PhysicalProxy => "PhysicalProxy",
            Self::Decorative => "Decorative",
        })
    }
}

impl EvidenceLevel {
    /// Returns `true` for evidence that can support a physical claim.
    pub fn is_physical(self) -> bool {
        matches!(self, Self::Empirical | Self::PhysicalProxy)
    }
}

pub(crate) fn validate_text(value: &str, field: &'static str) -> Result<(), ProvenanceError> {
    if value.trim().is_empty() {
        Err(ProvenanceError::EmptyField { field })
    } else {
        Ok(())
    }
}

pub(crate) fn validate_optional_text(
    value: &Option<String>,
    field: &'static str,
) -> Result<(), ProvenanceError> {
    if let Some(value) = value {
        validate_text(value, field)?;
    }
    Ok(())
}

pub(crate) fn validate_finite(value: f64, field: &'static str) -> Result<(), ProvenanceError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ProvenanceError::NonFinite { field })
    }
}

pub(crate) fn validate_finite_map(
    values: &BTreeMap<String, f64>,
    field: &'static str,
) -> Result<(), ProvenanceError> {
    for (key, value) in values {
        validate_text(key, field)?;
        validate_finite(*value, field)?;
    }
    Ok(())
}

pub(crate) fn validate_probability_level(
    value: Option<f64>,
    field: &'static str,
) -> Result<(), ProvenanceError> {
    if let Some(value) = value {
        validate_finite(value, field)?;
        if !(0.0 < value && value <= 1.0) {
            return Err(ProvenanceError::InvalidProbabilityLevel { field });
        }
    }
    Ok(())
}

pub(crate) fn validate_unique<T: Ord + fmt::Display>(
    values: impl IntoIterator<Item = T>,
    kind: &'static str,
) -> Result<(), ProvenanceError> {
    let mut seen = BTreeSet::new();
    for value in values {
        let display = value.to_string();
        if !seen.insert(value) {
            return Err(ProvenanceError::DuplicateIdentifier { kind, id: display });
        }
    }
    Ok(())
}
