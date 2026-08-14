//! Versioned plausibility validation receipts.

use serde::{Deserialize, Serialize};

use super::error::ProvenanceError;
use super::identifiers::{
    ClaimId, validate_finite, validate_optional_text, validate_text, validate_unique,
};

/// Result of evaluating one plausibility constraint.
/// Status recorded for one validation constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintStatus {
    /// The candidate satisfied this constraint.
    Passed,
    /// The candidate violated this constraint.
    Failed,
    /// The constraint was applicable but could not be evaluated.
    NotEvaluated,
}

/// Acceptance significance assigned to a validation constraint by the policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintClass {
    /// The constraint must pass before the candidate can be accepted.
    Required,
    /// The constraint may remain unevaluated when its limitation is recorded.
    Advisory,
    /// The requested evaluation lies outside the validator's model coverage.
    OutOfCoverage,
}

/// A receipt preserves every relevant constraint result, not only the first failure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ConstraintEvaluationWire")]
pub struct ConstraintEvaluation {
    /// Stable identity of the validation constraint.
    pub id: String,
    /// Acceptance significance assigned by the versioned policy.
    pub class: ConstraintClass,
    /// Result of evaluating the constraint.
    pub status: ConstraintStatus,
    /// Value observed by the constraint, when applicable.
    pub evaluated_value: Option<f64>,
    /// Threshold or limit used by the constraint, when applicable.
    pub threshold: Option<f64>,
    /// Signed distance from the threshold, when applicable.
    pub margin: Option<f64>,
    /// Additional diagnostic detail.
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ConstraintEvaluationWire {
    id: String,
    class: ConstraintClass,
    status: ConstraintStatus,
    evaluated_value: Option<f64>,
    threshold: Option<f64>,
    margin: Option<f64>,
    detail: Option<String>,
}

impl ConstraintEvaluation {
    /// Creates and validates a complete constraint result.
    pub fn new(
        id: impl Into<String>,
        class: ConstraintClass,
        status: ConstraintStatus,
        evaluated_value: Option<f64>,
        threshold: Option<f64>,
        margin: Option<f64>,
        detail: Option<String>,
    ) -> Result<Self, ProvenanceError> {
        let evaluation = Self {
            id: id.into(),
            class,
            status,
            evaluated_value,
            threshold,
            margin,
            detail,
        };
        evaluation.validate()?;
        Ok(evaluation)
    }

    /// Creates a passed constraint result.
    pub fn passed(
        id: impl Into<String>,
        evaluated_value: Option<f64>,
        threshold: Option<f64>,
        margin: Option<f64>,
        detail: Option<impl Into<String>>,
    ) -> Result<Self, ProvenanceError> {
        Self::new(
            id,
            ConstraintClass::Required,
            ConstraintStatus::Passed,
            evaluated_value,
            threshold,
            margin,
            detail.map(Into::into),
        )
    }

    /// Creates a failed constraint result.
    pub fn failed(
        id: impl Into<String>,
        evaluated_value: Option<f64>,
        threshold: Option<f64>,
        margin: Option<f64>,
        detail: Option<impl Into<String>>,
    ) -> Result<Self, ProvenanceError> {
        Self::new(
            id,
            ConstraintClass::Required,
            ConstraintStatus::Failed,
            evaluated_value,
            threshold,
            margin,
            detail.map(Into::into),
        )
    }

    /// Creates a not-evaluated result with a required explanation.
    pub fn not_evaluated(
        id: impl Into<String>,
        class: ConstraintClass,
        detail: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        Self::new(
            id,
            class,
            ConstraintStatus::NotEvaluated,
            None,
            None,
            None,
            Some(detail.into()),
        )
    }

    /// Validates identity, numeric fields, and the not-evaluated explanation rule.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_text(&self.id, "constraint identity")?;
        for value in [self.evaluated_value, self.threshold, self.margin]
            .into_iter()
            .flatten()
        {
            validate_finite(value, "constraint numeric result")?;
        }
        if self.status == ConstraintStatus::NotEvaluated
            && self
                .detail
                .as_deref()
                .is_none_or(|detail| detail.trim().is_empty())
        {
            return Err(ProvenanceError::InvalidConstraintEvaluation);
        }
        if self.class == ConstraintClass::OutOfCoverage
            && self.status != ConstraintStatus::NotEvaluated
        {
            return Err(ProvenanceError::InvalidConstraintEvaluation);
        }
        validate_optional_text(&self.detail, "constraint detail")?;
        Ok(())
    }
}

impl TryFrom<ConstraintEvaluationWire> for ConstraintEvaluation {
    type Error = ProvenanceError;

    fn try_from(value: ConstraintEvaluationWire) -> Result<Self, Self::Error> {
        Self::new(
            value.id,
            value.class,
            value.status,
            value.evaluated_value,
            value.threshold,
            value.margin,
            value.detail,
        )
    }
}

/// Versioned record of the validation policy and every constraint result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ValidationReceiptWire")]
pub struct ValidationReceipt {
    /// Stable identity of the policy that performed validation.
    pub policy_identity: String,
    /// Version of that validation policy.
    pub policy_version: String,
    /// Claims consumed by the validation policy.
    pub input_claims: Vec<ClaimId>,
    /// Complete results for every relevant constraint.
    pub constraints: Vec<ConstraintEvaluation>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ValidationReceiptWire {
    policy_identity: String,
    policy_version: String,
    input_claims: Vec<ClaimId>,
    constraints: Vec<ConstraintEvaluation>,
}

impl ValidationReceipt {
    /// Creates a receipt and requires at least one unique constraint.
    pub fn new(
        policy_identity: impl Into<String>,
        policy_version: impl Into<String>,
        input_claims: Vec<ClaimId>,
        constraints: Vec<ConstraintEvaluation>,
    ) -> Result<Self, ProvenanceError> {
        let receipt = Self {
            policy_identity: policy_identity.into(),
            policy_version: policy_version.into(),
            input_claims,
            constraints,
        };
        receipt.validate()?;
        Ok(receipt)
    }

    /// Validates policy metadata, input references, and constraint uniqueness.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_text(&self.policy_identity, "validation policy identity")?;
        validate_text(&self.policy_version, "validation policy version")?;
        for input_claim in &self.input_claims {
            input_claim.validate()?;
        }
        validate_unique(self.input_claims.iter().cloned(), "validation input claim")?;
        if self.constraints.is_empty() {
            return Err(ProvenanceError::EmptyValidationReceipt);
        }
        for constraint in &self.constraints {
            constraint.validate()?;
        }
        validate_unique(
            self.constraints
                .iter()
                .map(|constraint| constraint.id.clone()),
            "constraint",
        )?;
        Ok(())
    }

    /// Returns `true` when no constraint failed and every required check was evaluated.
    pub fn is_successful(&self) -> bool {
        self.constraints
            .iter()
            .all(|constraint| match constraint.status {
                ConstraintStatus::Passed => true,
                ConstraintStatus::Failed => false,
                ConstraintStatus::NotEvaluated => constraint.class == ConstraintClass::Advisory,
            })
    }

    /// Returns `true` when any constraint failed.
    pub fn has_failure(&self) -> bool {
        self.constraints
            .iter()
            .any(|constraint| constraint.status == ConstraintStatus::Failed)
    }
}

impl TryFrom<ValidationReceiptWire> for ValidationReceipt {
    type Error = ProvenanceError;

    fn try_from(value: ValidationReceiptWire) -> Result<Self, Self::Error> {
        Self::new(
            value.policy_identity,
            value.policy_version,
            value.input_claims,
            value.constraints,
        )
    }
}
