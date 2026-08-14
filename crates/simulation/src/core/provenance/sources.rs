//! Scientific source metadata, generating prescriptions, and catalogs.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::error::ProvenanceError;
use super::identifiers::{
    CorrelationGroupId, EvidenceLevel, ModelRealizationId, PrescriptionId, SourceId,
    validate_optional_text, validate_text, validate_unique,
};

/// A precise reference to one entry in the source catalog.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ScientificSourceReferenceWire")]
pub struct ScientificSourceReference {
    /// Stable key of the bibliographic source in the enclosing catalog.
    pub source_id: SourceId,
    /// A page, table, equation, row, or other source-local locator.
    pub locator: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ScientificSourceReferenceWire {
    source_id: SourceId,
    locator: Option<String>,
}

impl ScientificSourceReference {
    /// Validates the source key and optional locator.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.source_id.validate()?;
        validate_optional_text(&self.locator, "source reference locator")
    }
}

impl TryFrom<ScientificSourceReferenceWire> for ScientificSourceReference {
    type Error = ProvenanceError;

    fn try_from(value: ScientificSourceReferenceWire) -> Result<Self, Self::Error> {
        let reference = Self {
            source_id: value.source_id,
            locator: value.locator,
        };
        reference.validate()?;
        Ok(reference)
    }
}

/// Bibliographic metadata stored once and referenced by stable key.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ScientificSourceWire")]
pub struct ScientificSource {
    /// Stable bibliographic key referenced by claims and prescriptions.
    pub id: SourceId,
    /// Human-readable title of the source.
    pub title: String,
    /// Authors as recorded by the source catalog.
    pub authors: Vec<String>,
    /// Journal, book, archive, or other publication venue.
    pub publication: Option<String>,
    /// Publication year, when known.
    pub publication_year: Option<u16>,
    /// DOI, when available.
    pub doi: Option<String>,
    /// Stable URL, when available.
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ScientificSourceWire {
    id: SourceId,
    title: String,
    authors: Vec<String>,
    publication: Option<String>,
    publication_year: Option<u16>,
    doi: Option<String>,
    url: Option<String>,
}

impl ScientificSource {
    /// Creates a source with required bibliographic fields and empty optional metadata.
    pub fn new(id: impl Into<SourceId>, title: impl Into<String>) -> Result<Self, ProvenanceError> {
        let source = Self {
            id: id.into(),
            title: title.into(),
            authors: Vec::new(),
            publication: None,
            publication_year: None,
            doi: None,
            url: None,
        };
        source.validate()?;
        Ok(source)
    }

    /// Validates all required text fields and optional metadata.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.id.validate()?;
        validate_text(&self.title, "scientific source title")?;
        for author in &self.authors {
            validate_text(author, "scientific source author")?;
        }
        validate_optional_text(&self.publication, "scientific source publication")?;
        validate_optional_text(&self.doi, "scientific source DOI")?;
        validate_optional_text(&self.url, "scientific source URL")?;
        Ok(())
    }
}

impl TryFrom<ScientificSourceWire> for ScientificSource {
    type Error = ProvenanceError;

    fn try_from(value: ScientificSourceWire) -> Result<Self, Self::Error> {
        let source = Self {
            id: value.id,
            title: value.title,
            authors: value.authors,
            publication: value.publication,
            publication_year: value.publication_year,
            doi: value.doi,
            url: value.url,
        };
        source.validate()?;
        Ok(source)
    }
}

/// Immutable, versioned method identity for a generated claim.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "GeneratingPrescriptionWire")]
pub struct GeneratingPrescription {
    /// Stable identity of this versioned generation method.
    pub id: PrescriptionId,
    /// Namespace used to identify the generating algorithm and draw-address domain.
    pub namespace: String,
    /// Human-managed version of the prescription.
    pub version: String,
    /// Strongest evidence level directly supplied by this prescription.
    pub evidence_level: EvidenceLevel,
    /// Human-readable description of the generation method.
    pub description: String,
    /// Sources declared by the prescription and available for claim references.
    pub source_references: Vec<ScientificSourceReference>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct GeneratingPrescriptionWire {
    id: PrescriptionId,
    namespace: String,
    version: String,
    evidence_level: EvidenceLevel,
    description: String,
    source_references: Vec<ScientificSourceReference>,
}

impl GeneratingPrescription {
    /// Creates and validates a versioned generation method.
    pub fn new(
        id: impl Into<PrescriptionId>,
        namespace: impl Into<String>,
        version: impl Into<String>,
        evidence_level: EvidenceLevel,
        description: impl Into<String>,
        source_references: Vec<ScientificSourceReference>,
    ) -> Result<Self, ProvenanceError> {
        let prescription = Self {
            id: id.into(),
            namespace: namespace.into(),
            version: version.into(),
            evidence_level,
            description: description.into(),
            source_references,
        };
        prescription.validate()?;
        Ok(prescription)
    }

    /// Validates identity, source references, and empirical-source requirements.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.id.validate()?;
        validate_text(&self.namespace, "generating prescription namespace")?;
        validate_text(&self.version, "generating prescription version")?;
        validate_text(&self.description, "generating prescription description")?;
        let mut source_references = BTreeSet::new();
        for reference in &self.source_references {
            reference.validate()?;
            if !source_references.insert((reference.source_id.clone(), reference.locator.clone())) {
                return Err(ProvenanceError::DuplicateIdentifier {
                    kind: "source reference",
                    id: format!("{}:{:?}", reference.source_id, reference.locator),
                });
            }
        }
        if self.evidence_level == EvidenceLevel::Empirical && self.source_references.is_empty() {
            return Err(ProvenanceError::EmpiricalClaimWithoutSource);
        }
        Ok(())
    }

    /// Returns whether this prescription declares the given source.
    pub fn cites_source(&self, source_id: &SourceId) -> bool {
        self.source_references
            .iter()
            .any(|reference| &reference.source_id == source_id)
    }
}

impl TryFrom<GeneratingPrescriptionWire> for GeneratingPrescription {
    type Error = ProvenanceError;

    fn try_from(value: GeneratingPrescriptionWire) -> Result<Self, Self::Error> {
        let prescription = Self {
            id: value.id,
            namespace: value.namespace,
            version: value.version,
            evidence_level: value.evidence_level,
            description: value.description,
            source_references: value.source_references,
        };
        prescription.validate()?;
        Ok(prescription)
    }
}

/// One coherent selection of shared epistemic model parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "ModelRealizationWire")]
pub struct ModelRealization {
    /// Stable identity of the shared model-parameter realization.
    pub id: ModelRealizationId,
    /// Version of the model implementation or parameterization.
    pub version: String,
    /// Seed or realization selector used by the model.
    pub seed: u64,
    /// Human-readable description of the realized model parameters.
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ModelRealizationWire {
    id: ModelRealizationId,
    version: String,
    seed: u64,
    description: String,
}

impl ModelRealization {
    /// Validates the realization identity and description.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.id.validate()?;
        validate_text(&self.version, "model realization version")?;
        validate_text(&self.description, "model realization description")?;
        Ok(())
    }
}

impl TryFrom<ModelRealizationWire> for ModelRealization {
    type Error = ProvenanceError;

    fn try_from(value: ModelRealizationWire) -> Result<Self, Self::Error> {
        let realization = Self {
            id: value.id,
            version: value.version,
            seed: value.seed,
            description: value.description,
        };
        realization.validate()?;
        Ok(realization)
    }
}

/// A named set of claims that share an epistemic correlation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "CorrelationGroupWire")]
pub struct CorrelationGroup {
    /// Stable identity shared by correlated claims.
    pub id: CorrelationGroupId,
    /// Description of the correlation semantics.
    pub description: String,
    /// Required realization that defines the shared epistemic parameters.
    pub model_realization_id: ModelRealizationId,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct CorrelationGroupWire {
    id: CorrelationGroupId,
    description: String,
    model_realization_id: ModelRealizationId,
}

impl CorrelationGroup {
    /// Validates the group and its mandatory model-realization reference.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        self.id.validate()?;
        self.model_realization_id.validate()?;
        validate_text(&self.description, "correlation group description")?;
        Ok(())
    }
}

impl TryFrom<CorrelationGroupWire> for CorrelationGroup {
    type Error = ProvenanceError;

    fn try_from(value: CorrelationGroupWire) -> Result<Self, Self::Error> {
        let group = Self {
            id: value.id,
            description: value.description,
            model_realization_id: value.model_realization_id,
        };
        group.validate()?;
        Ok(group)
    }
}

/// Deduplicated source, prescription, model-realization, and correlation metadata.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(try_from = "ScientificSourceCatalogWire")]
pub struct ScientificSourceCatalog {
    /// Global simulation seed used to resolve deterministic claim draw addresses.
    pub simulation_seed: u64,
    /// Bibliographic sources referenced by prescriptions and claims.
    pub sources: Vec<ScientificSource>,
    /// Versioned generation methods used by claims.
    pub prescriptions: Vec<GeneratingPrescription>,
    /// Shared epistemic model realizations.
    pub model_realizations: Vec<ModelRealization>,
    /// Groups that describe correlated epistemic uncertainty.
    pub correlation_groups: Vec<CorrelationGroup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct ScientificSourceCatalogWire {
    simulation_seed: u64,
    sources: Vec<ScientificSource>,
    prescriptions: Vec<GeneratingPrescription>,
    model_realizations: Vec<ModelRealization>,
    correlation_groups: Vec<CorrelationGroup>,
}

impl ScientificSourceCatalog {
    /// Creates a catalog and validates uniqueness plus all internal references.
    pub fn new(
        simulation_seed: u64,
        sources: Vec<ScientificSource>,
        prescriptions: Vec<GeneratingPrescription>,
        model_realizations: Vec<ModelRealization>,
        correlation_groups: Vec<CorrelationGroup>,
    ) -> Result<Self, ProvenanceError> {
        let catalog = Self {
            simulation_seed,
            sources,
            prescriptions,
            model_realizations,
            correlation_groups,
        };
        catalog.validate()?;
        Ok(catalog)
    }

    /// Validates catalog entries, uniqueness, and references between entries.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        for source in &self.sources {
            source.validate()?;
        }
        for prescription in &self.prescriptions {
            prescription.validate()?;
        }
        for realization in &self.model_realizations {
            realization.validate()?;
        }
        for group in &self.correlation_groups {
            group.validate()?;
        }

        validate_unique(
            self.sources.iter().map(|source| source.id.clone()),
            "source",
        )?;
        validate_unique(
            self.prescriptions
                .iter()
                .map(|prescription| prescription.id.clone()),
            "prescription",
        )?;
        validate_unique(
            self.model_realizations
                .iter()
                .map(|realization| realization.id.clone()),
            "model realization",
        )?;
        validate_unique(
            self.correlation_groups.iter().map(|group| group.id.clone()),
            "correlation group",
        )?;

        for prescription in &self.prescriptions {
            for reference in &prescription.source_references {
                if self.source(&reference.source_id).is_none() {
                    return Err(ProvenanceError::DanglingReference {
                        kind: "source",
                        id: reference.source_id.to_string(),
                    });
                }
            }
        }
        for group in &self.correlation_groups {
            if self
                .model_realization(&group.model_realization_id)
                .is_none()
            {
                return Err(ProvenanceError::DanglingReference {
                    kind: "model realization",
                    id: group.model_realization_id.to_string(),
                });
            }
        }
        Ok(())
    }

    /// Looks up a bibliographic source by stable identifier.
    pub fn source(&self, id: &SourceId) -> Option<&ScientificSource> {
        self.sources.iter().find(|source| &source.id == id)
    }

    /// Looks up a generating prescription by stable identifier.
    pub fn prescription(&self, id: &PrescriptionId) -> Option<&GeneratingPrescription> {
        self.prescriptions
            .iter()
            .find(|prescription| &prescription.id == id)
    }

    /// Looks up a shared model realization by stable identifier.
    pub fn model_realization(&self, id: &ModelRealizationId) -> Option<&ModelRealization> {
        self.model_realizations
            .iter()
            .find(|realization| &realization.id == id)
    }

    /// Looks up a correlation group by stable identifier.
    pub fn correlation_group(&self, id: &CorrelationGroupId) -> Option<&CorrelationGroup> {
        self.correlation_groups.iter().find(|group| &group.id == id)
    }
}

impl TryFrom<ScientificSourceCatalogWire> for ScientificSourceCatalog {
    type Error = ProvenanceError;

    fn try_from(value: ScientificSourceCatalogWire) -> Result<Self, Self::Error> {
        Self::new(
            value.simulation_seed,
            value.sources,
            value.prescriptions,
            value.model_realizations,
            value.correlation_groups,
        )
    }
}
