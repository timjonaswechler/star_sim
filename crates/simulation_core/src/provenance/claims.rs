//! Scientific claims, their provenance, derivations, and draw addresses.

use serde::{Deserialize, Serialize};

use super::applicability::ClaimApplicability;
use super::error::ProvenanceError;
use super::identifiers::{
    ClaimId, EvidenceLevel, ObjectId, PrescriptionId, validate_text, validate_unique,
};
use super::sources::ScientificSourceReference;
use super::uncertainty::ClaimUncertainty;

/// Immediate inputs of a derived claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ClaimDerivationWire")]
pub struct ClaimDerivation {
    /// Immediate input claims used to derive the current claim.
    pub input_claims: Vec<ClaimId>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ClaimDerivationWire {
    input_claims: Vec<ClaimId>,
}

impl ClaimDerivation {
    /// Creates a derivation with at least one unique input claim.
    pub fn new(input_claims: Vec<ClaimId>) -> Result<Self, ProvenanceError> {
        let derivation = Self { input_claims };
        derivation.validate()?;
        Ok(derivation)
    }

    /// Validates that all immediate inputs are present and unique.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        if self.input_claims.is_empty() {
            return Err(ProvenanceError::EmptyField {
                field: "claim derivation inputs",
            });
        }
        for input_claim in &self.input_claims {
            input_claim.validate()?;
        }
        validate_unique(self.input_claims.iter().cloned(), "claim derivation input")
    }
}

impl TryFrom<ClaimDerivationWire> for ClaimDerivation {
    type Error = ProvenanceError;

    fn try_from(value: ClaimDerivationWire) -> Result<Self, Self::Error> {
        Self::new(value.input_claims)
    }
}

/// Stable address of one stochastic draw, independent of unrelated draw order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "RandomDrawAddressWire")]
pub struct RandomDrawAddress {
    /// Name of the random algorithm used for the draw.
    pub algorithm: String,
    /// Version of the random algorithm or implementation.
    pub algorithm_version: String,
    /// Prescription namespace owning the draw stream.
    pub prescription_namespace: String,
    /// Stable object identity used instead of collection position.
    pub stable_object_id: ObjectId,
    /// Claim key whose draw is being addressed.
    pub claim_key: String,
    /// Deterministic retry/attempt number within the bounded stream.
    pub bounded_attempt_index: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct RandomDrawAddressWire {
    algorithm: String,
    algorithm_version: String,
    prescription_namespace: String,
    stable_object_id: ObjectId,
    claim_key: String,
    bounded_attempt_index: u32,
}

impl RandomDrawAddress {
    /// Creates a stable draw address independent of unrelated draw order.
    pub fn new(
        algorithm: impl Into<String>,
        algorithm_version: impl Into<String>,
        prescription_namespace: impl Into<String>,
        stable_object_id: impl Into<ObjectId>,
        claim_key: impl Into<String>,
        bounded_attempt_index: u32,
    ) -> Result<Self, ProvenanceError> {
        let address = Self {
            algorithm: algorithm.into(),
            algorithm_version: algorithm_version.into(),
            prescription_namespace: prescription_namespace.into(),
            stable_object_id: stable_object_id.into(),
            claim_key: claim_key.into(),
            bounded_attempt_index,
        };
        address.validate()?;
        Ok(address)
    }

    /// Validates all identity components used to reproduce or audit the draw.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.stable_object_id.validate()?;
        validate_text(&self.algorithm, "random algorithm")?;
        validate_text(&self.algorithm_version, "random algorithm version")?;
        validate_text(
            &self.prescription_namespace,
            "random prescription namespace",
        )?;
        validate_text(&self.claim_key, "random claim key")?;
        Ok(())
    }
}

impl TryFrom<RandomDrawAddressWire> for RandomDrawAddress {
    type Error = ProvenanceError;

    fn try_from(value: RandomDrawAddressWire) -> Result<Self, Self::Error> {
        Self::new(
            value.algorithm,
            value.algorithm_version,
            value.prescription_namespace,
            value.stable_object_id,
            value.claim_key,
            value.bounded_attempt_index,
        )
    }
}

/// Provenance attached to one scientifically meaningful claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ClaimProvenanceWire")]
pub struct ClaimProvenance {
    /// Stable identity of the object to which this claim belongs.
    pub object_id: ObjectId,
    /// Stable property/decision key within the object.
    pub claim_key: String,
    /// Evidence strength declared for this claim.
    pub evidence_level: EvidenceLevel,
    /// Versioned method that generated the claim.
    pub generating_prescription: PrescriptionId,
    /// Sources directly cited by this claim.
    pub source_references: Vec<ScientificSourceReference>,
    /// Calibration-domain status of the claim's inputs.
    pub applicability: ClaimApplicability,
    /// Separate aleatory and epistemic uncertainty metadata.
    pub uncertainty: ClaimUncertainty,
    /// Immediate inputs when this claim is derived from other claims.
    pub derivation: Option<ClaimDerivation>,
    /// Stable address of the stochastic draw, when a draw realized this claim.
    pub random_draw_address: Option<RandomDrawAddress>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ClaimProvenanceWire {
    object_id: ObjectId,
    claim_key: String,
    evidence_level: EvidenceLevel,
    generating_prescription: PrescriptionId,
    source_references: Vec<ScientificSourceReference>,
    applicability: ClaimApplicability,
    uncertainty: ClaimUncertainty,
    derivation: Option<ClaimDerivation>,
    random_draw_address: Option<RandomDrawAddress>,
}

impl ClaimProvenance {
    /// Creates provenance and checks local evidence, applicability, and uncertainty rules.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        object_id: impl Into<ObjectId>,
        claim_key: impl Into<String>,
        evidence_level: EvidenceLevel,
        generating_prescription: impl Into<PrescriptionId>,
        source_references: Vec<ScientificSourceReference>,
        applicability: ClaimApplicability,
        uncertainty: ClaimUncertainty,
        derivation: Option<ClaimDerivation>,
        random_draw_address: Option<RandomDrawAddress>,
    ) -> Result<Self, ProvenanceError> {
        let provenance = Self {
            object_id: object_id.into(),
            claim_key: claim_key.into(),
            evidence_level,
            generating_prescription: generating_prescription.into(),
            source_references,
            applicability,
            uncertainty,
            derivation,
            random_draw_address,
        };
        provenance.validate()?;
        Ok(provenance)
    }

    /// Validates local invariants; catalog and derivation checks happen in a document.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_text(&self.claim_key, "claim key")?;
        self.object_id.validate()?;
        self.generating_prescription.validate()?;
        for reference in &self.source_references {
            reference.validate()?;
        }
        self.applicability.validate()?;
        self.uncertainty.validate()?;
        if let Some(derivation) = &self.derivation {
            derivation.validate()?;
        }
        if let Some(address) = &self.random_draw_address {
            address.validate()?;
            if address.stable_object_id != self.object_id || address.claim_key != self.claim_key {
                return Err(ProvenanceError::RandomDrawAddressMismatch);
            }
        }
        if self.evidence_level == EvidenceLevel::Empirical {
            if self.source_references.is_empty() {
                return Err(ProvenanceError::EmpiricalClaimWithoutSource);
            }
            if !self.applicability.is_inside_domain() {
                return Err(ProvenanceError::EmpiricalClaimOutsideDomain);
            }
        }
        if self.applicability.is_extrapolated()
            && self.evidence_level != EvidenceLevel::PhysicalProxy
        {
            return Err(ProvenanceError::ExtrapolationRequiresPhysicalProxy);
        }
        if self.evidence_level == EvidenceLevel::PhysicalProxy
            && matches!(self.applicability, ClaimApplicability::PresentationOnly)
        {
            return Err(ProvenanceError::PhysicalProxyWithoutApplicability);
        }
        if self.evidence_level == EvidenceLevel::Decorative
            && !matches!(self.applicability, ClaimApplicability::PresentationOnly)
        {
            return Err(ProvenanceError::DecorativeWithoutPresentationApplicability);
        }
        Ok(())
    }

    /// Validates the additional draw invariant required by realized outcomes.
    pub(crate) fn validate_realized_draw(&self) -> Result<(), ProvenanceError> {
        if self.uncertainty.aleatory_variation.is_some() && self.random_draw_address.is_none() {
            return Err(ProvenanceError::MissingRandomDrawAddress);
        }
        Ok(())
    }
}

impl TryFrom<ClaimProvenanceWire> for ClaimProvenance {
    type Error = ProvenanceError;

    fn try_from(value: ClaimProvenanceWire) -> Result<Self, Self::Error> {
        Self::new(
            value.object_id,
            value.claim_key,
            value.evidence_level,
            value.generating_prescription,
            value.source_references,
            value.applicability,
            value.uncertainty,
            value.derivation,
            value.random_draw_address,
        )
    }
}

/// A realised value and the provenance that explains how it was obtained.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    try_from = "ScientificClaimWire<T>",
    bound(deserialize = "T: Deserialize<'de>")
)]
pub struct ScientificClaim<T> {
    /// Stable identity of this claim within its provenance document.
    pub id: ClaimId,
    /// Domain value being asserted.
    pub value: T,
    /// Evidence and generation metadata for `value`.
    pub provenance: ClaimProvenance,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ScientificClaimWire<T> {
    id: ClaimId,
    value: T,
    provenance: ClaimProvenance,
}

impl<T> ScientificClaim<T> {
    /// Combines a value with validated provenance.
    pub fn new(
        id: impl Into<ClaimId>,
        value: T,
        provenance: ClaimProvenance,
    ) -> Result<Self, ProvenanceError> {
        let claim = Self {
            id: id.into(),
            value,
            provenance,
        };
        claim.validate()?;
        Ok(claim)
    }

    /// Validates the claim identity, provenance, and realized-draw requirement.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.id.validate()?;
        self.provenance.validate()?;
        self.provenance.validate_realized_draw()
    }
}

impl<T> TryFrom<ScientificClaimWire<T>> for ScientificClaim<T> {
    type Error = ProvenanceError;

    fn try_from(value: ScientificClaimWire<T>) -> Result<Self, Self::Error> {
        Self::new(value.id, value.value, value.provenance)
    }
}
