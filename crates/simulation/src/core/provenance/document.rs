//! Cross-reference validation for complete provenance documents.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::claims::ClaimProvenance;
use super::error::ProvenanceError;
use super::identifiers::{ClaimId, EvidenceLevel, validate_unique};
use super::outcomes::ClaimOutcome;
use super::sources::ScientificSourceCatalog;
use super::summaries::ObjectEvidenceSummary;
use super::validation::ValidationReceipt;

/// A serializable provenance graph with all cross-reference invariants checked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    try_from = "ProvenanceDocumentWire<T>",
    bound(deserialize = "T: Deserialize<'de>")
)]
pub struct ProvenanceDocument<T> {
    /// Deduplicated sources, prescriptions, model realizations, seed, and groups.
    pub catalog: ScientificSourceCatalog,
    /// Every accepted, rejected, non-selected, or unsupported claim attempt.
    pub outcomes: Vec<ClaimOutcome<T>>,
    /// Object-level summaries that must match `outcomes` exactly.
    pub object_summaries: Vec<ObjectEvidenceSummary>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ProvenanceDocumentWire<T> {
    catalog: ScientificSourceCatalog,
    outcomes: Vec<ClaimOutcome<T>>,
    object_summaries: Vec<ObjectEvidenceSummary>,
}

impl<T> ProvenanceDocument<T> {
    /// Creates a document and performs all local and cross-reference validation.
    pub fn new(
        catalog: ScientificSourceCatalog,
        outcomes: Vec<ClaimOutcome<T>>,
        object_summaries: Vec<ObjectEvidenceSummary>,
    ) -> Result<Self, ProvenanceError> {
        let document = Self {
            catalog,
            outcomes,
            object_summaries,
        };
        document.validate()?;
        Ok(document)
    }

    /// Validates catalog references, unique claims, derivation acyclicity, and summaries.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.catalog.validate()?;

        let mut claims = BTreeMap::new();
        for outcome in &self.outcomes {
            outcome.validate()?;
            if let Some(claim) = outcome.claim()
                && claims
                    .insert(claim.id.clone(), claim.provenance.evidence_level)
                    .is_some()
            {
                return Err(ProvenanceError::DuplicateClaimOutcome {
                    claim: claim.id.to_string(),
                });
            }
        }

        let mut derivations = BTreeMap::new();
        for outcome in &self.outcomes {
            let provenance = outcome.provenance();
            let claim_label = outcome
                .claim()
                .map(|claim| claim.id.to_string())
                .unwrap_or_else(|| format!("{}/{}", provenance.object_id, provenance.claim_key));
            self.validate_provenance_references(
                provenance,
                outcome.random_draw_address(),
                &claims,
                &claim_label,
            )?;
            if let Some(claim) = outcome.claim() {
                if let Some(receipt) = outcome_validation_receipt(outcome) {
                    for input_claim in &receipt.input_claims {
                        if !claims.contains_key(input_claim) {
                            return Err(ProvenanceError::DanglingReference {
                                kind: "validation input claim",
                                id: input_claim.to_string(),
                            });
                        }
                    }
                }
                derivations.insert(
                    claim.id.clone(),
                    provenance
                        .derivation
                        .as_ref()
                        .map(|derivation| derivation.input_claims.clone())
                        .unwrap_or_default(),
                );
            }
        }
        validate_derivation_acyclic(&derivations)?;

        validate_unique(
            self.object_summaries
                .iter()
                .map(|summary| summary.object_id.clone()),
            "object evidence summary",
        )?;
        let outcome_object_ids = self
            .outcomes
            .iter()
            .map(|outcome| outcome.provenance().object_id.clone())
            .collect::<BTreeSet<_>>();
        let summary_object_ids = self
            .object_summaries
            .iter()
            .map(|summary| summary.object_id.clone())
            .collect::<BTreeSet<_>>();
        if outcome_object_ids != summary_object_ids {
            let object = outcome_object_ids
                .symmetric_difference(&summary_object_ids)
                .next()
                .expect("different sets have a member")
                .to_string();
            return Err(ProvenanceError::InvalidObjectEvidenceSummary { object });
        }
        for summary in &self.object_summaries {
            let expected =
                ObjectEvidenceSummary::from_outcomes(summary.object_id.clone(), &self.outcomes)?;
            if &expected != summary {
                return Err(ProvenanceError::InvalidObjectEvidenceSummary {
                    object: summary.object_id.to_string(),
                });
            }
        }
        Ok(())
    }

    fn validate_provenance_references(
        &self,
        provenance: &ClaimProvenance,
        random_draw_address: Option<&super::claims::RandomDrawAddress>,
        claims: &BTreeMap<ClaimId, EvidenceLevel>,
        claim_label: &str,
    ) -> Result<(), ProvenanceError> {
        let prescription = self
            .catalog
            .prescription(&provenance.generating_prescription)
            .ok_or_else(|| ProvenanceError::DanglingReference {
                kind: "prescription",
                id: provenance.generating_prescription.to_string(),
            })?;

        for reference in &provenance.source_references {
            if self.catalog.source(&reference.source_id).is_none() {
                return Err(ProvenanceError::DanglingReference {
                    kind: "source",
                    id: reference.source_id.to_string(),
                });
            }
            if !prescription.cites_source(&reference.source_id) {
                return Err(ProvenanceError::SourceNotDeclaredByPrescription {
                    source_id: reference.source_id.to_string(),
                    prescription: prescription.id.to_string(),
                });
            }
        }
        if let Some(address) = random_draw_address
            && address.prescription_namespace != prescription.namespace
        {
            return Err(ProvenanceError::RandomDrawAddressMismatch);
        }
        if let Some(epistemic) = &provenance.uncertainty.epistemic_uncertainty {
            if let Some(realization_id) = &epistemic.model_realization_id
                && self.catalog.model_realization(realization_id).is_none()
            {
                return Err(ProvenanceError::DanglingReference {
                    kind: "model realization",
                    id: realization_id.to_string(),
                });
            }
            if let Some(group_id) = &epistemic.correlation_group_id {
                let group = self.catalog.correlation_group(group_id).ok_or_else(|| {
                    ProvenanceError::DanglingReference {
                        kind: "correlation group",
                        id: group_id.to_string(),
                    }
                })?;
                if epistemic.model_realization_id.as_ref() != Some(&group.model_realization_id) {
                    return Err(ProvenanceError::CorrelationGroupModelRealizationMismatch);
                }
            }
        }
        if let Some(derivation) = &provenance.derivation {
            for input_claim in &derivation.input_claims {
                if !claims.contains_key(input_claim) {
                    return Err(ProvenanceError::DanglingReference {
                        kind: "claim",
                        id: input_claim.to_string(),
                    });
                }
            }
        }
        let mut effective = prescription.evidence_level;
        if let Some(derivation) = &provenance.derivation {
            for input_claim in &derivation.input_claims {
                let input_level = claims
                    .get(input_claim)
                    .copied()
                    .expect("validated derivation input reference");
                effective = effective.max(input_level);
            }
        }
        if effective != provenance.evidence_level {
            return Err(ProvenanceError::EvidenceLevelMismatch {
                claim: claim_label.to_owned(),
                declared: provenance.evidence_level.to_string(),
                effective: effective.to_string(),
            });
        }
        Ok(())
    }

    /// Recomputes summaries from outcomes without mutating the document.
    pub fn derived_object_summaries(&self) -> Result<Vec<ObjectEvidenceSummary>, ProvenanceError> {
        let mut object_ids = BTreeSet::new();
        for outcome in &self.outcomes {
            object_ids.insert(outcome.provenance().object_id.clone());
        }
        object_ids
            .into_iter()
            .map(|object_id| ObjectEvidenceSummary::from_outcomes(object_id, &self.outcomes))
            .collect()
    }
}

impl<T> TryFrom<ProvenanceDocumentWire<T>> for ProvenanceDocument<T> {
    type Error = ProvenanceError;

    fn try_from(value: ProvenanceDocumentWire<T>) -> Result<Self, Self::Error> {
        Self::new(value.catalog, value.outcomes, value.object_summaries)
    }
}

fn outcome_validation_receipt<T>(outcome: &ClaimOutcome<T>) -> Option<&ValidationReceipt> {
    match outcome {
        ClaimOutcome::Accepted(_, receipt) | ClaimOutcome::Rejected(_, receipt) => Some(receipt),
        ClaimOutcome::NotSelected(_, _) | ClaimOutcome::Unsupported(_, _) => None,
    }
}

fn validate_derivation_acyclic(
    derivations: &BTreeMap<ClaimId, Vec<ClaimId>>,
) -> Result<(), ProvenanceError> {
    fn visit(
        id: &ClaimId,
        derivations: &BTreeMap<ClaimId, Vec<ClaimId>>,
        visiting: &mut BTreeSet<ClaimId>,
        visited: &mut BTreeSet<ClaimId>,
    ) -> Result<(), ProvenanceError> {
        if visiting.contains(id) {
            return Err(ProvenanceError::ClaimDerivationCycle {
                claim: id.to_string(),
            });
        }
        if !visited.insert(id.clone()) {
            return Ok(());
        }
        visiting.insert(id.clone());
        if let Some(inputs) = derivations.get(id) {
            for input in inputs {
                visit(input, derivations, visiting, visited)?;
            }
        }
        visiting.remove(id);
        Ok(())
    }

    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for id in derivations.keys() {
        visit(id, derivations, &mut visiting, &mut visited)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_order_keeps_decorative_as_least_supported() {
        assert_eq!(
            EvidenceLevel::Empirical.max(EvidenceLevel::Decorative),
            EvidenceLevel::Decorative
        );
        assert!(EvidenceLevel::Empirical.is_physical());
        assert!(!EvidenceLevel::Decorative.is_physical());
    }
}
