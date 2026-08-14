//! Outcomes of claim-generation attempts.

use serde::{Deserialize, Serialize};

use super::claims::{ClaimProvenance, RandomDrawAddress, ScientificClaim};
use super::error::ProvenanceError;
use super::identifiers::validate_text;
use super::validation::ValidationReceipt;

/// A typed reason for coverage that the generator does not support.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "UnsupportedReasonWire")]
pub struct UnsupportedReason {
    /// Stable machine-readable reason code.
    pub code: String,
    /// Human-readable explanation of the unsupported coverage.
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct UnsupportedReasonWire {
    code: String,
    detail: String,
}

impl UnsupportedReason {
    /// Creates a typed, non-empty unsupported-coverage reason.
    pub fn new(
        code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let reason = Self {
            code: code.into(),
            detail: detail.into(),
        };
        reason.validate()?;
        Ok(reason)
    }

    /// Validates the reason code and detail.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_text(&self.code, "unsupported reason code")?;
        validate_text(&self.detail, "unsupported reason detail")?;
        Ok(())
    }
}

impl TryFrom<UnsupportedReasonWire> for UnsupportedReason {
    type Error = ProvenanceError;

    fn try_from(value: UnsupportedReasonWire) -> Result<Self, Self::Error> {
        Self::new(value.code, value.detail)
    }
}

/// The result of attempting to generate one claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    try_from = "ClaimOutcomeWire<T>",
    bound(deserialize = "T: Deserialize<'de>")
)]
pub enum ClaimOutcome<T> {
    /// A value was generated and passed every relevant validation constraint.
    Accepted(ScientificClaim<T>, ValidationReceipt),
    /// The stochastic process did not select a value; provenance and draw identity remain.
    NotSelected(ClaimProvenance, RandomDrawAddress),
    /// A value was generated but rejected; the receipt must retain a failed constraint.
    Rejected(ScientificClaim<T>, ValidationReceipt),
    /// The generator has no supported coverage for this requested claim.
    Unsupported(ClaimProvenance, Vec<UnsupportedReason>),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
enum ClaimOutcomeWire<T> {
    Accepted(ScientificClaim<T>, ValidationReceipt),
    NotSelected(ClaimProvenance, RandomDrawAddress),
    Rejected(ScientificClaim<T>, ValidationReceipt),
    Unsupported(ClaimProvenance, Vec<UnsupportedReason>),
}

impl<T> ClaimOutcome<T> {
    /// Validates the outcome-specific receipt, draw, and reason invariants.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        match self {
            Self::Accepted(claim, receipt) => {
                claim.validate()?;
                receipt.validate()?;
                claim.provenance.validate_realized_draw()?;
                if !receipt.is_successful() {
                    return Err(ProvenanceError::AcceptedWithoutSuccessfulValidation);
                }
            }
            Self::NotSelected(provenance, address) => {
                provenance.validate()?;
                address.validate()?;
                if provenance.uncertainty.aleatory_variation.is_none() {
                    return Err(ProvenanceError::NonSelectionWithoutAleatoryVariation);
                }
                if address.stable_object_id != provenance.object_id
                    || address.claim_key != provenance.claim_key
                {
                    return Err(ProvenanceError::RandomDrawAddressMismatch);
                }
                if let Some(provenance_address) = &provenance.random_draw_address
                    && provenance_address != address
                {
                    return Err(ProvenanceError::RandomDrawAddressMismatch);
                }
            }
            Self::Rejected(claim, receipt) => {
                claim.validate()?;
                receipt.validate()?;
                claim.provenance.validate_realized_draw()?;
                if !receipt.has_failure() {
                    return Err(ProvenanceError::RejectedWithoutFailure);
                }
            }
            Self::Unsupported(provenance, reasons) => {
                provenance.validate()?;
                provenance.validate_realized_draw()?;
                if reasons.is_empty() {
                    return Err(ProvenanceError::UnsupportedWithoutReason);
                }
                for reason in reasons {
                    reason.validate()?;
                }
            }
        }
        Ok(())
    }

    /// Returns provenance for every outcome variant, including non-values.
    pub fn provenance(&self) -> &ClaimProvenance {
        match self {
            Self::Accepted(claim, _) | Self::Rejected(claim, _) => &claim.provenance,
            Self::NotSelected(provenance, _) | Self::Unsupported(provenance, _) => provenance,
        }
    }

    /// Returns the draw address retained by a stochastic outcome, when present.
    pub(crate) fn random_draw_address(&self) -> Option<&RandomDrawAddress> {
        match self {
            Self::NotSelected(_, address) => Some(address),
            Self::Accepted(claim, _) | Self::Rejected(claim, _) => {
                claim.provenance.random_draw_address.as_ref()
            }
            Self::Unsupported(provenance, _) => provenance.random_draw_address.as_ref(),
        }
    }

    /// Returns the realized claim for accepted or rejected outcomes.
    pub fn claim(&self) -> Option<&ScientificClaim<T>> {
        match self {
            Self::Accepted(claim, _) | Self::Rejected(claim, _) => Some(claim),
            Self::NotSelected(_, _) | Self::Unsupported(_, _) => None,
        }
    }
}

impl<T> TryFrom<ClaimOutcomeWire<T>> for ClaimOutcome<T> {
    type Error = ProvenanceError;

    fn try_from(value: ClaimOutcomeWire<T>) -> Result<Self, Self::Error> {
        let outcome = match value {
            ClaimOutcomeWire::Accepted(claim, receipt) => Self::Accepted(claim, receipt),
            ClaimOutcomeWire::NotSelected(provenance, address) => {
                Self::NotSelected(provenance, address)
            }
            ClaimOutcomeWire::Rejected(claim, receipt) => Self::Rejected(claim, receipt),
            ClaimOutcomeWire::Unsupported(provenance, reasons) => {
                Self::Unsupported(provenance, reasons)
            }
        };
        outcome.validate()?;
        Ok(outcome)
    }
}
