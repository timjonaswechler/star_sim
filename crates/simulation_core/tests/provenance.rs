use std::collections::BTreeMap;

use simulation_core::{
    AleatoryVariation, ClaimApplicability, ClaimDerivation, ClaimExtrapolation, ClaimOutcome,
    ClaimProvenance, ClaimUncertainty, ConstraintClass, ConstraintEvaluation, CorrelationGroup,
    EpistemicUncertainty, EvidenceLevel, ExtrapolatedInputAxis, ExtrapolationDirection,
    GeneratingPrescription, ModelRealization, ModelRealizationId, ObjectEvidenceSummary, ObjectId,
    PrescriptionId, ProvenanceDocument, RandomDrawAddress, ScientificClaim, ScientificSource,
    ScientificSourceCatalog, ScientificSourceReference, SourceId, UncertaintyRepresentation,
    UnsupportedReason, ValidationReceipt,
};

fn source_catalog() -> ScientificSourceCatalog {
    ScientificSourceCatalog::new(
        42,
        vec![ScientificSource {
            id: SourceId::from("source.kepler-2019"),
            title: "A measured occurrence rate".into(),
            authors: vec!["A. Author".into()],
            publication: Some("Astronomy Journal".into()),
            publication_year: Some(2019),
            doi: Some("10.0000/example".into()),
            url: None,
        }],
        vec![GeneratingPrescription {
            id: PrescriptionId::from("prescription.kepler-radius"),
            namespace: "planet/radius".into(),
            version: "1".into(),
            evidence_level: EvidenceLevel::Empirical,
            description: "Draw a radius inside the measured domain".into(),
            source_references: vec![ScientificSourceReference {
                source_id: SourceId::from("source.kepler-2019"),
                locator: Some("Table 2, radius bin 1".into()),
            }],
        }],
        vec![],
        vec![],
    )
    .expect("test catalog is valid")
}

fn random_address(claim_key: &str) -> RandomDrawAddress {
    RandomDrawAddress::new(
        "ChaCha8",
        "1",
        "planet/radius",
        ObjectId::from("star-1"),
        claim_key,
        0,
    )
    .expect("test draw address is valid")
}

fn empirical_provenance(claim_key: &str) -> ClaimProvenance {
    ClaimProvenance::new(
        ObjectId::from("star-1"),
        claim_key,
        EvidenceLevel::Empirical,
        PrescriptionId::from("prescription.kepler-radius"),
        vec![ScientificSourceReference {
            source_id: SourceId::from("source.kepler-2019"),
            locator: Some("Table 2, radius bin 1".into()),
        }],
        ClaimApplicability::inside_domain(
            "Kepler FGK host and radius calibration",
            BTreeMap::from([(String::from("radius_rearth"), 1.4)]),
        )
        .expect("test applicability is valid"),
        ClaimUncertainty::not_quantified("the source supplied no claim-level interval")
            .expect("test uncertainty is valid"),
        None,
        Some(random_address(claim_key)),
    )
    .expect("test provenance is valid")
}

fn successful_receipt() -> ValidationReceipt {
    ValidationReceipt::new(
        "planetary-stability",
        "1",
        vec![],
        vec![
            ConstraintEvaluation::passed(
                "inside-circumstellar-zone",
                Some(0.4),
                Some(1.0),
                Some(0.6),
                Some("candidate is inside the evaluated zone"),
            )
            .expect("test constraint is valid"),
        ],
    )
    .expect("test receipt is valid")
}

#[test]
fn accepted_claim_document_round_trips_through_ron() {
    let claim = ScientificClaim::new(
        "star-1/planet-1/radius",
        1.4_f64,
        empirical_provenance("radius"),
    )
    .expect("test claim is valid");
    let outcome = ClaimOutcome::Accepted(claim, successful_receipt());
    let object_summary = ObjectEvidenceSummary::from_outcomes(
        ObjectId::from("star-1"),
        std::slice::from_ref(&outcome),
    )
    .expect("summary is valid");
    let document =
        ProvenanceDocument::new(source_catalog(), vec![outcome], vec![object_summary])
            .expect("document is valid");

    let encoded = ron::to_string(&document).expect("document serializes");
    let decoded: ProvenanceDocument<f64> = ron::from_str(&encoded).expect("document deserializes");

    assert_eq!(decoded, document);
    assert_eq!(decoded.outcomes.len(), 1);
}

#[test]
fn empirical_claims_require_sources_and_inside_domain_applicability() {
    let mut provenance = empirical_provenance("radius");
    provenance.source_references.clear();
    assert!(
        ClaimProvenance::new(
            provenance.object_id,
            provenance.claim_key,
            provenance.evidence_level,
            provenance.generating_prescription,
            provenance.source_references,
            provenance.applicability,
            provenance.uncertainty,
            provenance.derivation,
            provenance.random_draw_address,
        )
        .is_err()
    );

    let mut provenance = empirical_provenance("radius");
    provenance.applicability = ClaimApplicability::PresentationOnly;
    assert!(ScientificClaim::new("claim", 1.0_f64, provenance).is_err());
}

#[test]
fn accepted_and_rejected_outcomes_require_matching_validation_results() {
    let claim = ScientificClaim::new(
        "star-1/planet-1/radius",
        1.4_f64,
        empirical_provenance("radius"),
    )
    .expect("test claim is valid");

    let failed_receipt = ValidationReceipt::new(
        "planetary-stability",
        "1",
        vec![],
        vec![
            ConstraintEvaluation::failed(
                "inside-circumstellar-zone",
                Some(2.0),
                Some(1.0),
                Some(-1.0),
                Some("candidate is outside the evaluated zone"),
            )
            .expect("test constraint is valid"),
        ],
    )
    .expect("test receipt is valid");

    assert!(
        ClaimOutcome::Accepted(claim.clone(), failed_receipt.clone())
            .validate()
            .is_err()
    );
    assert!(
        ClaimOutcome::Rejected(claim.clone(), successful_receipt())
            .validate()
            .is_err()
    );
    assert!(
        ClaimOutcome::Rejected(claim, failed_receipt)
            .validate()
            .is_ok()
    );
}

#[test]
fn derived_decorative_input_cannot_be_declared_physical() {
    let mut catalog = source_catalog();
    catalog.prescriptions.push(GeneratingPrescription {
        id: PrescriptionId::from("prescription.decorative-shape"),
        namespace: "presentation/shape".into(),
        version: "1".into(),
        evidence_level: EvidenceLevel::Decorative,
        description: "Bounded display-only shape variation".into(),
        source_references: vec![],
    });
    catalog.prescriptions.push(GeneratingPrescription {
        id: PrescriptionId::from("prescription.proxy-temperature"),
        namespace: "planet/temperature".into(),
        version: "1".into(),
        evidence_level: EvidenceLevel::PhysicalProxy,
        description: "Transfer a named physical proxy".into(),
        source_references: vec![],
    });
    catalog.validate().expect("extended catalog is valid");

    let mut decorative_provenance = empirical_provenance("shape");
    decorative_provenance.evidence_level = EvidenceLevel::Decorative;
    decorative_provenance.generating_prescription =
        PrescriptionId::from("prescription.decorative-shape");
    decorative_provenance.source_references.clear();
    decorative_provenance.applicability = ClaimApplicability::PresentationOnly;
    let decorative = ScientificClaim::new("star-1/planet-1/shape", 1.0_f64, decorative_provenance)
        .expect("decorative claim is valid");

    let mut proxy_provenance = empirical_provenance("temperature");
    proxy_provenance.evidence_level = EvidenceLevel::PhysicalProxy;
    proxy_provenance.generating_prescription =
        PrescriptionId::from("prescription.proxy-temperature");
    proxy_provenance.source_references.clear();
    proxy_provenance.derivation =
        Some(ClaimDerivation::new(vec![decorative.id.clone()]).expect("derivation is valid"));
    let proxy = ScientificClaim::new("star-1/planet-1/temperature", 800.0_f64, proxy_provenance)
        .expect("claim is locally valid");

    let outcomes = vec![
        ClaimOutcome::Accepted(decorative, successful_receipt()),
        ClaimOutcome::Accepted(proxy, successful_receipt()),
    ];
    let summaries = vec![
        ObjectEvidenceSummary::from_outcomes(ObjectId::from("star-1"), &outcomes)
            .expect("summary is valid"),
    ];
    let document = ProvenanceDocument::new(catalog, outcomes, summaries);
    assert!(document.is_err());
}

#[test]
fn stochastic_claims_and_non_selection_keep_stable_draw_addresses() {
    let mut provenance = empirical_provenance("radius");
    provenance.random_draw_address = None;
    provenance.uncertainty = ClaimUncertainty::new(
        Some(
            AleatoryVariation::new(
                UncertaintyRepresentation::parametric_distribution(
                    "log_uniform",
                    BTreeMap::from([
                        (String::from("minimum"), 1.0),
                        (String::from("maximum"), 2.0),
                    ]),
                )
                .expect("distribution is valid"),
            )
            .expect("aleatory variation is valid"),
        ),
        None,
    )
    .expect("uncertainty is valid");

    assert!(ScientificClaim::new("star-1/planet-1/radius", 1.4_f64, provenance.clone()).is_err());
    let unsupported: ClaimOutcome<f64> = ClaimOutcome::Unsupported(
        provenance.clone(),
        vec![
            UnsupportedReason::new("outside-domain", "no calibrated model")
                .expect("reason is valid"),
        ],
    );
    assert!(unsupported.validate().is_err());

    let not_selected: ClaimOutcome<f64> =
        ClaimOutcome::NotSelected(provenance, random_address("radius"));
    assert!(not_selected.validate().is_ok());
}

#[test]
fn proxy_extrapolation_and_correlated_epistemic_uncertainty_round_trip() {
    let mut catalog = source_catalog();
    catalog.prescriptions.push(GeneratingPrescription {
        id: PrescriptionId::from("prescription.proxy-temperature"),
        namespace: "planet/temperature".into(),
        version: "1".into(),
        evidence_level: EvidenceLevel::PhysicalProxy,
        description: "Named transfer proxy for a temperature estimate".into(),
        source_references: vec![],
    });
    catalog.model_realizations.push(ModelRealization {
        id: ModelRealizationId::from("realization-42"),
        version: "stellar-v1".into(),
        seed: 42,
        description: "One shared epistemic model selection".into(),
    });
    catalog.correlation_groups.push(CorrelationGroup {
        id: "host-temperature".into(),
        description: "Shared host-temperature model parameters".into(),
        model_realization_id: "realization-42".into(),
    });
    catalog.validate().expect("proxy catalog is valid");

    let extrapolation = ClaimExtrapolation::new(
        "calibrated host-temperature rectangle",
        vec![
            ExtrapolatedInputAxis::new(
                "host_mass_msun",
                0.7,
                1.3,
                1.8,
                ExtrapolationDirection::AboveMaximum,
                0.5,
            )
            .expect("axis is valid"),
        ],
        "bounded physical transfer",
    )
    .expect("extrapolation is valid");
    let uncertainty = ClaimUncertainty::new(
        None,
        Some(
            EpistemicUncertainty::new(
                UncertaintyRepresentation::asymmetric_interval(40.0, 60.0, Some(0.9))
                    .expect("interval is valid"),
                Some("realization-42".into()),
                Some("host-temperature".into()),
            )
            .expect("epistemic uncertainty is valid"),
        ),
    )
    .expect("uncertainty is valid");
    let provenance = ClaimProvenance::new(
        "star-1",
        "temperature",
        EvidenceLevel::PhysicalProxy,
        "prescription.proxy-temperature",
        vec![],
        ClaimApplicability::extrapolated(extrapolation).expect("applicability is valid"),
        uncertainty,
        None,
        None,
    )
    .expect("proxy provenance is valid");
    let claim = ScientificClaim::new("star-1/planet-1/temperature", 1_800.0_f64, provenance)
        .expect("proxy claim is valid");
    let outcomes = vec![ClaimOutcome::Accepted(claim, successful_receipt())];
    let summaries = vec![
        ObjectEvidenceSummary::from_outcomes(ObjectId::from("star-1"), &outcomes)
            .expect("summary is valid"),
    ];
    let document =
        ProvenanceDocument::new(catalog, outcomes, summaries).expect("proxy document is valid");

    let encoded = ron::to_string(&document).expect("proxy document serializes");
    let decoded: ProvenanceDocument<f64> =
        ron::from_str(&encoded).expect("proxy document deserializes");
    assert_eq!(decoded, document);
}

#[test]
fn object_summaries_reject_inconsistent_derived_counts_during_deserialization() {
    let claim = ScientificClaim::new(
        "star-1/planet-1/radius",
        1.4_f64,
        empirical_provenance("radius"),
    )
    .expect("test claim is valid");
    let outcome = ClaimOutcome::Accepted(claim, successful_receipt());
    let mut summary = ObjectEvidenceSummary::from_outcomes(
        ObjectId::from("star-1"),
        std::slice::from_ref(&outcome),
    )
    .expect("summary is valid");
    summary.total_claim_count = 0;
    assert!(summary.validate().is_err());

    let encoded = ron::to_string(&summary).expect("summary serializes");
    assert!(ron::from_str::<ObjectEvidenceSummary>(&encoded).is_err());
}

#[test]
fn required_not_evaluated_constraints_prevent_acceptance() {
    let claim = ScientificClaim::new(
        "star-1/planet-1/radius",
        1.4_f64,
        empirical_provenance("radius"),
    )
    .expect("test claim is valid");
    let receipt = ValidationReceipt::new(
        "planetary-stability",
        "1",
        vec![],
        vec![
            ConstraintEvaluation::not_evaluated(
                "mutual-hill-spacing",
                ConstraintClass::Required,
                "planet mass is unavailable",
            )
            .expect("test constraint is valid"),
        ],
    )
    .expect("test receipt is valid");

    assert!(ClaimOutcome::Accepted(claim, receipt).validate().is_err());
}

#[test]
fn out_of_coverage_constraints_cannot_form_accepted_or_rejected_receipts() {
    assert!(
        ConstraintEvaluation::new(
            "n-body-coverage",
            ConstraintClass::OutOfCoverage,
            simulation_core::ConstraintStatus::Passed,
            None,
            None,
            None,
            Some("outside integration coverage".into()),
        )
        .is_err()
    );

    let claim = ScientificClaim::new(
        "star-1/planet-1/radius",
        1.4_f64,
        empirical_provenance("radius"),
    )
    .expect("test claim is valid");
    let receipt = ValidationReceipt::new(
        "planetary-stability",
        "1",
        vec![],
        vec![
            ConstraintEvaluation::not_evaluated(
                "n-body-coverage",
                ConstraintClass::OutOfCoverage,
                "outside integration coverage",
            )
            .expect("test constraint is valid"),
        ],
    )
    .expect("test receipt is valid");

    assert!(ClaimOutcome::Accepted(claim.clone(), receipt.clone()).validate().is_err());
    assert!(ClaimOutcome::Rejected(claim, receipt).validate().is_err());
}

#[test]
fn advisory_not_evaluated_constraints_allow_acceptance() {
    let claim = ScientificClaim::new(
        "star-1/planet-1/radius",
        1.4_f64,
        empirical_provenance("radius"),
    )
    .expect("test claim is valid");
    let receipt = ValidationReceipt::new(
        "planetary-stability",
        "1",
        vec![],
        vec![
            ConstraintEvaluation::not_evaluated(
                "n-body-integration",
                ConstraintClass::Advisory,
                "integration was not requested",
            )
            .expect("test constraint is valid"),
        ],
    )
    .expect("test receipt is valid");

    assert!(ClaimOutcome::Accepted(claim, receipt).validate().is_ok());
}

#[test]
fn documents_require_exactly_one_summary_for_each_outcome_object() {
    let claim = ScientificClaim::new(
        "star-1/planet-1/radius",
        1.4_f64,
        empirical_provenance("radius"),
    )
    .expect("test claim is valid");
    let outcomes = vec![ClaimOutcome::Accepted(claim, successful_receipt())];

    assert!(ProvenanceDocument::new(source_catalog(), outcomes, vec![]).is_err());
}

#[test]
fn non_selection_namespace_must_match_its_prescription() {
    let mut provenance = empirical_provenance("radius");
    provenance.uncertainty = ClaimUncertainty::new(
        Some(
            AleatoryVariation::new(
                UncertaintyRepresentation::not_quantified("draw")
                    .expect("uncertainty representation is valid"),
            )
            .expect("variation is valid"),
        ),
        None,
    )
    .expect("uncertainty is valid");
    provenance.random_draw_address = None;
    let wrong_address = RandomDrawAddress::new(
        "ChaCha8",
        "1",
        "wrong/namespace",
        ObjectId::from("star-1"),
        "radius",
        0,
    )
    .expect("address is locally valid");
    let outcomes: Vec<ClaimOutcome<f64>> =
        vec![ClaimOutcome::NotSelected(provenance, wrong_address)];
    let summaries = vec![
        ObjectEvidenceSummary::from_outcomes(ObjectId::from("star-1"), &outcomes)
            .expect("summary is valid"),
    ];

    assert!(ProvenanceDocument::new(source_catalog(), outcomes, summaries).is_err());
}

#[test]
fn decorative_claims_are_presentation_only() {
    let mut provenance = empirical_provenance("shape");
    provenance.evidence_level = EvidenceLevel::Decorative;
    provenance.source_references.clear();

    assert!(ScientificClaim::new("star-1/shape", 1.0_f64, provenance).is_err());
}

#[test]
fn extrapolation_departure_must_match_the_crossed_boundary() {
    assert!(
        ExtrapolatedInputAxis::new(
            "host_mass_msun",
            0.7,
            1.3,
            1.8,
            ExtrapolationDirection::AboveMaximum,
            99.0,
        )
        .is_err()
    );
}

#[test]
fn extrapolation_departure_rejects_tiny_mismatch_and_overflow() {
    assert!(
        ExtrapolatedInputAxis::new(
            "tiny",
            0.0,
            1.0,
            1.0 + f64::EPSILON,
            ExtrapolationDirection::AboveMaximum,
            f64::EPSILON * 2.0,
        )
        .is_err()
    );
    assert!(
        ExtrapolatedInputAxis::new(
            "overflow",
            -f64::MAX,
            -f64::MAX / 2.0,
            f64::MAX,
            ExtrapolationDirection::AboveMaximum,
            f64::MAX,
        )
        .is_err()
    );
}

#[test]
fn dangling_source_references_fail_during_deserialization() {
    let mut catalog = source_catalog();
    catalog.prescriptions[0].source_references[0].source_id = SourceId::from("missing");
    assert!(catalog.validate().is_err());

    let encoded = ron::to_string(&catalog).expect("catalog serializes");
    let decoded = ron::from_str::<ScientificSourceCatalog>(&encoded);
    assert!(decoded.is_err());
}
