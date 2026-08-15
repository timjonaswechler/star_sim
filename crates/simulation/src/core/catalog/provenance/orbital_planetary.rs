//! Orbital and planetary claim adapter for the catalog provenance seam.

use std::collections::{BTreeMap, HashMap};

use super::super::super::*;
use super::super::{
    CircumstellarStabilityClaim, ExplicitPlanetClaim, GeneratedStellarCatalog, OrbitalNodeClaim,
    OrbitalNodeClaimKind, PlanetOccurrenceClaim, RelativeStellarOrbitClaim,
    RelativeStellarOrbitScaleClaim, StellarClaimValue, StellarOrbitalHierarchyAttemptClaim,
    StellarOrbitalHierarchyClaim,
};

const ORBIT_SOURCE_M_DWARF: &str = "source.susemiehl-meyer-2022-m-dwarf-separations";
const ORBIT_SOURCE_SOLAR: &str = "source.raghavan-2010-solar-type-multiplicity";
const ORBIT_SOURCE_TOPOLOGY: &str = "source.tokovinin-2014-hierarchical-multiplicity";
const ORBIT_SOURCE_LOW_MASS_RADIUS: &str = "source.baraffe-2015-bhac15-low-mass-radius";
const STABILITY_SOURCE: &str = "source.holman-wiegert-1999-stability";
const MULTIPLE_STABILITY_SOURCE: &str = "source.verrier-evans-2007-hierarchical-stability";
const FGK_OCCURRENCE_SOURCE: &str = "source.petigura-2018-cks-occurrence";
const M_DWARF_OCCURRENCE_SOURCE: &str = "source.dressing-charbonneau-2015-occurrence";
const GIANT_OCCURRENCE_SOURCE: &str = "source.johnson-2010-giant-occurrence";
const GIANT_PROPERTIES_SOURCE: &str = "source.cumming-2008-giant-properties";
const CLOSE_BINARY_SOURCE: &str = "source.kraus-2016-close-binary-planet-suppression";

const ORBIT_PRESCRIPTION: &str = "prescription.stellar-orbital-hierarchy-static-field-v1";
const ORBIT_TOPOLOGY_PRESCRIPTION: &str = "prescription.stellar-orbit-quadruple-topology-v1";
const ORBIT_SCALE_PRESCRIPTION: &str = "prescription.relative-stellar-orbit-scale-v1";
const ORBIT_ECCENTRICITY_PRESCRIPTION: &str = "prescription.relative-stellar-orbit-eccentricity-v1";
const STABILITY_PRESCRIPTION: &str = "prescription.circumstellar-s-type-stability-v1";
const FGK_SUPER_EARTH_PRESCRIPTION: &str = "prescription.fgk-warm-super-earth-occurrence-v1";
const FGK_SUB_NEPTUNE_PRESCRIPTION: &str = "prescription.fgk-warm-sub-neptune-occurrence-v1";
const M_DWARF_SMALL_PRESCRIPTION: &str = "prescription.m-dwarf-small-planet-occurrence-v1";
const M_DWARF_SUB_EARTH_PRESCRIPTION: &str = "prescription.m-dwarf-sub-earth-occurrence-v1";
const GIANT_OCCURRENCE_PRESCRIPTION: &str = "prescription.giant-planet-occurrence-v1";
const OCCURRENCE_COVERAGE_PRESCRIPTION: &str = "prescription.planet-occurrence-coverage-v1";
const CLOSE_BINARY_SUPPRESSION_PRESCRIPTION: &str =
    "prescription.close-binary-planet-occurrence-suppression-v1";
const EXPLICIT_SOURCE_PRESCRIPTION: &str = "prescription.explicit-planet-source-channel-v1";
const EXPLICIT_PLANET_PRESCRIPTION: &str = "prescription.explicit-planet-candidate-derivation-v1";
const PLANET_SEMIMAJOR_AXIS_PRESCRIPTION: &str =
    "prescription.explicit-planet-semimajor-axis-derivation-v1";
const FGK_SMALL_RADIUS_PRESCRIPTION: &str = "prescription.fgk-small-planet-radius-v1";
const FGK_SMALL_PERIOD_PRESCRIPTION: &str = "prescription.fgk-small-planet-period-v1";
const M_DWARF_CELL_PRESCRIPTION: &str = "prescription.m-dwarf-occurrence-cell-selection-v1";
const M_DWARF_SUB_EARTH_CELL_PRESCRIPTION: &str =
    "prescription.m-dwarf-sub-earth-cell-selection-v1";
const M_DWARF_RADIUS_PRESCRIPTION: &str = "prescription.m-dwarf-planet-radius-v1";
const M_DWARF_PERIOD_PRESCRIPTION: &str = "prescription.m-dwarf-planet-period-v1";
const M_DWARF_SUB_EARTH_RADIUS_PRESCRIPTION: &str = "prescription.m-dwarf-sub-earth-radius-v1";
const M_DWARF_SUB_EARTH_PERIOD_PRESCRIPTION: &str = "prescription.m-dwarf-sub-earth-period-v1";
const GIANT_MASS_PRESCRIPTION: &str = "prescription.fgk-doppler-giant-minimum-mass-v1";
const GIANT_PERIOD_PRESCRIPTION: &str = "prescription.fgk-doppler-giant-period-v1";
const UNRESOLVED_PLANET_PRESCRIPTION: &str = "prescription.unresolved-planet-population-v1";

pub(super) struct Registrations {
    pub(super) sources: Vec<ScientificSource>,
    pub(super) prescriptions: Vec<GeneratingPrescription>,
}

pub(super) fn registrations() -> Result<Registrations, ProvenanceError> {
    let orbit_m_dwarf = source(
        ORBIT_SOURCE_M_DWARF,
        "The M-dwarf binary fraction as a function of separation",
        2022,
        "10.1051/0004-6361/202038582",
        "https://arxiv.org/abs/2109.05951",
    )?;
    let orbit_solar = source(
        ORBIT_SOURCE_SOLAR,
        "A Survey of Stellar Families: Multiplicity of Solar-type Stars",
        2010,
        "10.1088/0067-0049/190/1/1",
        "https://arxiv.org/abs/1007.0414",
    )?;
    let orbit_topology = source(
        ORBIT_SOURCE_TOPOLOGY,
        "From binaries to multiples. II. Hierarchical multiplicity of F and G dwarfs",
        2014,
        "10.1088/0004-6256/147/4/87",
        "https://arxiv.org/abs/1401.6827",
    )?;
    let low_mass_radius = source(
        ORBIT_SOURCE_LOW_MASS_RADIUS,
        "New evolutionary models for pre-main sequence and main sequence low-mass stars down to the hydrogen-burning limit",
        2015,
        "10.1051/0004-6361/201425481",
        "https://perso.ens-lyon.fr/isabelle.baraffe/BHAC15dir/BHAC15_tracks+structure",
    )?;
    let stability = source(
        STABILITY_SOURCE,
        "Long-Term Stability of Planets in Binary Systems",
        1999,
        "10.1086/300695",
        "https://physics.uwo.ca/~pwiegert/papers/1999AJ.117.621.pdf",
    )?;
    let multiple_stability = source(
        MULTIPLE_STABILITY_SOURCE,
        "Planetary stability zones in hierarchical triple star systems",
        2007,
        "10.1111/j.1365-2966.2007.12493.x",
        "https://arxiv.org/abs/0710.1167",
    )?;
    let fgk = source(
        FGK_OCCURRENCE_SOURCE,
        "The California-Kepler Survey. IV. Metal-rich Stars Host a Greater Diversity of Planets",
        2018,
        "10.3847/1538-3881/aaa54c",
        "https://arxiv.org/abs/1712.04042",
    )?;
    let m_dwarf = source(
        M_DWARF_OCCURRENCE_SOURCE,
        "The Occurrence of Potentially Habitable Planets Orbiting M Dwarfs",
        2015,
        "10.1088/0004-637X/807/1/45",
        "https://arxiv.org/abs/1501.01623",
    )?;
    let giant = source(
        GIANT_OCCURRENCE_SOURCE,
        "Giant Planet Occurrence in the Stellar Mass-Metallicity Plane",
        2010,
        "10.1086/655775",
        "https://arxiv.org/abs/1005.3084",
    )?;
    let giant_properties = source(
        GIANT_PROPERTIES_SOURCE,
        "The Keck Planet Search: Detectability and the Minimum Mass and Orbital Period Distribution of Extrasolar Planets",
        2008,
        "10.1086/588487",
        "https://arxiv.org/abs/0803.3357",
    )?;

    let orbit_refs = vec![
        reference(ORBIT_SOURCE_M_DWARF, "normalized M-dwarf separation shape"),
        reference(
            ORBIT_SOURCE_SOLAR,
            "solar-type period and eccentricity distributions",
        ),
        reference(
            ORBIT_SOURCE_LOW_MASS_RADIUS,
            "BHAC15 low-mass contact-radius proxy anchor",
        ),
    ];
    let stability_ref = reference(STABILITY_SOURCE, "S-type critical semimajor-axis fit");
    let fgk_ref = reference(FGK_OCCURRENCE_SOURCE, "warm FGK radius-period domains");
    let m_dwarf_ref = reference(
        M_DWARF_OCCURRENCE_SOURCE,
        "early-M-dwarf completeness-corrected occurrence domains",
    );
    let giant_ref = reference(
        GIANT_OCCURRENCE_SOURCE,
        "giant-host mass-metallicity relation",
    );
    let giant_properties_ref = reference(
        GIANT_PROPERTIES_SOURCE,
        "Doppler minimum-mass and period distribution",
    );
    let close_binary = source(
        CLOSE_BINARY_SOURCE,
        "The Impact of Stellar Multiplicity on Planetary Systems. I. The Ruinous Influence of Close Binary Companions",
        2016,
        "10.3847/0004-6256/152/1/8",
        "https://arxiv.org/abs/1604.05744",
    )?;

    Ok(Registrations {
        sources: vec![
            orbit_m_dwarf,
            orbit_solar,
            orbit_topology,
            low_mass_radius,
            stability,
            multiple_stability,
            fgk,
            m_dwarf,
            giant,
            giant_properties,
            close_binary,
        ],
        prescriptions: vec![
            GeneratingPrescription::new(
                ORBIT_PRESCRIPTION,
                "stellar_orbits/static_field_hierarchy/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Builds a deterministic latent hierarchy from staged field-star orbit distributions",
                orbit_refs,
            )?,
            GeneratingPrescription::new(
                ORBIT_TOPOLOGY_PRESCRIPTION,
                "stellar_orbit/quadruple_topology/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Selects the configured quadruple hierarchy topology",
                vec![reference(
                    ORBIT_SOURCE_TOPOLOGY,
                    "recursive F/G hierarchy experiment and topology frequencies",
                )],
            )?,
            GeneratingPrescription::new(
                ORBIT_SCALE_PRESCRIPTION,
                "stellar_orbit/scale/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples one hierarchy slot's orbital scale from the configured field-star distribution",
                vec![
                    reference(ORBIT_SOURCE_M_DWARF, "normalized M-dwarf separation shape"),
                    reference(ORBIT_SOURCE_SOLAR, "solar-type period distribution"),
                ],
            )?,
            GeneratingPrescription::new(
                ORBIT_ECCENTRICITY_PRESCRIPTION,
                "stellar_orbit/eccentricity/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples eccentricity above the configured circularization period",
                vec![reference(
                    ORBIT_SOURCE_SOLAR,
                    "solar-type eccentricity distribution and circularization scale",
                )],
            )?,
            GeneratingPrescription::new(
                STABILITY_PRESCRIPTION,
                "planetary_stability/holman_wiegert_s_type/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Evaluates the narrow S-type boundary and retains its declared approximations",
                vec![
                    stability_ref,
                    reference(
                        MULTIPLE_STABILITY_SOURCE,
                        "hierarchical sibling-subtree point-mass approximation",
                    ),
                ],
            )?,
            GeneratingPrescription::new(
                FGK_SUPER_EARTH_PRESCRIPTION,
                "planet_occurrence/fgk_super_earth/v1",
                "1",
                EvidenceLevel::Empirical,
                "Samples warm FGK super-Earth counts inside the CKS domain",
                vec![fgk_ref.clone()],
            )?,
            GeneratingPrescription::new(
                FGK_SUB_NEPTUNE_PRESCRIPTION,
                "planet_occurrence/fgk_sub_neptune/v1",
                "1",
                EvidenceLevel::Empirical,
                "Samples warm FGK sub-Neptune counts inside the CKS domain",
                vec![fgk_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_SMALL_PRESCRIPTION,
                "planet_occurrence/m_dwarf_small/v1",
                "1",
                EvidenceLevel::Empirical,
                "Samples early-M-dwarf 1-4 R_earth counts inside the measured domain",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_SUB_EARTH_PRESCRIPTION,
                "planet_occurrence/m_dwarf_sub_earth/v1",
                "1",
                EvidenceLevel::Empirical,
                "Samples early-M-dwarf sub-Earth counts inside measured cells",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                GIANT_OCCURRENCE_PRESCRIPTION,
                "planet_occurrence/cps_giant/v1",
                "1",
                EvidenceLevel::Empirical,
                "Samples whether a host has at least one giant in the CPS domain",
                vec![giant_ref.clone()],
            )?,
            GeneratingPrescription::new(
                CLOSE_BINARY_SUPPRESSION_PRESCRIPTION,
                "planet_occurrence/kraus2016_close_binary_suppression/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Transfers the Kraus et al. close-binary occurrence step to the generated host",
                vec![reference(
                    CLOSE_BINARY_SOURCE,
                    "47 AU step and 0.34 occurrence factor",
                )],
            )?,
            GeneratingPrescription::new(
                OCCURRENCE_COVERAGE_PRESCRIPTION,
                "planet_occurrence/coverage/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Maps typed occurrence-model coverage failures without clamping",
                vec![],
            )?,
            GeneratingPrescription::new(
                EXPLICIT_SOURCE_PRESCRIPTION,
                "explicit_planet/source_channel/v1",
                "1",
                EvidenceLevel::Empirical,
                "Retains the empirical occurrence channel that selected an explicit candidate",
                vec![fgk_ref.clone(), m_dwarf_ref.clone(), giant_ref],
            )?,
            GeneratingPrescription::new(
                FGK_SMALL_RADIUS_PRESCRIPTION,
                "explicit_planet/small_radius/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples a radius within the selected FGK occurrence bin",
                vec![fgk_ref.clone()],
            )?,
            GeneratingPrescription::new(
                FGK_SMALL_PERIOD_PRESCRIPTION,
                "explicit_planet/small_period/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples a period within the selected FGK occurrence bin",
                vec![fgk_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_CELL_PRESCRIPTION,
                "explicit_planet/m_dwarf_cell/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Selects one measured M-dwarf radius-period occurrence cell",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_SUB_EARTH_CELL_PRESCRIPTION,
                "explicit_planet/m_dwarf_sub_earth_cell/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Selects one measured M-dwarf sub-Earth occurrence cell",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_RADIUS_PRESCRIPTION,
                "explicit_planet/m_dwarf_radius/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples a radius within the selected M-dwarf occurrence cell",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_PERIOD_PRESCRIPTION,
                "explicit_planet/m_dwarf_period/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples a period within the selected M-dwarf occurrence cell",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_SUB_EARTH_RADIUS_PRESCRIPTION,
                "explicit_planet/m_dwarf_sub_earth_radius/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples a radius within a measured M-dwarf sub-Earth cell",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                M_DWARF_SUB_EARTH_PERIOD_PRESCRIPTION,
                "explicit_planet/m_dwarf_sub_earth_period/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples a period within a measured M-dwarf sub-Earth cell",
                vec![m_dwarf_ref.clone()],
            )?,
            GeneratingPrescription::new(
                GIANT_MASS_PRESCRIPTION,
                "explicit_planet/giant_minimum_mass/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples Doppler minimum mass from the Cumming et al. distribution",
                vec![giant_properties_ref.clone()],
            )?,
            GeneratingPrescription::new(
                GIANT_PERIOD_PRESCRIPTION,
                "explicit_planet/giant_period/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Samples period from the Cumming et al. distribution",
                vec![giant_properties_ref],
            )?,
            GeneratingPrescription::new(
                PLANET_SEMIMAJOR_AXIS_PRESCRIPTION,
                "explicit_planet/semimajor_axis_derivation/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Derives present-day semimajor axis from period and host current mass",
                vec![],
            )?,
            GeneratingPrescription::new(
                EXPLICIT_PLANET_PRESCRIPTION,
                "explicit_planet/candidate_derivation/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Combines addressed observable-property claims and applies the current S-type screen",
                vec![],
            )?,
            GeneratingPrescription::new(
                UNRESOLVED_PLANET_PRESCRIPTION,
                "explicit_planet/unresolved_population/v1",
                "1",
                EvidenceLevel::PhysicalProxy,
                "Retains positive occurrence whose properties cannot be materialised without inventing a distribution",
                vec![],
            )?,
        ],
    })
}

pub(super) fn append_outcomes(
    seed: u64,
    catalog: &GeneratedStellarCatalog,
    orbital_hierarchy_model: &StellarOrbitalHierarchyModel,
    planetary_stability_model: &PlanetaryStabilityModel,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<(), ProvenanceError> {
    for system in &catalog.systems {
        let system_object_id = system_object_id(system.id);
        let hierarchy_claim_id =
            ClaimId::from(format!("{system_object_id}/stellar_orbital_hierarchy"));
        let attempt_claim_ids = hierarchy_attempt_outcomes(system, model_realization_id, outcomes)?;
        match &system.orbital_hierarchy {
            Ok(hierarchy) => {
                let member_mass_claims = orbital_input_claim_ids(system);
                let (root_node, orbital_relationships) = orbital_relationships(
                    system.id,
                    hierarchy.sampling_attempt,
                    &hierarchy.root,
                );
                let relationship_claim_ids = orbital_relationships
                    .iter()
                    .map(|relationship| {
                        ClaimId::from(format!(
                            "{}/relative_stellar_orbit",
                            orbit_object_id(system.id, relationship)
                        ))
                    })
                    .collect::<Vec<_>>();
                let topology_claim_id = hierarchy
                    .quadruple_topology
                    .map(|topology| {
                        let claim_key = "quadruple_topology";
                        let claim_id = ClaimId::from(format!("{system_object_id}/{claim_key}"));
                        let address = RandomDrawAddress::new(
                            "blake3-seeded-chacha8-indexed",
                            "1",
                            "stellar_orbit/quadruple_topology/v1",
                            system_object_id.clone(),
                            claim_key,
                            0,
                        )?;
                        outcomes.push(ClaimOutcome::Accepted(
                            ScientificClaim::new(
                                claim_id.clone(),
                                StellarClaimValue::QuadrupleTopology(topology),
                                ClaimProvenance::new(
                                    system_object_id.clone(),
                                    claim_key,
                                    EvidenceLevel::PhysicalProxy,
                                    ORBIT_TOPOLOGY_PRESCRIPTION,
                                    vec![reference(
                                        ORBIT_SOURCE_TOPOLOGY,
                                        "recursive F/G hierarchy experiment and topology frequencies",
                                    )],
                                    ClaimApplicability::inside_domain(
                                        "configured quadruple topology support",
                                        BTreeMap::from([("stellar_member_count".into(), 4.0)]),
                                    )?,
                                    uncertainty(
                                        Some("configured quadruple topology variation"),
                                        "the topology split is an engineering proxy for higher-order multiplicity",
                                        model_realization_id,
                                    )?,
                                    Some(ClaimDerivation::new(member_mass_claims.clone())?),
                                    Some(address),
                                )?,
                            )?,
                            ValidationReceipt::new(
                                "stellar-orbit-quadruple-topology",
                                "1",
                                member_mass_claims.clone(),
                                vec![ConstraintEvaluation::passed(
                                    "quadruple-member-count",
                                    Some(system.members.len() as f64),
                                    Some(4.0),
                                    Some(system.members.len() as f64 - 4.0),
                                    None::<String>,
                                )?],
                            )?,
                        ));
                        Ok::<_, ProvenanceError>(claim_id)
                    })
                    .transpose()?;
                let mut hierarchy_inputs = member_mass_claims.clone();
                hierarchy_inputs.extend(relationship_claim_ids.clone());
                if let Some(topology_claim_id) = topology_claim_id {
                    hierarchy_inputs.push(topology_claim_id);
                }
                let provenance = ClaimProvenance::new(
                    system_object_id.clone(),
                    "stellar_orbital_hierarchy",
                    EvidenceLevel::PhysicalProxy,
                    ORBIT_PRESCRIPTION,
                    orbit_references(),
                    ClaimApplicability::inside_domain(
                        "configured static field-star hierarchy model",
                        BTreeMap::from([("stellar_member_count".into(), system.members.len() as f64)]),
                    )?,
                    uncertainty(
                        None,
                        "staged orbit distributions omit their full covariance and orbital evolution",
                        model_realization_id,
                    )?,
                    Some(ClaimDerivation::new(hierarchy_inputs.clone())?),
                    None,
                )?;
                outcomes.push(ClaimOutcome::Accepted(
                    ScientificClaim::new(
                        hierarchy_claim_id.clone(),
                        StellarClaimValue::StellarOrbitalHierarchy(StellarOrbitalHierarchyClaim {
                            root: root_node,
                            relative_orbit_count: orbital_relationships.len() as u8,
                            quadruple_topology: hierarchy.quadruple_topology,
                        }),
                        provenance,
                    )?,
                    ValidationReceipt::new(
                        "stellar-orbital-hierarchy-generation",
                        "1",
                        hierarchy_inputs,
                        vec![ConstraintEvaluation::passed(
                            "hierarchy-generation-completed",
                            Some(hierarchy.member_ids().len() as f64),
                            Some(system.members.len() as f64),
                            Some(hierarchy.member_ids().len() as f64 - system.members.len() as f64),
                            Some("every generated stellar member occurs in the hierarchy"),
                        )?],
                    )?,
                ));
                for (index, flag) in hierarchy.quality_flags.iter().enumerate() {
                    quality_flag_outcome(
                        system_object_id.clone(),
                        &format!("stellar_orbital_hierarchy_quality_{index}"),
                        StellarClaimValue::StellarOrbitalQualityFlag(*flag),
                        ORBIT_PRESCRIPTION,
                        vec![],
                        Some(hierarchy_claim_id.clone()),
                        model_realization_id,
                        outcomes,
                    )?;
                }

                for relationship in orbital_relationships {
                    let orbit = relationship.orbit;
                    let orbit_object_id = orbit_object_id(system.id, &relationship);
                    let scale_key = "relative_stellar_orbit_scale";
                    let scale_claim_id = ClaimId::from(format!("{orbit_object_id}/{scale_key}"));
                    let scale_address = RandomDrawAddress::new(
                        "blake3-seeded-chacha8-indexed",
                        "1",
                        "stellar_orbit/scale/v1",
                        orbit_object_id.clone(),
                        scale_key,
                        0,
                    )?;
                    outcomes.push(ClaimOutcome::Accepted(
                        ScientificClaim::new(
                            scale_claim_id.clone(),
                            StellarClaimValue::RelativeStellarOrbitScale(
                                RelativeStellarOrbitScaleClaim {
                                    semimajor_axis_au: orbit.semimajor_axis_au,
                                    period_days: orbit.period_days,
                                    combined_mass_msun: orbit.combined_mass_msun,
                                    sampling_attempt: relationship.sampling_attempt,
                                    sampling_slot: relationship.sampling_slot,
                                },
                            ),
                            ClaimProvenance::new(
                                orbit_object_id.clone(),
                                scale_key,
                                EvidenceLevel::PhysicalProxy,
                                ORBIT_SCALE_PRESCRIPTION,
                                orbit_references(),
                                ClaimApplicability::inside_domain(
                                    "configured static field-star orbit-scale support",
                                    BTreeMap::from([(
                                        "semimajor_axis_au".into(),
                                        orbit.semimajor_axis_au,
                                    )]),
                                )?,
                                uncertainty(
                                    Some("field-star orbital-scale variation"),
                                    "the staged scale distribution omits hierarchy-level covariance",
                                    model_realization_id,
                                )?,
                                Some(ClaimDerivation::new(member_mass_claims.clone())?),
                                Some(scale_address),
                            )?,
                        )?,
                        ValidationReceipt::new(
                            "relative-stellar-orbit-scale",
                            "1",
                            member_mass_claims.clone(),
                            vec![ConstraintEvaluation::passed(
                                "positive-semimajor-axis",
                                Some(orbit.semimajor_axis_au),
                                Some(0.0),
                                Some(orbit.semimajor_axis_au),
                                None::<String>,
                            )?],
                        )?,
                    ));

                    let eccentricity_key = "relative_stellar_orbit_eccentricity";
                    let eccentricity_claim_id =
                        ClaimId::from(format!("{orbit_object_id}/{eccentricity_key}"));
                    let stochastic_eccentricity = orbit.eccentricity > 0.0;
                    let eccentricity_address = stochastic_eccentricity
                        .then(|| {
                            RandomDrawAddress::new(
                                "blake3-seeded-chacha8-indexed",
                                "1",
                                "stellar_orbit/eccentricity/v1",
                                orbit_object_id.clone(),
                                eccentricity_key,
                                0,
                            )
                        })
                        .transpose()?;
                    outcomes.push(ClaimOutcome::Accepted(
                        ScientificClaim::new(
                            eccentricity_claim_id.clone(),
                            StellarClaimValue::RelativeStellarOrbitEccentricity(
                                orbit.eccentricity,
                            ),
                            ClaimProvenance::new(
                                orbit_object_id.clone(),
                                eccentricity_key,
                                EvidenceLevel::PhysicalProxy,
                                ORBIT_ECCENTRICITY_PRESCRIPTION,
                                vec![reference(
                                    ORBIT_SOURCE_SOLAR,
                                    "solar-type eccentricity distribution and circularization scale",
                                )],
                                ClaimApplicability::inside_domain(
                                    "configured relative-orbit eccentricity support",
                                    BTreeMap::from([
                                        ("period_days".into(), orbit.period_days),
                                        ("eccentricity".into(), orbit.eccentricity),
                                    ]),
                                )?,
                                uncertainty(
                                    stochastic_eccentricity.then_some(
                                        "bounded relative-orbit eccentricity variation",
                                    ),
                                    "the eccentricity prescription omits mass and hierarchy covariance",
                                    model_realization_id,
                                )?,
                                Some(ClaimDerivation::new(vec![scale_claim_id.clone()])?),
                                eccentricity_address,
                            )?,
                        )?,
                        ValidationReceipt::new(
                            "relative-stellar-orbit-eccentricity",
                            "1",
                            vec![scale_claim_id.clone()],
                            vec![ConstraintEvaluation::passed(
                                "bound-eccentricity",
                                Some(orbit.eccentricity),
                                Some(1.0),
                                Some(1.0 - orbit.eccentricity),
                                None::<String>,
                            )?],
                        )?,
                    ));

                    let claim_key = "relative_stellar_orbit";
                    outcomes.push(ClaimOutcome::Accepted(
                        ScientificClaim::new(
                            ClaimId::from(format!("{orbit_object_id}/{claim_key}")),
                            StellarClaimValue::RelativeStellarOrbit(relationship),
                            ClaimProvenance::new(
                                orbit_object_id.clone(),
                                claim_key,
                                EvidenceLevel::PhysicalProxy,
                                ORBIT_PRESCRIPTION,
                                orbit_references(),
                                ClaimApplicability::inside_domain(
                                    "configured static field-star orbit support",
                                    BTreeMap::from([
                                        ("semimajor_axis_au".into(), orbit.semimajor_axis_au),
                                        ("eccentricity".into(), orbit.eccentricity),
                                    ]),
                                )?,
                                uncertainty(
                                    None,
                                    "the relationship is derived from separately addressed scale and eccentricity claims",
                                    model_realization_id,
                                )?,
                                Some(ClaimDerivation::new({
                                    let mut inputs = member_mass_claims.clone();
                                    inputs.extend([
                                        scale_claim_id.clone(),
                                        eccentricity_claim_id.clone(),
                                    ]);
                                    inputs
                                })?),
                                None,
                            )?,
                        )?,
                        ValidationReceipt::new(
                            "relative-stellar-orbit-relationship",
                            "1",
                            {
                                let mut inputs = member_mass_claims.clone();
                                inputs.extend([scale_claim_id, eccentricity_claim_id]);
                                inputs
                            },
                            vec![ConstraintEvaluation::passed(
                                "orbital-children-resolve-within-system",
                                None,
                                None,
                                None,
                                Some("both child nodes are retained in the canonical hierarchy"),
                            )?],
                        )?,
                    ));
                }
            }
            Err(StellarOrbitalHierarchyError::StableHierarchySamplingExhausted) => {
                let claim_key = "stellar_orbital_hierarchy";
                let mut inputs = orbital_input_claim_ids(system);
                inputs.extend(attempt_claim_ids);
                let derivation = (!inputs.is_empty())
                    .then(|| ClaimDerivation::new(inputs.clone()))
                    .transpose()?;
                outcomes.push(ClaimOutcome::Rejected(
                    ScientificClaim::new(
                        hierarchy_claim_id.clone(),
                        StellarClaimValue::OrbitalHierarchySamplingExhaustion {
                            attempted_candidates: orbital_hierarchy_model
                                .stability
                                .maximum_sampling_attempts,
                        },
                        ClaimProvenance::new(
                            system_object_id.clone(),
                            claim_key,
                            EvidenceLevel::PhysicalProxy,
                            ORBIT_PRESCRIPTION,
                            orbit_references(),
                            ClaimApplicability::inside_domain(
                                "configured bounded hierarchy placement policy",
                                BTreeMap::from([(
                                    "maximum_sampling_attempts".into(),
                                    f64::from(
                                        orbital_hierarchy_model.stability.maximum_sampling_attempts,
                                    ),
                                )]),
                            )?,
                            uncertainty(
                                None,
                                "every bounded placement candidate failed contact or hierarchy stability checks",
                                model_realization_id,
                            )?,
                            derivation,
                            None,
                        )?,
                    )?,
                    ValidationReceipt::new(
                        "stellar-orbital-hierarchy-bounded-placement",
                        "1",
                        inputs,
                        vec![ConstraintEvaluation::failed(
                            "stable-hierarchy-candidate-found",
                            Some(0.0),
                            Some(1.0),
                            Some(-1.0),
                            Some("all configured deterministic placement attempts were rejected"),
                        )?],
                    )?,
                ));
            }
            Err(error) => outcomes.push(ClaimOutcome::Unsupported(
                ClaimProvenance::new(
                    system_object_id.clone(),
                    "stellar_orbital_hierarchy",
                    EvidenceLevel::PhysicalProxy,
                    ORBIT_PRESCRIPTION,
                    orbit_references(),
                    ClaimApplicability::outside_domain(
                        "configured static field-star hierarchy model",
                        BTreeMap::from([("stellar_member_count".into(), system.members.len() as f64)]),
                    )?,
                    uncertainty(
                        None,
                        "no hierarchy is invented when model coverage or required inputs are unavailable",
                        model_realization_id,
                    )?,
                    None,
                    None,
                )?,
                vec![orbital_unsupported_reason(error)?],
            )),
        }

        for member in &system.members {
            stability_outcome(
                system,
                &hierarchy_claim_id,
                member,
                planetary_stability_model,
                model_realization_id,
                outcomes,
            )?;
            let occurrence_claims =
                occurrence_outcomes(seed, system.id, member, model_realization_id, outcomes)?;
            planet_outcomes(
                seed,
                system.id,
                member,
                &occurrence_claims,
                model_realization_id,
                outcomes,
            )?;
        }
    }
    Ok(())
}

fn hierarchy_attempt_outcomes(
    system: &StellarCatalogSystem,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<Vec<ClaimId>, ProvenanceError> {
    let mut claim_ids = Vec::with_capacity(system.orbital_hierarchy_failed_attempts.len());
    for diagnostic in &system.orbital_hierarchy_failed_attempts {
        let object_id = ObjectId::from(format!(
            "{}/stellar-orbit-attempt:{:04}",
            system_object_id(system.id),
            diagnostic.attempt
        ));
        let claim_key = format!(
            "stellar_orbital_hierarchy_attempt_{:04}",
            diagnostic.attempt
        );
        let claim_id = ClaimId::from(format!("{object_id}/{claim_key}"));
        let (candidate_root, candidate_relationships) = diagnostic
            .candidate
            .as_ref()
            .map(|candidate| orbital_relationships(system.id, Some(diagnostic.attempt), candidate))
            .map_or((None, Vec::new()), |(root, relationships)| {
                (Some(root), relationships)
            });
        let input_claims = orbital_input_claim_ids(system);
        let constraints = diagnostic
            .constraints
            .iter()
            .map(hierarchy_attempt_constraint)
            .collect::<Result<Vec<_>, _>>()?;
        let address = RandomDrawAddress::new(
            "blake3-seeded-chacha8-indexed",
            "1",
            "stellar_orbits/static_field_hierarchy/v1",
            object_id.clone(),
            claim_key.clone(),
            0,
        )?;
        outcomes.push(ClaimOutcome::Rejected(
            ScientificClaim::new(
                claim_id.clone(),
                StellarClaimValue::OrbitalHierarchySamplingAttempt(
                    StellarOrbitalHierarchyAttemptClaim {
                        attempt: diagnostic.attempt,
                        candidate_root,
                        candidate_relationships,
                        constraints: diagnostic.constraints.clone(),
                    },
                ),
                ClaimProvenance::new(
                    object_id,
                    claim_key,
                    EvidenceLevel::PhysicalProxy,
                    ORBIT_PRESCRIPTION,
                    orbit_references(),
                    ClaimApplicability::inside_domain(
                        "configured bounded hierarchy placement policy",
                        BTreeMap::from([("attempt".into(), f64::from(diagnostic.attempt))]),
                    )?,
                    uncertainty(
                        Some("seed-deterministic bounded placement attempt"),
                        "the immutable candidate retains every failed evaluated constraint",
                        model_realization_id,
                    )?,
                    Some(ClaimDerivation::new(input_claims.clone())?),
                    Some(address),
                )?,
            )?,
            ValidationReceipt::new(
                "stellar-orbital-hierarchy-bounded-attempt",
                "1",
                input_claims,
                constraints,
            )?,
        ));
        claim_ids.push(claim_id);
    }
    Ok(claim_ids)
}

fn hierarchy_attempt_constraint(
    constraint: &StellarOrbitalHierarchyAttemptConstraint,
) -> Result<ConstraintEvaluation, ProvenanceError> {
    let (name, evaluated, threshold, margin, passed, detail) = match constraint {
        StellarOrbitalHierarchyAttemptConstraint::OrbitalScaleCalibration {
            sampling_slot,
            passed,
        } => (
            format!("orbital-scale-slot-{sampling_slot}-inside-calibration"),
            None,
            None,
            None,
            *passed,
            "the addressed scale draw must lie inside the configured bounded support",
        ),
        StellarOrbitalHierarchyAttemptConstraint::StellarContact {
            sampling_slot,
            periastron_au,
            minimum_separation_au,
            passed,
        } => (
            format!("stellar-contact-slot-{sampling_slot}"),
            Some(*periastron_au),
            Some(*minimum_separation_au),
            Some(*periastron_au - *minimum_separation_au),
            *passed,
            "the addressed leaf orbit must clear both finite contact radii",
        ),
        StellarOrbitalHierarchyAttemptConstraint::HierarchicalStability {
            outer_sampling_slot,
            semimajor_axis_ratio,
            critical_ratio,
            passed,
        } => (
            format!("hierarchical-stability-slot-{outer_sampling_slot}"),
            Some(*semimajor_axis_ratio),
            Some(*critical_ratio),
            Some(*semimajor_axis_ratio - *critical_ratio),
            *passed,
            "the addressed nested orbit must pass the configured hierarchy screen",
        ),
    };
    if passed {
        ConstraintEvaluation::passed(name, evaluated, threshold, margin, Some(detail))
    } else {
        ConstraintEvaluation::failed(name, evaluated, threshold, margin, Some(detail))
    }
}

fn stability_outcome(
    system: &StellarCatalogSystem,
    hierarchy_claim_id: &ClaimId,
    member: &StellarCatalogMember,
    planetary_stability_model: &PlanetaryStabilityModel,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<(), ProvenanceError> {
    let system_id = system.id;
    let object_id = member_object_id(system_id, member.birth.id);
    let claim_key = "circumstellar_s_type_stability_zone";
    match &member.circumstellar_stability_zone {
        Ok(zone) => {
            let mut base_claim_inputs = orbital_input_claim_ids(system);
            base_claim_inputs.push(hierarchy_claim_id.clone());
            let (value, domain_inputs, claim_inputs, constraints, quality_flags) = match zone {
                CircumstellarSTypeStabilityZone::UnboundedByStellarCompanion { .. } => (
                    CircumstellarStabilityClaim::UnboundedByStellarCompanion,
                    BTreeMap::from([("stellar_member_count".into(), 1.0)]),
                    base_claim_inputs,
                    vec![ConstraintEvaluation::passed(
                        "no-stellar-companion-boundary",
                        Some(0.0),
                        Some(0.0),
                        Some(0.0),
                        Some("the model found no stellar companion limiting this host scope"),
                    )?],
                    Vec::new(),
                ),
                CircumstellarSTypeStabilityZone::CompanionLimited {
                    nominal_outer_critical_semimajor_axis_au,
                    fit_residual_lower_semimajor_axis_au,
                    limiting_companion_mass_msun,
                    companion_mass_fraction,
                    limiting_relative_orbit,
                    quality_flags,
                    ..
                } => {
                    let limiting_relationship =
                        limiting_relationship(system, limiting_relative_orbit).expect(
                            "a supported stability zone references its generated hierarchy orbit",
                        );
                    let limiting_barycentre_id = limiting_relationship.barycentre.stable_id;
                    let mut claim_inputs = base_claim_inputs;
                    claim_inputs.push(ClaimId::from(format!(
                        "{}/relative_stellar_orbit",
                        orbit_object_id(system.id, &limiting_relationship)
                    )));
                    (
                        CircumstellarStabilityClaim::CompanionLimited {
                            model: planetary_stability_model.s_type,
                            nominal_outer_critical_semimajor_axis_au:
                                *nominal_outer_critical_semimajor_axis_au,
                            fit_residual_lower_semimajor_axis_au:
                                *fit_residual_lower_semimajor_axis_au,
                            limiting_companion_mass_msun: *limiting_companion_mass_msun,
                            companion_mass_fraction: *companion_mass_fraction,
                            limiting_barycentre_id,
                        },
                        BTreeMap::from([
                            ("companion_mass_fraction".into(), *companion_mass_fraction),
                            (
                                "binary_eccentricity".into(),
                                limiting_relative_orbit.eccentricity,
                            ),
                        ]),
                        claim_inputs,
                        vec![
                            ConstraintEvaluation::passed(
                                "positive-critical-semimajor-axis",
                                Some(*nominal_outer_critical_semimajor_axis_au),
                                Some(0.0),
                                Some(*nominal_outer_critical_semimajor_axis_au),
                                None::<String>,
                            )?,
                            ConstraintEvaluation::not_evaluated(
                                "long-term-and-multi-planet-stability",
                                ConstraintClass::Advisory,
                                "the source fit covers massless circular coplanar test particles for 10^4 binary periods",
                            )?,
                        ],
                        quality_flags.clone(),
                    )
                }
            };
            let provenance = ClaimProvenance::new(
                object_id.clone(),
                claim_key,
                EvidenceLevel::PhysicalProxy,
                STABILITY_PRESCRIPTION,
                vec![reference(
                    STABILITY_SOURCE,
                    "S-type critical semimajor-axis fit",
                )],
                ClaimApplicability::inside_domain(
                    "Holman-Wiegert S-type v1 support",
                    domain_inputs,
                )?,
                uncertainty(
                    None,
                    "the fit residual and omitted perturbations are not a guarantee of long-term stability",
                    model_realization_id,
                )?,
                Some(ClaimDerivation::new(claim_inputs.clone())?),
                None,
            )?;
            let stability_claim_id = ClaimId::from(format!("{object_id}/{claim_key}"));
            outcomes.push(ClaimOutcome::Accepted(
                ScientificClaim::new(
                    stability_claim_id.clone(),
                    StellarClaimValue::CircumstellarStability(value),
                    provenance,
                )?,
                ValidationReceipt::new(
                    "circumstellar-s-type-stability",
                    "1",
                    claim_inputs,
                    constraints,
                )?,
            ));
            for (index, flag) in quality_flags.into_iter().enumerate() {
                quality_flag_outcome(
                    object_id.clone(),
                    &format!("circumstellar_stability_quality_{index}"),
                    StellarClaimValue::CircumstellarStabilityQualityFlag(flag),
                    STABILITY_PRESCRIPTION,
                    stability_quality_references(flag),
                    Some(stability_claim_id.clone()),
                    model_realization_id,
                    outcomes,
                )?;
            }
        }
        Err(error) => outcomes.push(ClaimOutcome::Unsupported(
            ClaimProvenance::new(
                object_id,
                claim_key,
                EvidenceLevel::PhysicalProxy,
                STABILITY_PRESCRIPTION,
                vec![reference(
                    STABILITY_SOURCE,
                    "S-type critical semimajor-axis fit",
                )],
                ClaimApplicability::outside_domain(
                    "Holman-Wiegert S-type v1 support",
                    BTreeMap::new(),
                )?,
                uncertainty(
                    None,
                    "the stability boundary is not clamped or extrapolated outside coverage",
                    model_realization_id,
                )?,
                None,
                None,
            )?,
            vec![stability_unsupported_reason(error)?],
        )),
    }
    Ok(())
}

fn occurrence_outcomes(
    _seed: u64,
    system_id: u64,
    member: &StellarCatalogMember,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<HashMap<ExplicitPlanetSourceChannel, ClaimId>, ProvenanceError> {
    let mut selected_claims = HashMap::new();
    let suppression_claim_id = member
        .planet_population
        .close_binary_occurrence_factor
        .map(|factor| {
            let object_id = occurrence_object_id(system_id, member.birth.id);
            let claim_key = "close_binary_occurrence_factor";
            let claim_id = ClaimId::from(format!("{object_id}/{claim_key}"));
            let hierarchy_claim_id = ClaimId::from(format!(
                "{}/stellar_orbital_hierarchy",
                system_object_id(system_id)
            ));
            outcomes.push(ClaimOutcome::Accepted(
                ScientificClaim::new(
                    claim_id.clone(),
                    StellarClaimValue::CloseBinaryOccurrenceFactor(factor),
                    ClaimProvenance::new(
                        object_id,
                        claim_key,
                        EvidenceLevel::PhysicalProxy,
                        CLOSE_BINARY_SUPPRESSION_PRESCRIPTION,
                        vec![reference(
                            CLOSE_BINARY_SOURCE,
                            "47 AU step and 0.34 occurrence factor",
                        )],
                        ClaimApplicability::inside_domain(
                            "declared transfer of the Kraus et al. close-binary step",
                            BTreeMap::from([("occurrence_factor".into(), factor)]),
                        )?,
                        uncertainty(
                            None,
                            "the solar-type projected-separation calibration is transferred to the generated host and relative semimajor axis",
                            model_realization_id,
                        )?,
                        Some(ClaimDerivation::new(vec![hierarchy_claim_id.clone()])?),
                        None,
                    )?,
                )?,
                ValidationReceipt::new(
                    "close-binary-occurrence-suppression",
                    "1",
                    vec![hierarchy_claim_id],
                    vec![ConstraintEvaluation::passed(
                        "factor-inside-unit-interval",
                        Some(factor),
                        Some(1.0),
                        Some(1.0 - factor),
                        None::<String>,
                    )?],
                )?,
            ));
            Ok::<_, ProvenanceError>(claim_id)
        })
        .transpose()?;
    match &member.planet_population.small_planets {
        Ok(SmallPlanetOccurrence::FgkWarm {
            warm_super_earth_count,
            warm_sub_neptune_count,
        }) => {
            count_channel(
                system_id,
                member,
                suppression_claim_id.clone(),
                "fgk_warm_super_earth_occurrence",
                *warm_super_earth_count,
                ExplicitPlanetSourceChannel::FgkWarmSuperEarth,
                FGK_SUPER_EARTH_PRESCRIPTION,
                FGK_OCCURRENCE_SOURCE,
                "warm super-Earth CKS domain",
                "planet_occurrence/fgk_super_earth/v1",
                model_realization_id,
                outcomes,
                &mut selected_claims,
            )?;
            count_channel(
                system_id,
                member,
                suppression_claim_id.clone(),
                "fgk_warm_sub_neptune_occurrence",
                *warm_sub_neptune_count,
                ExplicitPlanetSourceChannel::FgkWarmSubNeptune,
                FGK_SUB_NEPTUNE_PRESCRIPTION,
                FGK_OCCURRENCE_SOURCE,
                "warm sub-Neptune CKS domain",
                "planet_occurrence/fgk_sub_neptune/v1",
                model_realization_id,
                outcomes,
                &mut selected_claims,
            )?;
        }
        Ok(SmallPlanetOccurrence::MDwarfAggregate {
            small_planet_count,
            sub_earth_count,
        }) => {
            count_channel(
                system_id,
                member,
                suppression_claim_id.clone(),
                "m_dwarf_small_planet_occurrence",
                *small_planet_count,
                ExplicitPlanetSourceChannel::MDwarfSmallPlanet,
                M_DWARF_SMALL_PRESCRIPTION,
                M_DWARF_OCCURRENCE_SOURCE,
                "1-4 R_earth and P < 200 d measured domain",
                "planet_occurrence/m_dwarf_small/v1",
                model_realization_id,
                outcomes,
                &mut selected_claims,
            )?;
            count_channel(
                system_id,
                member,
                suppression_claim_id.clone(),
                "m_dwarf_sub_earth_occurrence",
                *sub_earth_count,
                ExplicitPlanetSourceChannel::MDwarfSubEarth,
                M_DWARF_SUB_EARTH_PRESCRIPTION,
                M_DWARF_OCCURRENCE_SOURCE,
                "measured sub-Earth radius-period cells",
                "planet_occurrence/m_dwarf_sub_earth/v1",
                model_realization_id,
                outcomes,
                &mut selected_claims,
            )?;
        }
        Err(error) => occurrence_unsupported(
            system_id,
            member.birth.id,
            "small_planet_occurrence",
            error,
            model_realization_id,
            outcomes,
        )?,
    }

    match &member.planet_population.giant_planets {
        Ok(giant) if giant.has_at_least_one_cps_giant => {
            let object_id = occurrence_object_id(system_id, member.birth.id);
            let claim_key = "giant_planet_occurrence";
            let claim_id = ClaimId::from(format!("{object_id}/{claim_key}"));
            let address = draw_address(
                object_id.clone(),
                claim_key,
                "planet_occurrence/cps_giant/v1",
            )?;
            let provenance = empirical_occurrence_provenance(
                object_id,
                claim_key,
                GIANT_OCCURRENCE_PRESCRIPTION,
                GIANT_OCCURRENCE_SOURCE,
                "CPS giant-planet host domain",
                address,
                suppression_claim_id.clone(),
                model_realization_id,
            )?;
            outcomes.push(ClaimOutcome::Accepted(
                ScientificClaim::new(
                    claim_id.clone(),
                    StellarClaimValue::PlanetOccurrence(
                        PlanetOccurrenceClaim::HasAtLeastOneGiant {
                            multiplicity_suppression_extrapolated: member
                                .planet_population
                                .quality_flags
                                .contains(
                                    &PlanetOccurrenceQualityFlag::MultiplicitySuppressionExtrapolated,
                                ),
                        },
                    ),
                    provenance,
                )?,
                occurrence_receipt(
                    system_id,
                    member.birth.id,
                    claim_key,
                    suppression_claim_id.clone(),
                )?,
            ));
            selected_claims.insert(ExplicitPlanetSourceChannel::FgkDopplerGiant, claim_id);
        }
        Ok(_) => {
            let object_id = occurrence_object_id(system_id, member.birth.id);
            let claim_key = "giant_planet_occurrence";
            let address = draw_address(
                object_id.clone(),
                claim_key,
                "planet_occurrence/cps_giant/v1",
            )?;
            let provenance = empirical_occurrence_provenance(
                object_id,
                claim_key,
                GIANT_OCCURRENCE_PRESCRIPTION,
                GIANT_OCCURRENCE_SOURCE,
                "CPS giant-planet host domain",
                address.clone(),
                suppression_claim_id.clone(),
                model_realization_id,
            )?;
            outcomes.push(ClaimOutcome::NotSelected(provenance, address));
        }
        Err(error) => occurrence_unsupported(
            system_id,
            member.birth.id,
            "giant_planet_occurrence",
            error,
            model_realization_id,
            outcomes,
        )?,
    }
    let quality_input = suppression_claim_id
        .clone()
        .or_else(|| selected_claims.values().min().cloned());
    for (index, flag) in member.planet_population.quality_flags.iter().enumerate() {
        quality_flag_outcome(
            occurrence_object_id(system_id, member.birth.id),
            &format!("planet_occurrence_quality_{index}"),
            StellarClaimValue::PlanetOccurrenceQualityFlag(*flag),
            OCCURRENCE_COVERAGE_PRESCRIPTION,
            vec![],
            quality_input.clone(),
            model_realization_id,
            outcomes,
        )?;
    }
    Ok(selected_claims)
}

#[allow(clippy::too_many_arguments)]
fn count_channel(
    system_id: u64,
    member: &StellarCatalogMember,
    suppression_claim_id: Option<ClaimId>,
    claim_key: &str,
    count: u32,
    channel: ExplicitPlanetSourceChannel,
    prescription: &str,
    source_id: &str,
    source_locator: &str,
    namespace: &str,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
    selected_claims: &mut HashMap<ExplicitPlanetSourceChannel, ClaimId>,
) -> Result<(), ProvenanceError> {
    let object_id = occurrence_object_id(system_id, member.birth.id);
    let address = draw_address(object_id.clone(), claim_key, namespace)?;
    let provenance = empirical_occurrence_provenance(
        object_id.clone(),
        claim_key,
        prescription,
        source_id,
        source_locator,
        address.clone(),
        suppression_claim_id.clone(),
        model_realization_id,
    )?;
    if count == 0 {
        outcomes.push(ClaimOutcome::NotSelected(provenance, address));
    } else {
        let claim_id = ClaimId::from(format!("{object_id}/{claim_key}"));
        outcomes.push(ClaimOutcome::Accepted(
            ScientificClaim::new(
                claim_id.clone(),
                StellarClaimValue::PlanetOccurrence(PlanetOccurrenceClaim::PlanetCount {
                    count,
                    multiplicity_suppression_extrapolated: member
                        .planet_population
                        .quality_flags
                        .contains(
                            &PlanetOccurrenceQualityFlag::MultiplicitySuppressionExtrapolated,
                        ),
                }),
                provenance,
            )?,
            occurrence_receipt(system_id, member.birth.id, claim_key, suppression_claim_id)?,
        ));
        selected_claims.insert(channel, claim_id);
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn empirical_occurrence_provenance(
    object_id: ObjectId,
    claim_key: &str,
    prescription: &str,
    source_id: &str,
    source_locator: &str,
    address: RandomDrawAddress,
    suppression_claim_id: Option<ClaimId>,
    model_realization_id: &ModelRealizationId,
) -> Result<ClaimProvenance, ProvenanceError> {
    ClaimProvenance::new(
        object_id,
        claim_key,
        if suppression_claim_id.is_some() {
            EvidenceLevel::PhysicalProxy
        } else {
            EvidenceLevel::Empirical
        },
        prescription,
        vec![reference(source_id, source_locator)],
        ClaimApplicability::inside_domain(source_locator, BTreeMap::new())?,
        uncertainty(
            Some("seed-deterministic occurrence draw"),
            "survey completeness and fitted occurrence uncertainty remain source-specific",
            model_realization_id,
        )?,
        suppression_claim_id
            .map(|claim_id| ClaimDerivation::new(vec![claim_id]))
            .transpose()?,
        Some(address),
    )
}

fn occurrence_receipt(
    system_id: u64,
    member_id: u64,
    claim_key: &str,
    suppression_claim_id: Option<ClaimId>,
) -> Result<ValidationReceipt, ProvenanceError> {
    let mut inputs = vec![
        member_claim_id(system_id, member_id, "evolutionary_state"),
        system_claim_id(system_id, "iron_abundance_feh"),
    ];
    inputs.extend(suppression_claim_id);
    ValidationReceipt::new(
        "empirical-planet-occurrence-domain",
        "1",
        inputs,
        vec![ConstraintEvaluation::passed(
            format!("{claim_key}-source-domain"),
            None,
            None,
            None,
            Some("the generator returned a calibrated occurrence result"),
        )?],
    )
}

fn occurrence_unsupported(
    system_id: u64,
    member_id: u64,
    claim_key: &str,
    error: &PlanetOccurrenceError,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<(), ProvenanceError> {
    outcomes.push(ClaimOutcome::Unsupported(
        ClaimProvenance::new(
            occurrence_object_id(system_id, member_id),
            claim_key,
            EvidenceLevel::PhysicalProxy,
            OCCURRENCE_COVERAGE_PRESCRIPTION,
            vec![],
            ClaimApplicability::outside_domain(
                "configured planet-occurrence source domains",
                BTreeMap::new(),
            )?,
            uncertainty(
                None,
                "unsupported occurrence domains produce no planet value",
                model_realization_id,
            )?,
            None,
            None,
        )?,
        vec![occurrence_unsupported_reason(error)?],
    ));
    Ok(())
}

fn planet_outcomes(
    seed: u64,
    system_id: u64,
    member: &StellarCatalogMember,
    occurrence_claims: &HashMap<ExplicitPlanetSourceChannel, ClaimId>,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<(), ProvenanceError> {
    for candidate in &member.planetary_system.accepted_planets {
        planet_candidate(
            seed,
            system_id,
            candidate,
            None,
            &member.circumstellar_stability_zone,
            member
                .planet_population
                .close_binary_occurrence_factor
                .is_some(),
            occurrence_claims,
            model_realization_id,
            outcomes,
        )?;
    }
    for rejected in &member.planetary_system.rejected_candidates {
        planet_candidate(
            seed,
            system_id,
            &rejected.candidate,
            Some(&rejected.reason),
            &member.circumstellar_stability_zone,
            member
                .planet_population
                .close_binary_occurrence_factor
                .is_some(),
            occurrence_claims,
            model_realization_id,
            outcomes,
        )?;
    }
    for (index, unresolved) in member
        .planetary_system
        .unresolved_populations
        .iter()
        .enumerate()
    {
        let object_id = ObjectId::from(format!(
            "{}/unresolved-planet-population:{index:04}",
            member_object_id(system_id, member.birth.id)
        ));
        let (reason, derivation) = match unresolved {
            UnresolvedPlanetPopulation::MDwarfSmallPlanets { count } => (
                UnsupportedReason::new(
                    "m_dwarf_planet_properties_unresolved",
                    format!(
                        "the aggregate occurrence count {count} does not determine unique planet properties"
                    ),
                )?,
                occurrence_claims
                    .get(&ExplicitPlanetSourceChannel::MDwarfSmallPlanet)
                    .cloned(),
            ),
            UnresolvedPlanetPopulation::GiantPlanetPropertiesUnavailable => (
                UnsupportedReason::new(
                    "giant_planet_properties_unavailable",
                    "the positive giant occurrence result lies outside the explicit FGK property model",
                )?,
                occurrence_claims
                    .get(&ExplicitPlanetSourceChannel::FgkDopplerGiant)
                    .cloned(),
            ),
        };
        let derivation = derivation
            .map(|claim| ClaimDerivation::new(vec![claim]))
            .transpose()?;
        outcomes.push(ClaimOutcome::Unsupported(
            ClaimProvenance::new(
                object_id,
                "unresolved_planet_population",
                EvidenceLevel::PhysicalProxy,
                UNRESOLVED_PLANET_PRESCRIPTION,
                vec![],
                ClaimApplicability::outside_domain(
                    "explicit planet property distributions",
                    BTreeMap::new(),
                )?,
                uncertainty(
                    None,
                    "no distribution is invented to expand the positive occurrence result",
                    model_realization_id,
                )?,
                derivation,
                None,
            )?,
            vec![reason],
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn planet_candidate(
    _seed: u64,
    system_id: u64,
    candidate: &ExplicitPlanetCandidate,
    rejection: Option<&RejectedPlanetCandidateReason>,
    stability_zone: &Result<CircumstellarSTypeStabilityZone, PlanetaryStabilityError>,
    occurrence_is_suppressed_proxy: bool,
    occurrence_claims: &HashMap<ExplicitPlanetSourceChannel, ClaimId>,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<(), ProvenanceError> {
    let object_id = ObjectId::from(format!(
        "indexed-u64-le:{:016x}/stellar-system-owner:{system_id:016x}/stellar-member-host:{:016x}/explicit-planet",
        candidate.id, candidate.host_member_id
    ));
    let occurrence_claim = occurrence_claims
        .get(&candidate.source_channel)
        .cloned()
        .ok_or_else(|| ProvenanceError::DanglingReference {
            kind: "explicit planet occurrence channel",
            id: format!("{:?}", candidate.source_channel),
        })?;
    let source_reference = source_channel_reference(candidate.source_channel);
    let source_claim_id = ClaimId::from(format!("{object_id}/occurrence_source_channel"));
    outcomes.push(ClaimOutcome::Accepted(
        ScientificClaim::new(
            source_claim_id.clone(),
            StellarClaimValue::ExplicitPlanetSourceChannel(candidate.source_channel),
            ClaimProvenance::new(
                object_id.clone(),
                "occurrence_source_channel",
                if occurrence_is_suppressed_proxy {
                    EvidenceLevel::PhysicalProxy
                } else {
                    EvidenceLevel::Empirical
                },
                EXPLICIT_SOURCE_PRESCRIPTION,
                vec![source_reference],
                ClaimApplicability::inside_domain(
                    "selected empirical occurrence channel",
                    BTreeMap::new(),
                )?,
                uncertainty(
                    None,
                    "the source channel does not imply a complete planetary inventory",
                    model_realization_id,
                )?,
                Some(ClaimDerivation::new(vec![occurrence_claim.clone()])?),
                None,
            )?,
        )?,
        ValidationReceipt::new(
            "explicit-planet-source-channel",
            "1",
            vec![occurrence_claim],
            vec![ConstraintEvaluation::passed(
                "occurrence-channel-retained",
                None,
                None,
                None,
                Some("the explicit candidate retains its generating occurrence channel"),
            )?],
        )?,
    ));

    let cell_claim_id = candidate
        .source_cell_index
        .map(|cell_index| {
            let (prescription, namespace) = match candidate.source_channel {
                ExplicitPlanetSourceChannel::MDwarfSmallPlanet => {
                    (M_DWARF_CELL_PRESCRIPTION, "explicit_planet/m_dwarf_cell/v1")
                }
                ExplicitPlanetSourceChannel::MDwarfSubEarth => (
                    M_DWARF_SUB_EARTH_CELL_PRESCRIPTION,
                    "explicit_planet/m_dwarf_sub_earth_cell/v1",
                ),
                _ => unreachable!("only gridded M-dwarf channels retain a source cell"),
            };
            planet_property_outcome(
                object_id.clone(),
                "m_dwarf_occurrence_cell_index",
                StellarClaimValue::MDwarfOccurrenceCellSelection {
                    index: cell_index,
                    cell_count: candidate
                        .source_cell_count
                        .expect("gridded source cell retains its domain size"),
                },
                prescription,
                namespace,
                explicit_property_references(candidate.source_channel),
                source_claim_id.clone(),
                model_realization_id,
                outcomes,
            )
        })
        .transpose()?;
    let ((property_prescription, property_namespace), (period_prescription, period_namespace)) =
        explicit_property_prescriptions(candidate.source_channel);
    let property_references = explicit_property_references(candidate.source_channel);
    let (transit_radius_rearth, doppler_minimum_mass_mjup, property_key, property_value) =
        match candidate.properties {
            ExplicitPlanetProperties::TransitRadius { radius_rearth } => (
                Some(radius_rearth),
                None,
                "planet_transit_radius_rearth",
                StellarClaimValue::PlanetTransitRadiusRearth(radius_rearth),
            ),
            ExplicitPlanetProperties::DopplerMinimumMass { minimum_mass_mjup } => (
                None,
                Some(minimum_mass_mjup),
                "planet_doppler_minimum_mass_mjup",
                StellarClaimValue::PlanetDopplerMinimumMassMjup(minimum_mass_mjup),
            ),
        };
    let property_claim_id = planet_property_outcome(
        object_id.clone(),
        property_key,
        property_value,
        property_prescription,
        property_namespace,
        property_references.clone(),
        source_claim_id.clone(),
        model_realization_id,
        outcomes,
    )?;
    let period_claim_id = planet_property_outcome(
        object_id.clone(),
        "planet_orbital_period_days",
        StellarClaimValue::PlanetOrbitalPeriodDays(candidate.period_days),
        period_prescription,
        period_namespace,
        property_references,
        source_claim_id.clone(),
        model_realization_id,
        outcomes,
    )?;
    let semimajor_axis_claim_id = ClaimId::from(format!("{object_id}/planet_semimajor_axis_au"));
    let current_mass_claim_id = member_claim_id(
        system_id,
        candidate.host_member_id,
        "current_stellar_mass_msolar",
    );
    let semimajor_inputs = vec![period_claim_id.clone(), current_mass_claim_id];
    outcomes.push(ClaimOutcome::Accepted(
        ScientificClaim::new(
            semimajor_axis_claim_id.clone(),
            StellarClaimValue::PlanetSemimajorAxisAu(candidate.semimajor_axis_au),
            ClaimProvenance::new(
                object_id.clone(),
                "planet_semimajor_axis_au",
                EvidenceLevel::PhysicalProxy,
                PLANET_SEMIMAJOR_AXIS_PRESCRIPTION,
                vec![],
                ClaimApplicability::inside_domain(
                    "two-body Kepler semimajor-axis derivation",
                    BTreeMap::from([
                        ("period_days".into(), candidate.period_days),
                        ("semimajor_axis_au".into(), candidate.semimajor_axis_au),
                    ]),
                )?,
                uncertainty(
                    None,
                    "the derivation uses present host mass and omits planet mass",
                    model_realization_id,
                )?,
                Some(ClaimDerivation::new(semimajor_inputs.clone())?),
                None,
            )?,
        )?,
        ValidationReceipt::new(
            "explicit-planet-semimajor-axis-derivation",
            "1",
            semimajor_inputs,
            vec![ConstraintEvaluation::passed(
                "positive-semimajor-axis",
                Some(candidate.semimajor_axis_au),
                Some(0.0),
                Some(candidate.semimajor_axis_au),
                None::<String>,
            )?],
        )?,
    ));

    let value = StellarClaimValue::ExplicitPlanet(ExplicitPlanetClaim {
        host_member_id: candidate.host_member_id,
        orbital_parent_member_id: candidate.host_member_id,
        source_channel: candidate.source_channel,
        source_cell_index: candidate.source_cell_index,
        source_cell_count: candidate.source_cell_count,
        transit_radius_rearth,
        doppler_minimum_mass_mjup,
        period_days: candidate.period_days,
        semimajor_axis_au: candidate.semimajor_axis_au,
    });
    let mut candidate_derivation_inputs = vec![
        source_claim_id.clone(),
        property_claim_id,
        period_claim_id,
        semimajor_axis_claim_id,
    ];
    candidate_derivation_inputs.extend(cell_claim_id);
    let applicability = if matches!(
        rejection,
        Some(RejectedPlanetCandidateReason::StabilityZoneUnavailable(_))
    ) {
        ClaimApplicability::outside_domain(
            "explicit planet property domain plus available S-type screen",
            BTreeMap::from([("semimajor_axis_au".into(), candidate.semimajor_axis_au)]),
        )?
    } else {
        ClaimApplicability::inside_domain(
            "explicit planet observational domain",
            BTreeMap::from([
                ("period_days".into(), candidate.period_days),
                ("semimajor_axis_au".into(), candidate.semimajor_axis_au),
            ]),
        )?
    };
    let provenance = ClaimProvenance::new(
        object_id.clone(),
        "explicit_planet_candidate",
        EvidenceLevel::PhysicalProxy,
        EXPLICIT_PLANET_PRESCRIPTION,
        vec![],
        applicability,
        uncertainty(
            None,
            "addressed property draws and omitted planet interactions are explicit model limitations",
            model_realization_id,
        )?,
        Some(ClaimDerivation::new(candidate_derivation_inputs.clone())?),
        None,
    )?;
    let mut receipt_inputs = candidate_derivation_inputs;
    let constraints = match rejection {
        None => {
            if stability_zone.is_ok() {
                receipt_inputs.push(member_claim_id(
                    system_id,
                    candidate.host_member_id,
                    "circumstellar_s_type_stability_zone",
                ));
            }
            vec![
                candidate_stability_constraint(candidate, stability_zone, true)?,
                ConstraintEvaluation::not_evaluated(
                    "mutual-planet-and-long-term-stability",
                    ConstraintClass::Advisory,
                    "the current generator does not model planet-planet interactions or long-term N-body evolution",
                )?,
            ]
        }
        Some(reason) => {
            if matches!(
                reason,
                RejectedPlanetCandidateReason::OutsideCircumstellarStabilityZone { .. }
            ) && stability_zone.is_ok()
            {
                receipt_inputs.push(member_claim_id(
                    system_id,
                    candidate.host_member_id,
                    "circumstellar_s_type_stability_zone",
                ));
            }
            vec![rejected_candidate_constraint(reason)?]
        }
    };
    let receipt = ValidationReceipt::new(
        "explicit-planet-current-stability-screen",
        "1",
        receipt_inputs,
        constraints,
    )?;
    let candidate_claim_id = ClaimId::from(format!("{object_id}/explicit_planet_candidate"));
    let claim = ScientificClaim::new(candidate_claim_id.clone(), value, provenance)?;
    outcomes.push(if rejection.is_some() {
        ClaimOutcome::Rejected(claim, receipt)
    } else {
        ClaimOutcome::Accepted(claim, receipt)
    });
    for (index, flag) in candidate.quality_flags.iter().enumerate() {
        quality_flag_outcome(
            object_id.clone(),
            &format!("explicit_planet_quality_{index}"),
            StellarClaimValue::ExplicitPlanetQualityFlag(*flag),
            EXPLICIT_PLANET_PRESCRIPTION,
            vec![],
            Some(candidate_claim_id.clone()),
            model_realization_id,
            outcomes,
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn planet_property_outcome(
    object_id: ObjectId,
    claim_key: &str,
    value: StellarClaimValue,
    prescription: &str,
    namespace: &str,
    source_references: Vec<ScientificSourceReference>,
    source_claim_id: ClaimId,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<ClaimId, ProvenanceError> {
    let claim_id = ClaimId::from(format!("{object_id}/{claim_key}"));
    let address = RandomDrawAddress::new(
        "blake3-seeded-chacha8-indexed",
        "1",
        namespace,
        object_id.clone(),
        claim_key,
        0,
    )?;
    outcomes.push(ClaimOutcome::Accepted(
        ScientificClaim::new(
            claim_id.clone(),
            value,
            ClaimProvenance::new(
                object_id,
                claim_key,
                EvidenceLevel::PhysicalProxy,
                prescription,
                source_references,
                ClaimApplicability::inside_domain(
                    "selected explicit-planet observational domain",
                    BTreeMap::new(),
                )?,
                uncertainty(
                    Some("seed-deterministic within-domain property draw"),
                    "the configured conditional distribution retains its declared source limitations",
                    model_realization_id,
                )?,
                Some(ClaimDerivation::new(vec![source_claim_id.clone()])?),
                Some(address),
            )?,
        )?,
        ValidationReceipt::new(
            "explicit-planet-property-generation",
            "1",
            vec![source_claim_id],
            vec![ConstraintEvaluation::passed(
                "generated-property-inside-configured-domain",
                None,
                None,
                None,
                Some("the generated property passed its configured domain checks"),
            )?],
        )?,
    ));
    Ok(claim_id)
}

fn explicit_property_prescriptions(
    channel: ExplicitPlanetSourceChannel,
) -> ((&'static str, &'static str), (&'static str, &'static str)) {
    match channel {
        ExplicitPlanetSourceChannel::FgkWarmSuperEarth
        | ExplicitPlanetSourceChannel::FgkWarmSubNeptune => (
            (
                FGK_SMALL_RADIUS_PRESCRIPTION,
                "explicit_planet/small_radius/v1",
            ),
            (
                FGK_SMALL_PERIOD_PRESCRIPTION,
                "explicit_planet/small_period/v1",
            ),
        ),
        ExplicitPlanetSourceChannel::MDwarfSmallPlanet => (
            (
                M_DWARF_RADIUS_PRESCRIPTION,
                "explicit_planet/m_dwarf_radius/v1",
            ),
            (
                M_DWARF_PERIOD_PRESCRIPTION,
                "explicit_planet/m_dwarf_period/v1",
            ),
        ),
        ExplicitPlanetSourceChannel::MDwarfSubEarth => (
            (
                M_DWARF_SUB_EARTH_RADIUS_PRESCRIPTION,
                "explicit_planet/m_dwarf_sub_earth_radius/v1",
            ),
            (
                M_DWARF_SUB_EARTH_PERIOD_PRESCRIPTION,
                "explicit_planet/m_dwarf_sub_earth_period/v1",
            ),
        ),
        ExplicitPlanetSourceChannel::FgkDopplerGiant => (
            (
                GIANT_MASS_PRESCRIPTION,
                "explicit_planet/giant_minimum_mass/v1",
            ),
            (GIANT_PERIOD_PRESCRIPTION, "explicit_planet/giant_period/v1"),
        ),
    }
}

fn explicit_property_references(
    channel: ExplicitPlanetSourceChannel,
) -> Vec<ScientificSourceReference> {
    match channel {
        ExplicitPlanetSourceChannel::FgkWarmSuperEarth
        | ExplicitPlanetSourceChannel::FgkWarmSubNeptune => vec![reference(
            FGK_OCCURRENCE_SOURCE,
            "warm FGK radius-period occurrence domain",
        )],
        ExplicitPlanetSourceChannel::MDwarfSmallPlanet
        | ExplicitPlanetSourceChannel::MDwarfSubEarth => vec![reference(
            M_DWARF_OCCURRENCE_SOURCE,
            "measured M-dwarf radius-period cells",
        )],
        ExplicitPlanetSourceChannel::FgkDopplerGiant => vec![reference(
            GIANT_PROPERTIES_SOURCE,
            "Doppler minimum-mass and period density",
        )],
    }
}

#[allow(clippy::too_many_arguments)]
fn quality_flag_outcome(
    object_id: ObjectId,
    claim_key: &str,
    value: StellarClaimValue,
    prescription: &str,
    source_references: Vec<ScientificSourceReference>,
    input_claim_id: Option<ClaimId>,
    model_realization_id: &ModelRealizationId,
    outcomes: &mut Vec<ClaimOutcome<StellarClaimValue>>,
) -> Result<(), ProvenanceError> {
    let derivation = input_claim_id
        .clone()
        .map(|claim_id| ClaimDerivation::new(vec![claim_id]))
        .transpose()?;
    let receipt_inputs = input_claim_id.into_iter().collect();
    outcomes.push(ClaimOutcome::Accepted(
        ScientificClaim::new(
            ClaimId::from(format!("{object_id}/{claim_key}")),
            value,
            ClaimProvenance::new(
                object_id,
                claim_key,
                EvidenceLevel::PhysicalProxy,
                prescription,
                source_references,
                ClaimApplicability::inside_domain(
                    "declared generator limitation or proxy metadata",
                    BTreeMap::new(),
                )?,
                uncertainty(
                    None,
                    "the quality flag is categorical and has no quantified uncertainty",
                    model_realization_id,
                )?,
                derivation,
                None,
            )?,
        )?,
        ValidationReceipt::new(
            "quality-metadata-retention",
            "1",
            receipt_inputs,
            vec![ConstraintEvaluation::passed(
                "quality-flag-retained",
                None,
                None,
                None,
                Some("the generator quality flag is retained without changing acceptance"),
            )?],
        )?,
    ));
    Ok(())
}

fn candidate_stability_constraint(
    candidate: &ExplicitPlanetCandidate,
    stability_zone: &Result<CircumstellarSTypeStabilityZone, PlanetaryStabilityError>,
    passed: bool,
) -> Result<ConstraintEvaluation, ProvenanceError> {
    match stability_zone {
        Ok(CircumstellarSTypeStabilityZone::UnboundedByStellarCompanion { .. }) => {
            ConstraintEvaluation::passed(
                "inside-current-circumstellar-stability-zone",
                Some(candidate.semimajor_axis_au),
                None,
                None,
                Some("no stellar companion bounds the current S-type model"),
            )
        }
        Ok(CircumstellarSTypeStabilityZone::CompanionLimited {
            fit_residual_lower_semimajor_axis_au,
            ..
        }) if passed => ConstraintEvaluation::passed(
            "inside-current-circumstellar-stability-zone",
            Some(candidate.semimajor_axis_au),
            Some(*fit_residual_lower_semimajor_axis_au),
            Some(*fit_residual_lower_semimajor_axis_au - candidate.semimajor_axis_au),
            None::<String>,
        ),
        _ => unreachable!("accepted candidates always passed the current screen"),
    }
}

fn rejected_candidate_constraint(
    reason: &RejectedPlanetCandidateReason,
) -> Result<ConstraintEvaluation, ProvenanceError> {
    match reason {
        RejectedPlanetCandidateReason::OutsideCircumstellarStabilityZone {
            semimajor_axis_au,
            conservative_outer_limit_au,
        } => ConstraintEvaluation::failed(
            "inside-current-circumstellar-stability-zone",
            Some(*semimajor_axis_au),
            Some(*conservative_outer_limit_au),
            Some(*conservative_outer_limit_au - *semimajor_axis_au),
            Some("the immutable candidate lies outside the current conservative boundary"),
        ),
        RejectedPlanetCandidateReason::StabilityZoneUnavailable(error) => {
            ConstraintEvaluation::failed(
                "circumstellar-stability-zone-available",
                None,
                None,
                None,
                Some(error.to_string()),
            )
        }
    }
}

fn limiting_relationship(
    system: &StellarCatalogSystem,
    limiting_orbit: &RelativeStellarOrbit,
) -> Option<RelativeStellarOrbitClaim> {
    let hierarchy = system.orbital_hierarchy.as_ref().ok()?;
    let (_, relationships) =
        orbital_relationships(system.id, hierarchy.sampling_attempt, &hierarchy.root);
    relationships
        .into_iter()
        .find(|relationship| relationship.orbit == *limiting_orbit)
}

fn orbital_relationships(
    system_id: u64,
    sampling_attempt: Option<u16>,
    root: &StellarOrbitNode,
) -> (OrbitalNodeClaim, Vec<RelativeStellarOrbitClaim>) {
    fn visit(
        system_id: u64,
        sampling_attempt: u16,
        node: &StellarOrbitNode,
    ) -> (OrbitalNodeClaim, Vec<RelativeStellarOrbitClaim>, Vec<u64>) {
        match node {
            StellarOrbitNode::Member { member_id, .. } => (
                OrbitalNodeClaim {
                    stable_id: *member_id,
                    kind: OrbitalNodeClaimKind::StellarMember,
                },
                vec![],
                vec![*member_id],
            ),
            StellarOrbitNode::RelativeOrbit {
                orbit,
                sampling_slot,
                left,
                right,
            } => {
                let (left_claim, left_relationships, mut member_ids) =
                    visit(system_id, sampling_attempt, left);
                let (right_claim, right_relationships, right_member_ids) =
                    visit(system_id, sampling_attempt, right);
                member_ids.extend(right_member_ids);
                member_ids.sort_unstable();
                let barycentre = OrbitalNodeClaim {
                    stable_id: stable_barycentre_id(system_id, &member_ids),
                    kind: OrbitalNodeClaimKind::Barycentre,
                };
                let relationship = RelativeStellarOrbitClaim {
                    barycentre,
                    left_child: left_claim,
                    right_child: right_claim,
                    orbit: *orbit,
                    sampling_attempt,
                    sampling_slot: *sampling_slot,
                };
                let mut relationships =
                    Vec::with_capacity(1 + left_relationships.len() + right_relationships.len());
                relationships.push(relationship);
                relationships.extend(left_relationships);
                relationships.extend(right_relationships);
                (barycentre, relationships, member_ids)
            }
        }
    }

    let Some(sampling_attempt) = sampling_attempt else {
        return match root {
            StellarOrbitNode::Member { member_id, .. } => (
                OrbitalNodeClaim {
                    stable_id: *member_id,
                    kind: OrbitalNodeClaimKind::StellarMember,
                },
                vec![],
            ),
            StellarOrbitNode::RelativeOrbit { .. } => {
                unreachable!("relative-orbit hierarchy retains its accepted sampling attempt")
            }
        };
    };
    let (root, relationships, _) = visit(system_id, sampling_attempt, root);
    (root, relationships)
}

fn orbit_object_id(system_id: u64, relationship: &RelativeStellarOrbitClaim) -> ObjectId {
    let draw_id = stable_orbit_draw_id(
        system_id,
        relationship.sampling_attempt,
        relationship.sampling_slot,
    );
    ObjectId::from(format!(
        "indexed-u64-le:{draw_id:016x}/stellar-system-owner:{system_id:016x}/barycentre:{:016x}",
        relationship.barycentre.stable_id
    ))
}

fn source(
    id: &str,
    title: &str,
    year: u16,
    doi: &str,
    url: &str,
) -> Result<ScientificSource, ProvenanceError> {
    let mut source = ScientificSource::new(id, title)?;
    source.publication_year = Some(year);
    source.doi = Some(doi.into());
    source.url = Some(url.into());
    source.validate()?;
    Ok(source)
}

fn reference(source_id: &str, locator: &str) -> ScientificSourceReference {
    ScientificSourceReference {
        source_id: SourceId::from(source_id),
        locator: Some(locator.into()),
    }
}

fn orbit_references() -> Vec<ScientificSourceReference> {
    vec![
        reference(ORBIT_SOURCE_M_DWARF, "normalized M-dwarf separation shape"),
        reference(
            ORBIT_SOURCE_SOLAR,
            "solar-type period and eccentricity distributions",
        ),
    ]
}

fn stability_quality_references(
    flag: CircumstellarStabilityQualityFlag,
) -> Vec<ScientificSourceReference> {
    match flag {
        CircumstellarStabilityQualityFlag::SiblingSubtreePointMassApproximation
        | CircumstellarStabilityQualityFlag::HierarchicalMultipleNearestEdgeOnly
        | CircumstellarStabilityQualityFlag::ApproximateAdditionalPerturbersNotIntegrated => {
            vec![reference(
                MULTIPLE_STABILITY_SOURCE,
                "hierarchical sibling-subtree point-mass approximation",
            )]
        }
        CircumstellarStabilityQualityFlag::MasslessTestParticleApproximation
        | CircumstellarStabilityQualityFlag::CircularCoplanarProgradePlanetAssumption
        | CircumstellarStabilityQualityFlag::TenThousandBinaryPeriodIntegration => vec![reference(
            STABILITY_SOURCE,
            "elliptic restricted three-body S-type assumptions",
        )],
    }
}

fn source_channel_reference(channel: ExplicitPlanetSourceChannel) -> ScientificSourceReference {
    match channel {
        ExplicitPlanetSourceChannel::FgkWarmSuperEarth
        | ExplicitPlanetSourceChannel::FgkWarmSubNeptune => reference(
            FGK_OCCURRENCE_SOURCE,
            "warm FGK radius-period occurrence channel",
        ),
        ExplicitPlanetSourceChannel::MDwarfSmallPlanet
        | ExplicitPlanetSourceChannel::MDwarfSubEarth => reference(
            M_DWARF_OCCURRENCE_SOURCE,
            "early-M-dwarf radius-period occurrence channel",
        ),
        ExplicitPlanetSourceChannel::FgkDopplerGiant => reference(
            GIANT_OCCURRENCE_SOURCE,
            "CPS giant-planet host occurrence channel",
        ),
    }
}

fn uncertainty(
    aleatory_detail: Option<&str>,
    epistemic_detail: &str,
    model_realization_id: &ModelRealizationId,
) -> Result<ClaimUncertainty, ProvenanceError> {
    ClaimUncertainty::new(
        aleatory_detail
            .map(|detail| {
                AleatoryVariation::new(UncertaintyRepresentation::not_quantified(detail)?)
            })
            .transpose()?,
        Some(EpistemicUncertainty::new(
            UncertaintyRepresentation::not_quantified(epistemic_detail)?,
            Some(model_realization_id.clone()),
            None,
        )?),
    )
}

fn draw_address(
    object_id: ObjectId,
    claim_key: &str,
    namespace: &str,
) -> Result<RandomDrawAddress, ProvenanceError> {
    RandomDrawAddress::new(
        "blake3-seeded-chacha8-indexed",
        "1",
        namespace,
        object_id,
        claim_key,
        0,
    )
}

fn system_object_id(system_id: u64) -> ObjectId {
    ObjectId::from(format!("indexed-u64-le:{system_id:016x}/stellar-system"))
}

fn orbital_input_claim_ids(system: &StellarCatalogSystem) -> Vec<ClaimId> {
    system
        .orbital_member_inputs
        .iter()
        .map(|input| {
            let claim_key = match input.input_source {
                StellarOrbitMemberProvenance::CurrentMassAndRadiusFromEvolution => {
                    "current_stellar_mass_msolar"
                }
                StellarOrbitMemberProvenance::SingleMemberInitialMass
                | StellarOrbitMemberProvenance::LowMassContactRadiusProxy { .. } => {
                    "initial_stellar_mass_msolar"
                }
            };
            member_claim_id(system.id, input.member_id, claim_key)
        })
        .collect()
}

fn occurrence_object_id(system_id: u64, member_id: u64) -> ObjectId {
    let draw_id = stable_planet_host_id(system_id, member_id);
    ObjectId::from(format!(
        "indexed-u64-le:{draw_id:016x}/stellar-system-owner:{system_id:016x}/stellar-member-host:{member_id:016x}/planet-occurrence"
    ))
}

fn member_object_id(system_id: u64, member_id: u64) -> ObjectId {
    ObjectId::from(format!(
        "indexed-u64-le:{system_id:016x}/stellar-member:{member_id:016x}"
    ))
}

fn system_claim_id(system_id: u64, claim_key: &str) -> ClaimId {
    ClaimId::from(format!("{}/{claim_key}", system_object_id(system_id)))
}

fn member_claim_id(system_id: u64, member_id: u64, claim_key: &str) -> ClaimId {
    ClaimId::from(format!(
        "{}/{claim_key}",
        member_object_id(system_id, member_id)
    ))
}

fn orbital_unsupported_reason(
    error: &StellarOrbitalHierarchyError,
) -> Result<UnsupportedReason, ProvenanceError> {
    let code = match error {
        StellarOrbitalHierarchyError::InvalidModel => "stellar_orbits_invalid_model",
        StellarOrbitalHierarchyError::UnsupportedMemberCount => {
            "stellar_orbits_unsupported_member_count"
        }
        StellarOrbitalHierarchyError::MissingStellarEvolution => {
            "stellar_orbits_missing_stellar_evolution"
        }
        StellarOrbitalHierarchyError::OrbitalEvolutionNotModeled { .. } => {
            "stellar_orbits_orbital_evolution_not_modeled"
        }
        StellarOrbitalHierarchyError::MissingStellarRadius => {
            "stellar_orbits_missing_stellar_radius"
        }
        StellarOrbitalHierarchyError::OutsideOrbitalScaleCalibration { .. } => {
            "stellar_orbits_outside_scale_calibration"
        }
        StellarOrbitalHierarchyError::OutsideEccentricityCalibration => {
            "stellar_orbits_outside_eccentricity_calibration"
        }
        StellarOrbitalHierarchyError::StableHierarchySamplingExhausted => {
            "stellar_orbits_sampling_exhausted"
        }
    };
    UnsupportedReason::new(code, error.to_string())
}

fn stability_unsupported_reason(
    error: &PlanetaryStabilityError,
) -> Result<UnsupportedReason, ProvenanceError> {
    let code = match error {
        PlanetaryStabilityError::InvalidModel => "planetary_stability_invalid_model",
        PlanetaryStabilityError::MissingStellarHierarchy => {
            "planetary_stability_missing_stellar_hierarchy"
        }
        PlanetaryStabilityError::MissingStellarMember => {
            "planetary_stability_missing_stellar_member"
        }
        PlanetaryStabilityError::OutsideMassRatioCalibration { .. } => {
            "planetary_stability_outside_mass_ratio_calibration"
        }
        PlanetaryStabilityError::OutsideEccentricityCalibration { .. } => {
            "planetary_stability_outside_eccentricity_calibration"
        }
        PlanetaryStabilityError::NonPositiveCriticalSemimajorAxis => {
            "planetary_stability_non_positive_boundary"
        }
    };
    UnsupportedReason::new(code, error.to_string())
}

fn occurrence_unsupported_reason(
    error: &PlanetOccurrenceError,
) -> Result<UnsupportedReason, ProvenanceError> {
    let code = match error {
        PlanetOccurrenceError::InvalidModel => "planet_occurrence_invalid_model",
        PlanetOccurrenceError::MissingStellarEvolution => {
            "planet_occurrence_missing_stellar_evolution"
        }
        PlanetOccurrenceError::UnsupportedEvolutionaryState { .. } => {
            "planet_occurrence_unsupported_evolutionary_state"
        }
        PlanetOccurrenceError::MissingStellarObservable { .. } => {
            "planet_occurrence_missing_stellar_observable"
        }
        PlanetOccurrenceError::OutsideHostCalibration => {
            "planet_occurrence_outside_host_calibration"
        }
        PlanetOccurrenceError::OutsideMetallicityCalibration => {
            "planet_occurrence_outside_metallicity_calibration"
        }
        PlanetOccurrenceError::MultiplicitySeparationRequired => {
            "planet_occurrence_multiplicity_separation_required"
        }
    };
    UnsupportedReason::new(code, error.to_string())
}
