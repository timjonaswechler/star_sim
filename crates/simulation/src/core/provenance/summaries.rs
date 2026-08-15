//! Derived object-level evidence summaries.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::error::ProvenanceError;
use super::identifiers::{EvidenceLevel, ObjectId};
use super::outcomes::ClaimOutcome;

/// Derived overview of the claims attached to one object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ObjectEvidenceSummaryWire")]
pub struct ObjectEvidenceSummary {
    /// Object whose outcomes are summarized.
    pub object_id: ObjectId,
    /// Number of outcomes at each evidence level.
    pub claim_counts_by_evidence_level: BTreeMap<EvidenceLevel, usize>,
    /// Total number of outcomes belonging to the object.
    pub total_claim_count: usize,
    /// Weakest physical evidence level actually represented by the object.
    pub least_supported_physical_level: Option<EvidenceLevel>,
    /// Whether at least one decorative outcome is present.
    pub has_decorative_claims: bool,
    /// Number of outcomes with explicit extrapolation.
    pub extrapolated_claim_count: usize,
    /// Derived flag for `extrapolated_claim_count > 0`.
    pub has_extrapolated_claims: bool,
    /// Number of outcomes with explicitly unquantified uncertainty.
    pub unquantified_uncertainty_claim_count: usize,
    /// Derived flag for `unquantified_uncertainty_claim_count > 0`.
    pub has_unquantified_uncertainty: bool,
    /// Number of generated candidates that were rejected.
    pub rejected_outcome_count: usize,
    /// Number of unsupported-generation outcomes.
    pub unsupported_outcome_count: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ObjectEvidenceSummaryWire {
    object_id: ObjectId,
    claim_counts_by_evidence_level: BTreeMap<EvidenceLevel, usize>,
    total_claim_count: usize,
    least_supported_physical_level: Option<EvidenceLevel>,
    has_decorative_claims: bool,
    extrapolated_claim_count: usize,
    has_extrapolated_claims: bool,
    unquantified_uncertainty_claim_count: usize,
    has_unquantified_uncertainty: bool,
    rejected_outcome_count: usize,
    unsupported_outcome_count: usize,
}

impl ObjectEvidenceSummary {
    /// Derives a summary for one object from validated outcomes.
    pub fn from_outcomes<T>(
        object_id: impl Into<ObjectId>,
        outcomes: &[ClaimOutcome<T>],
    ) -> Result<Self, ProvenanceError> {
        let object_id = object_id.into();
        for outcome in outcomes {
            outcome.validate()?;
        }
        Self::from_matching_outcomes(
            object_id.clone(),
            outcomes
                .iter()
                .filter(|outcome| outcome.provenance().object_id == object_id),
        )
    }

    /// Derives every object summary in one linear grouping pass.
    pub(crate) fn from_all_outcomes<T>(
        outcomes: &[ClaimOutcome<T>],
    ) -> Result<Vec<Self>, ProvenanceError> {
        let mut grouped = BTreeMap::<ObjectId, Vec<&ClaimOutcome<T>>>::new();
        for outcome in outcomes {
            grouped
                .entry(outcome.provenance().object_id.clone())
                .or_default()
                .push(outcome);
        }
        grouped
            .into_iter()
            .map(|(object_id, outcomes)| {
                Self::from_matching_outcomes(object_id, outcomes.into_iter())
            })
            .collect()
    }

    fn from_matching_outcomes<'a, T: 'a>(
        object_id: ObjectId,
        outcomes: impl Iterator<Item = &'a ClaimOutcome<T>>,
    ) -> Result<Self, ProvenanceError> {
        let mut counts = BTreeMap::from([
            (EvidenceLevel::Empirical, 0),
            (EvidenceLevel::PhysicalProxy, 0),
            (EvidenceLevel::Decorative, 0),
        ]);
        let mut extrapolated_claim_count = 0;
        let mut unquantified_uncertainty_claim_count = 0;
        let mut rejected_outcome_count = 0;
        let mut unsupported_outcome_count = 0;

        for outcome in outcomes {
            outcome.validate()?;
            let provenance = outcome.provenance();
            *counts.entry(provenance.evidence_level).or_default() += 1;
            if provenance.applicability.is_extrapolated() {
                extrapolated_claim_count += 1;
            }
            if provenance.uncertainty.is_unquantified() {
                unquantified_uncertainty_claim_count += 1;
            }
            if matches!(outcome, ClaimOutcome::Rejected(_, _)) {
                rejected_outcome_count += 1;
            }
            if matches!(outcome, ClaimOutcome::Unsupported(_, _)) {
                unsupported_outcome_count += 1;
            }
        }

        let least_supported_physical_level = counts
            .iter()
            .filter(|(level, count)| level.is_physical() && **count > 0)
            .map(|(level, _)| *level)
            .max();
        let has_decorative_claims = counts
            .get(&EvidenceLevel::Decorative)
            .copied()
            .unwrap_or_default()
            > 0;
        let total_claim_count = counts.values().sum();
        let summary = Self {
            object_id,
            claim_counts_by_evidence_level: counts,
            total_claim_count,
            least_supported_physical_level,
            has_decorative_claims,
            extrapolated_claim_count,
            has_extrapolated_claims: extrapolated_claim_count > 0,
            unquantified_uncertainty_claim_count,
            has_unquantified_uncertainty: unquantified_uncertainty_claim_count > 0,
            rejected_outcome_count,
            unsupported_outcome_count,
        };
        summary.validate()?;
        Ok(summary)
    }

    /// Validates all derived counts and flags against the summary contents.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.object_id.validate()?;
        let expected_counts = BTreeMap::from([
            (
                EvidenceLevel::Empirical,
                self.claim_counts_by_evidence_level
                    .get(&EvidenceLevel::Empirical)
                    .copied(),
            ),
            (
                EvidenceLevel::PhysicalProxy,
                self.claim_counts_by_evidence_level
                    .get(&EvidenceLevel::PhysicalProxy)
                    .copied(),
            ),
            (
                EvidenceLevel::Decorative,
                self.claim_counts_by_evidence_level
                    .get(&EvidenceLevel::Decorative)
                    .copied(),
            ),
        ]);
        if self.claim_counts_by_evidence_level.len() != 3
            || expected_counts.values().any(Option::is_none)
        {
            return Err(ProvenanceError::InvalidObjectEvidenceSummary {
                object: self.object_id.to_string(),
            });
        }
        let expected_total: usize = self.claim_counts_by_evidence_level.values().sum();
        if expected_total != self.total_claim_count
            || self.extrapolated_claim_count > self.total_claim_count
            || self.unquantified_uncertainty_claim_count > self.total_claim_count
            || self
                .rejected_outcome_count
                .saturating_add(self.unsupported_outcome_count)
                > self.total_claim_count
        {
            return Err(ProvenanceError::InvalidObjectEvidenceSummary {
                object: self.object_id.to_string(),
            });
        }
        if self.has_extrapolated_claims != (self.extrapolated_claim_count > 0)
            || self.has_unquantified_uncertainty != (self.unquantified_uncertainty_claim_count > 0)
        {
            return Err(ProvenanceError::InvalidObjectEvidenceSummary {
                object: self.object_id.to_string(),
            });
        }
        let expected_least = self
            .claim_counts_by_evidence_level
            .iter()
            .filter(|(level, count)| level.is_physical() && **count > 0)
            .map(|(level, _)| *level)
            .max();
        let has_decorative = self
            .claim_counts_by_evidence_level
            .get(&EvidenceLevel::Decorative)
            .copied()
            .unwrap_or_default()
            > 0;
        if expected_least != self.least_supported_physical_level
            || has_decorative != self.has_decorative_claims
        {
            return Err(ProvenanceError::InvalidObjectEvidenceSummary {
                object: self.object_id.to_string(),
            });
        }
        Ok(())
    }
}

impl TryFrom<ObjectEvidenceSummaryWire> for ObjectEvidenceSummary {
    type Error = ProvenanceError;

    fn try_from(value: ObjectEvidenceSummaryWire) -> Result<Self, Self::Error> {
        let summary = Self {
            object_id: value.object_id,
            claim_counts_by_evidence_level: value.claim_counts_by_evidence_level,
            total_claim_count: value.total_claim_count,
            least_supported_physical_level: value.least_supported_physical_level,
            has_decorative_claims: value.has_decorative_claims,
            extrapolated_claim_count: value.extrapolated_claim_count,
            has_extrapolated_claims: value.has_extrapolated_claims,
            unquantified_uncertainty_claim_count: value.unquantified_uncertainty_claim_count,
            has_unquantified_uncertainty: value.has_unquantified_uncertainty,
            rejected_outcome_count: value.rejected_outcome_count,
            unsupported_outcome_count: value.unsupported_outcome_count,
        };
        summary.validate()?;
        Ok(summary)
    }
}
