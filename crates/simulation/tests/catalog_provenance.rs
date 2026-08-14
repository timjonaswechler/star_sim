use simulation::{
    ClaimApplicability, ClaimOutcome, DeterministicDraws, ExplicitPlanetModel,
    GalacticLocationSampler, GalacticSamplingVolume, GalaxyModel, PlanetOccurrenceModel,
    PlanetaryStabilityModel, PopulationHistoryModel, ProvenanceDocument, SourceId,
    StellarBirthMassModel, StellarCatalogGenerator, StellarClaimValue, StellarEvolutionModel,
    StellarOrbitalHierarchyModel, WhiteDwarfCoolingModel, WhiteDwarfCoolingModelVersion,
    WhiteDwarfCoolingPoint, WhiteDwarfCoolingSequence,
};

fn generator_and_location(
    seed: u64,
) -> (
    simulation::StellarCatalogGenerator,
    simulation::SampledGalacticLocation,
) {
    let galaxy: GalaxyModel = ron::from_str(include_str!(
        "../../../assets/scientific_models/milky_way.ron"
    ))
    .expect("bundled galaxy model loads");
    let location = GalacticLocationSampler::new(galaxy, GalacticSamplingVolume::default())
        .expect("galactic sampler is valid")
        .sample(seed);
    let birth_mass_model: StellarBirthMassModel = ron::from_str(include_str!(
        "../../../assets/scientific_models/stellar_birth_masses.ron"
    ))
    .expect("bundled birth-mass model loads");
    let population_history_model: PopulationHistoryModel = ron::from_str(include_str!(
        "../../../assets/scientific_models/stellar_population_history.ron"
    ))
    .expect("bundled population-history model loads");
    let evolution_model: StellarEvolutionModel = ron::from_str(include_str!(
        "../../../assets/scientific_models/stellar_evolution.ron"
    ))
    .expect("bundled evolution model loads");
    let generator = StellarCatalogGenerator::new(
        birth_mass_model,
        population_history_model,
        evolution_model,
        PlanetOccurrenceModel::default(),
        StellarOrbitalHierarchyModel::default(),
        PlanetaryStabilityModel::default(),
        ExplicitPlanetModel::default(),
    )
    .expect("catalog generator is valid");
    (generator, location)
}

#[test]
fn catalog_generation_exposes_valid_initial_mass_provenance() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);

    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog with provenance generates");

    generated
        .provenance
        .validate()
        .expect("generated provenance is valid");
    assert_eq!(generated.provenance.catalog.simulation_seed, seed);

    let initial_masses: Vec<_> = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| {
            system
                .members
                .iter()
                .map(|member| member.birth.initial_mass_msun)
        })
        .collect();
    let claimed_initial_masses: Vec<_> = generated
        .provenance
        .outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => match claim.value {
                StellarClaimValue::InitialStellarMassMsolar(value) => Some(value),
                _ => None,
            },
            ClaimOutcome::NotSelected(_, _)
            | ClaimOutcome::Rejected(_, _)
            | ClaimOutcome::Unsupported(_, _) => None,
        })
        .collect();

    assert!(!initial_masses.is_empty());
    assert_eq!(claimed_initial_masses, initial_masses);
}

#[test]
fn every_stellar_member_exposes_birth_mass_and_role_claims() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog with provenance generates");

    let expected_member_count = generated
        .catalog
        .systems
        .iter()
        .map(|system| system.members.len())
        .sum::<usize>();
    let initial_mass_claims = generated
        .provenance
        .outcomes
        .iter()
        .filter(|outcome| {
            matches!(
                outcome.claim().map(|claim| claim.value),
                Some(StellarClaimValue::InitialStellarMassMsolar(_))
            )
        })
        .count();
    let member_count_claims = generated
        .provenance
        .outcomes
        .iter()
        .filter(|outcome| {
            matches!(
                outcome.claim().map(|claim| claim.value),
                Some(StellarClaimValue::StellarMemberCount(_))
            )
        })
        .count();
    let role_claims = generated
        .provenance
        .outcomes
        .iter()
        .filter(|outcome| {
            matches!(
                outcome.claim().map(|claim| claim.value),
                Some(StellarClaimValue::MemberRole(_))
            )
        })
        .count();

    assert!(
        generated
            .catalog
            .systems
            .iter()
            .any(|system| system.members.len() > 1),
        "fixture seed must exercise companion provenance"
    );
    assert_eq!(initial_mass_claims, expected_member_count);
    assert_eq!(member_count_claims, generated.catalog.systems.len());
    assert_eq!(role_claims, expected_member_count);
}

#[test]
fn every_stellar_system_exposes_population_history_and_chemistry_claims() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog with provenance generates");
    let system_count = generated.catalog.systems.len();

    for predicate in [
        |value: &StellarClaimValue| matches!(value, StellarClaimValue::Population(_)),
        |value: &StellarClaimValue| matches!(value, StellarClaimValue::StellarAgeGyr(_)),
        |value: &StellarClaimValue| matches!(value, StellarClaimValue::IronAbundanceFeH(_)),
        |value: &StellarClaimValue| matches!(value, StellarClaimValue::AlphaEnhancementAlphaFe(_)),
        |value: &StellarClaimValue| matches!(value, StellarClaimValue::StellarChemistry(_)),
    ] {
        assert_eq!(
            generated
                .provenance
                .outcomes
                .iter()
                .filter_map(ClaimOutcome::claim)
                .filter(|claim| predicate(&claim.value))
                .count(),
            system_count,
        );
    }
}

#[test]
fn every_stellar_member_exposes_an_evolutionary_state_or_typed_unsupported_outcome() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog with provenance generates");
    let expected_accepted = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .filter(|member| member.evolution.is_ok())
        .count();
    let expected_unsupported = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .filter(|member| member.evolution.is_err())
        .count();

    let accepted = generated
        .provenance
        .outcomes
        .iter()
        .filter(|outcome| {
            matches!(
                outcome.claim().map(|claim| claim.value),
                Some(StellarClaimValue::EvolutionaryState(_))
            )
        })
        .count();
    let unsupported: Vec<_> = generated
        .provenance
        .outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Unsupported(provenance, reasons)
                if provenance.claim_key == "evolutionary_state" =>
            {
                Some(reasons)
            }
            _ => None,
        })
        .collect();

    assert_eq!(accepted, expected_accepted);
    assert!(
        expected_unsupported > 0,
        "fixture seed exercises unsupported coverage"
    );
    assert_eq!(unsupported.len(), expected_unsupported);
    assert!(generated.provenance.outcomes.iter().all(|outcome| {
        !matches!(outcome, ClaimOutcome::Unsupported(_, _))
            || matches!(
                outcome.provenance().applicability,
                ClaimApplicability::OutsideDomain { .. }
            )
    }));
    assert!(unsupported.iter().all(|reasons| {
        reasons.len() == 1 && reasons[0].code.starts_with("stellar_evolution_")
    }));
}

#[test]
fn same_seed_reproduces_catalog_claim_identities_and_serialized_provenance() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);

    let first = generator
        .generate_with_provenance(seed, location)
        .expect("first catalog generates");
    let repeated = generator
        .generate_with_provenance(seed, location)
        .expect("repeated catalog generates");

    assert_eq!(repeated, first);
    assert_eq!(
        ron::to_string(&repeated.provenance).expect("repeated provenance serializes"),
        ron::to_string(&first.provenance).expect("first provenance serializes"),
    );
    assert!(first.provenance.outcomes.iter().all(|outcome| {
        let provenance = outcome.provenance();
        provenance.uncertainty.aleatory_variation.is_none()
            || provenance.random_draw_address.is_some()
    }));
}

fn primary_mass_from_uniforms(segment_draw: f64, mass_draw: f64) -> f64 {
    fn integral(minimum: f64, maximum: f64, exponent: f64) -> f64 {
        (maximum.powf(1.0 - exponent) - minimum.powf(1.0 - exponent)) / (1.0 - exponent)
    }
    fn inverse(minimum: f64, maximum: f64, exponent: f64, draw: f64) -> f64 {
        let power = 1.0 - exponent;
        (minimum.powf(power) + draw * (maximum.powf(power) - minimum.powf(power))).powf(1.0 / power)
    }

    let low_weight = integral(0.08, 0.5, 1.3);
    let high_weight = 0.5 * integral(0.5, 100.0, 2.3);
    let weighted_draw = segment_draw * (low_weight + high_weight);
    if weighted_draw < low_weight {
        inverse(0.08, 0.5, 1.3, mass_draw)
    } else {
        inverse(0.5, 100.0, 2.3, mass_draw)
    }
}

#[test]
fn primary_mass_claim_can_be_replayed_from_its_draw_address() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates");
    let claim = generated
        .provenance
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .find(|claim| {
            claim.provenance.claim_key == "initial_stellar_mass_msolar"
                && claim.provenance.generating_prescription.as_str()
                    == "prescription.stellar-birth-primary-mass-proxy-v1"
        })
        .expect("primary mass claim is accepted");
    let address = claim
        .provenance
        .random_draw_address
        .as_ref()
        .expect("stochastic claim retains its draw address");
    let StellarClaimValue::InitialStellarMassMsolar(actual_mass) = claim.value else {
        panic!("first catalog claim is the primary initial mass");
    };

    let draws = DeterministicDraws::new(seed);
    let segment_draw = draws.uniform(address);
    let mut conditional_mass_address = address.clone();
    conditional_mass_address.bounded_attempt_index = 1;
    let mass_draw = draws.uniform(&conditional_mass_address);
    let replayed_mass = primary_mass_from_uniforms(segment_draw, mass_draw);

    assert!((replayed_mass - actual_mass).abs() < 1e-12);
}

#[test]
fn stellar_population_claims_can_be_replayed_from_their_draw_addresses() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let density = location.local_density;
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates");

    for system in &generated.catalog.systems {
        let object_id = format!("indexed-u64-le:{:016x}/stellar-system", system.id);
        let claim = generated
            .provenance
            .outcomes
            .iter()
            .filter_map(ClaimOutcome::claim)
            .find(|claim| {
                claim.provenance.object_id.as_str() == object_id
                    && claim.provenance.claim_key == "stellar_population"
            })
            .expect("Stellar Population claim exists");
        let address = claim
            .provenance
            .random_draw_address
            .as_ref()
            .expect("sampled population retains its draw address");
        let draw = DeterministicDraws::new(seed).uniform(address) * density.total();
        let replayed = if draw < density.thin_disk {
            simulation::StellarPopulation::ThinDisk
        } else if draw < density.thin_disk + density.thick_disk {
            simulation::StellarPopulation::ThickDisk
        } else {
            simulation::StellarPopulation::Halo
        };

        assert_eq!(replayed, system.population);
    }
}

#[test]
fn companion_mass_ratio_claim_can_be_replayed_from_its_draw_address() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates");
    let birth_mass_model: StellarBirthMassModel = ron::from_str(include_str!(
        "../../../assets/scientific_models/stellar_birth_masses.ron"
    ))
    .expect("bundled birth-mass model loads");
    let mut replayed_companions = 0;
    for system in &generated.catalog.systems {
        let primary_mass = system.members[0].birth.initial_mass_msun;
        let bin = birth_mass_model
            .multiplicity_bins
            .iter()
            .find(|bin| primary_mass <= bin.maximum_primary_mass_msun)
            .expect("model covers primary mass");
        let minimum_ratio = bin
            .minimum_mass_ratio
            .max(birth_mass_model.minimum_companion_mass_msun / primary_mass);
        let power = 1.0 + bin.mass_ratio_power;

        for companion in system.members.iter().skip(1) {
            let object_id = format!(
                "indexed-u64-le:{:016x}/stellar-member:{:016x}",
                system.id, companion.birth.id
            );
            let claim = generated
                .provenance
                .outcomes
                .iter()
                .filter_map(ClaimOutcome::claim)
                .find(|claim| {
                    claim.provenance.object_id.as_str() == object_id
                        && claim.provenance.claim_key == "companion_mass_ratio"
                })
                .expect("companion mass-ratio claim exists");
            let StellarClaimValue::CompanionMassRatio(actual_ratio) = claim.value else {
                panic!("claim contains a companion mass ratio");
            };
            let address = claim
                .provenance
                .random_draw_address
                .as_ref()
                .expect("sampled mass ratio retains its draw address");
            let draw = DeterministicDraws::new(seed).uniform(address);
            let replayed_ratio = (minimum_ratio.powf(power)
                + draw * (1.0_f64.powf(power) - minimum_ratio.powf(power)))
            .powf(1.0 / power);

            assert!((replayed_ratio - actual_ratio).abs() < 1e-12);
            replayed_companions += 1;
        }
    }
    assert!(replayed_companions > 0, "fixture seed generates companions");
}

#[test]
fn optional_white_dwarf_cooling_claims_use_the_cooling_prescription_and_realization() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let cooling_point = |cooling_age_gyr, luminosity_lsun| WhiteDwarfCoolingPoint {
        cooling_age_gyr,
        luminosity_lsun,
        radius_rsun: 0.012,
        effective_temperature_k: 10_000.0,
        surface_gravity_log10_cgs: 8.0,
    };
    let cooling_model = WhiteDwarfCoolingModel {
        model_version: WhiteDwarfCoolingModelVersion::MontrealBedard2020ThickHydrogenV1,
        sequences: vec![
            WhiteDwarfCoolingSequence {
                mass_msun: 0.45,
                points: vec![cooling_point(0.0, 1.0), cooling_point(20.0, 1.0e-5)],
            },
            WhiteDwarfCoolingSequence {
                mass_msun: 1.10,
                points: vec![cooling_point(0.0, 1.0), cooling_point(20.0, 1.0e-5)],
            },
        ],
    };
    let generator = generator
        .with_white_dwarf_cooling(cooling_model)
        .expect("cooling model is valid");
    for generation_seed in 0..32 {
        let generated = generator
            .generate_with_provenance(generation_seed, location)
            .expect("catalog with cooling provenance generates");
        let cooling_claims: Vec<_> = generated
            .provenance
            .outcomes
            .iter()
            .filter_map(ClaimOutcome::claim)
            .filter(|claim| {
                claim.provenance.generating_prescription.as_str()
                    == "prescription.white-dwarf-cooling-montreal-bedard-2020-thick-h-v1"
                    && matches!(
                        claim.provenance.claim_key.as_str(),
                        "luminosity_lsolar"
                            | "radius_rsolar"
                            | "effective_temperature_k"
                            | "surface_gravity_log10_cgs"
                    )
            })
            .collect();
        if cooling_claims.is_empty() {
            continue;
        }

        assert!(cooling_claims.iter().all(|claim| {
            !claim.provenance.source_references.is_empty()
                && claim
                    .provenance
                    .uncertainty
                    .epistemic_uncertainty
                    .as_ref()
                    .and_then(|uncertainty| uncertainty.model_realization_id.as_ref())
                    .is_some()
        }));
        return;
    }
    panic!("fixture seed range produces no cooled white dwarfs");
}

#[test]
fn catalog_provenance_round_trips_through_ron() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates");

    let encoded = ron::to_string(&generated.provenance).expect("provenance serializes");
    let decoded: ProvenanceDocument<StellarClaimValue> =
        ron::from_str(&encoded).expect("provenance deserializes");

    assert_eq!(decoded, generated.provenance);
}

#[test]
fn catalog_provenance_rejects_dangling_source_references_on_deserialization() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let mut document = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates")
        .provenance;
    let claim = document
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) | ClaimOutcome::Rejected(claim, _) => Some(claim),
            ClaimOutcome::NotSelected(_, _) | ClaimOutcome::Unsupported(_, _) => None,
        })
        .find(|claim| !claim.provenance.source_references.is_empty())
        .expect("at least one generated claim references a source");
    claim.provenance.source_references[0].source_id = SourceId::from("source.missing");

    let encoded = ron::to_string(&document).expect("invalid document still serializes");
    let decoded = ron::from_str::<ProvenanceDocument<StellarClaimValue>>(&encoded);

    assert!(decoded.is_err());
}

#[test]
fn existing_generate_result_is_unchanged_by_the_provenance_path() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);

    let existing = generator
        .generate(seed, location)
        .expect("existing catalog generation succeeds");
    let provenance_bearing = generator
        .generate_with_provenance(seed, location)
        .expect("provenance-bearing generation succeeds");

    assert_eq!(provenance_bearing.catalog, existing);
}
