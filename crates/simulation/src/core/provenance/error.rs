//! Errors shared by the provenance modules.

use thiserror::Error;

/// Errors raised when provenance would otherwise become incomplete or inconsistent.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProvenanceError {
    #[error("{kind} identifier must not be empty")]
    EmptyIdentifier { kind: &'static str },
    #[error("field `{field}` must not be empty")]
    EmptyField { field: &'static str },
    #[error("field `{field}` must be finite")]
    NonFinite { field: &'static str },
    #[error("field `{field}` must be greater than zero")]
    NotPositive { field: &'static str },
    #[error("field `{field}` must be non-negative")]
    Negative { field: &'static str },
    #[error("field `{field}` has an invalid interval")]
    InvalidInterval { field: &'static str },
    #[error("field `{field}` must be between zero and one")]
    InvalidProbabilityLevel { field: &'static str },
    #[error("duplicate {kind} identifier `{id}`")]
    DuplicateIdentifier { kind: &'static str, id: String },
    #[error("dangling {kind} reference `{id}`")]
    DanglingReference { kind: &'static str, id: String },
    #[error("source `{source_id}` is not declared by prescription `{prescription}`")]
    SourceNotDeclaredByPrescription {
        source_id: String,
        prescription: String,
    },
    #[error("empirical claim must cite at least one scientific source")]
    EmpiricalClaimWithoutSource,
    #[error("empirical claim must have inside-domain applicability")]
    EmpiricalClaimOutsideDomain,
    #[error("physical-proxy claim must record inside-domain or extrapolated applicability")]
    PhysicalProxyWithoutApplicability,
    #[error("only physical-proxy claims may carry extrapolation")]
    ExtrapolationRequiresPhysicalProxy,
    #[error("decorative claims must have presentation-only applicability")]
    DecorativeWithoutPresentationApplicability,
    #[error(
        "claim uncertainty must record aleatory or epistemic uncertainty; use NotQuantified when necessary"
    )]
    MissingUncertainty,
    #[error("correlation group requires a model realization")]
    CorrelationGroupWithoutModelRealization,
    #[error(
        "epistemic uncertainty references a different model realization than its correlation group"
    )]
    CorrelationGroupModelRealizationMismatch,
    #[error("stochastic claim requires a random draw address")]
    MissingRandomDrawAddress,
    #[error("random draw address does not match the claim provenance")]
    RandomDrawAddressMismatch,
    #[error("not-selected outcome requires aleatory variation")]
    NonSelectionWithoutAleatoryVariation,
    #[error("unsupported outcome requires at least one reason")]
    UnsupportedWithoutReason,
    #[error("rejected outcome must retain at least one failed validation constraint")]
    RejectedWithoutFailure,
    #[error("accepted outcome requires a successful validation receipt")]
    AcceptedWithoutSuccessfulValidation,
    #[error("validation receipt must contain at least one constraint evaluation")]
    EmptyValidationReceipt,
    #[error("validation receipt contains duplicate constraint `{id}`")]
    DuplicateConstraint { id: String },
    #[error("validation receipt contains an invalid constraint evaluation")]
    InvalidConstraintEvaluation,
    #[error(
        "claim `{claim}` declares evidence `{declared}` but its inputs and prescription support only `{effective}`"
    )]
    EvidenceLevelMismatch {
        claim: String,
        declared: String,
        effective: String,
    },
    #[error("claim derivation contains a cycle involving `{claim}`")]
    ClaimDerivationCycle { claim: String },
    #[error("claim `{claim}` is already represented by another outcome")]
    DuplicateClaimOutcome { claim: String },
    #[error("object evidence summary for `{object}` is inconsistent with its claims")]
    InvalidObjectEvidenceSummary { object: String },
    #[error("claim `{claim}` contains an invalid value: {detail}")]
    InvalidClaimValue { claim: String, detail: String },
}
