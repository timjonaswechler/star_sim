//! Scientific provenance for generated claims.
//!
//! This module deliberately contains no domain-specific generator. It provides the
//! values, references, outcomes, and validation records that a generator can attach to
//! its own domain objects. A [`ProvenanceDocument`] is the boundary at which local
//! invariants and cross-reference invariants are checked together.
//!
//! ## Recommended workflow
//!
//! 1. Register bibliographic sources and versioned generation methods in a
//!    [`ScientificSourceCatalog`].
//! 2. Create a [`ClaimProvenance`] for each scientifically meaningful property. The
//!    provenance names the generating prescription, evidence level, applicability,
//!    uncertainty, and—when relevant—a [`RandomDrawAddress`].
//! 3. Wrap the value in a [`ScientificClaim`] and record the generation result as a
//!    [`ClaimOutcome`]. Keep rejected candidates and unsupported cases; do not silently
//!    turn them into missing data.
//! 4. Derive an [`ObjectEvidenceSummary`] and construct a [`ProvenanceDocument`]. Its
//!    validation performs the cross-reference checks that cannot be performed by an
//!    individual claim.
//!
//! Constructors validate immediately, and Serde deserialization uses the same checked
//! constructors. Therefore invalid states should be handled as `Result` errors rather
//! than assembled with unchecked defaults.
//!
//! ## Evidence and outcome rules
//!
//! - [`EvidenceLevel::Empirical`] requires source references and inside-domain
//!   applicability.
//! - [`EvidenceLevel::PhysicalProxy`] may be extrapolated, but the extrapolation must
//!   be explicit.
//! - [`EvidenceLevel::Decorative`] is presentation-only and cannot support a physical
//!   claim through a derivation.
//! - Aleatory variation requires a stable draw address for a realised claim.
//! - [`ClaimOutcome::Accepted`] requires a successful [`ValidationReceipt`], while
//!   [`ClaimOutcome::Rejected`] requires a failed constraint.
//! - [`ClaimOutcome::NotSelected`] has provenance and a draw address but no value;
//!   [`ClaimOutcome::Unsupported`] has provenance and at least one typed reason.
//!
//! ## Minimal example
//!
//! ```rust
//! # use std::collections::BTreeMap;
//! # use simulation_core::{
//! #     ClaimApplicability, ClaimOutcome, ClaimProvenance, ClaimUncertainty,
//! #     ConstraintEvaluation, EvidenceLevel, GeneratingPrescription, ObjectEvidenceSummary,
//! #     ObjectId, PrescriptionId, ProvenanceDocument, ProvenanceError, RandomDrawAddress,
//! #     ScientificClaim, ScientificSource, ScientificSourceCatalog, ScientificSourceReference,
//! #     SourceId, ValidationReceipt,
//! # };
//! # fn main() -> Result<(), ProvenanceError> {
//! let source_id = SourceId::from("source.example-2024");
//! let prescription_id = PrescriptionId::from("prescription.example-radius");
//! let source_reference = ScientificSourceReference {
//!     source_id: source_id.clone(),
//!     locator: Some("Table 1".into()),
//! };
//! let catalog = ScientificSourceCatalog::new(
//!     vec![ScientificSource::new(source_id, "Example measurement")?],
//!     vec![GeneratingPrescription::new(
//!         prescription_id.clone(),
//!         "planet/radius",
//!         "1",
//!         EvidenceLevel::Empirical,
//!         "Sample a radius inside the measured domain",
//!         vec![source_reference.clone()],
//!     )?],
//!     vec![],
//!     vec![],
//! )?;
//!
//! let claim_key = "radius";
//! let provenance = ClaimProvenance::new(
//!     ObjectId::from("star-1"),
//!     claim_key,
//!     EvidenceLevel::Empirical,
//!     prescription_id,
//!     vec![source_reference],
//!     ClaimApplicability::inside_domain(
//!         "example calibration",
//!         BTreeMap::from([(String::from("radius_rearth"), 1.4)]),
//!     )?,
//!     ClaimUncertainty::not_quantified("the example source supplies no interval")?,
//!     None,
//!     Some(RandomDrawAddress::new(
//!         "ChaCha8",
//!         "1",
//!         "planet/radius",
//!         ObjectId::from("star-1"),
//!         claim_key,
//!         0,
//!     )?),
//! )?;
//! let claim = ScientificClaim::new("star-1/planet-1/radius", 1.4_f64, provenance)?;
//! let receipt = ValidationReceipt::new(
//!     "planetary-stability",
//!     "1",
//!     vec![],
//!     vec![ConstraintEvaluation::passed(
//!         "inside-zone",
//!         Some(0.4),
//!         Some(1.0),
//!         Some(0.6),
//!         Some("candidate is inside the evaluated zone"),
//!     )?],
//! )?;
//! let outcome = ClaimOutcome::Accepted(claim, receipt);
//! let summary = ObjectEvidenceSummary::from_outcomes(
//!     ObjectId::from("star-1"),
//!     std::slice::from_ref(&outcome),
//! )?;
//! let document = ProvenanceDocument::new(catalog, vec![outcome], vec![summary])?;
//! assert_eq!(document.outcomes.len(), 1);
//! # Ok(())
//! # }
//! ```

mod applicability;
mod claims;
mod document;
mod error;
mod identifiers;
mod outcomes;
mod sources;
mod summaries;
mod uncertainty;
mod validation;

pub use applicability::*;
pub use claims::*;
pub use document::*;
pub use error::*;
pub use identifiers::*;
pub use outcomes::*;
pub use sources::*;
pub use summaries::*;
pub use uncertainty::*;
pub use validation::*;
