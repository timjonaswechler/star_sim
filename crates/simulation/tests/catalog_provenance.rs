use simulation::{
    ClaimApplicability, ClaimOutcome, DeterministicDraws, EvidenceLevel, ExplicitPlanetModel,
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
    generator_and_location_with_stability(seed, PlanetaryStabilityModel::default())
}

fn generator_and_location_with_stability(
    seed: u64,
    planetary_stability_model: PlanetaryStabilityModel,
) -> (
    simulation::StellarCatalogGenerator,
    simulation::SampledGalacticLocation,
) {
    generator_and_location_with_models(
        seed,
        StellarOrbitalHierarchyModel::default(),
        planetary_stability_model,
    )
}

fn generator_and_location_with_models(
    seed: u64,
    orbital_hierarchy_model: StellarOrbitalHierarchyModel,
    planetary_stability_model: PlanetaryStabilityModel,
) -> (
    simulation::StellarCatalogGenerator,
    simulation::SampledGalacticLocation,
) {
    generator_and_location_with_all_models(
        seed,
        orbital_hierarchy_model,
        planetary_stability_model,
        PlanetOccurrenceModel::default(),
    )
}

fn generator_and_location_with_all_models(
    seed: u64,
    orbital_hierarchy_model: StellarOrbitalHierarchyModel,
    planetary_stability_model: PlanetaryStabilityModel,
    planet_occurrence_model: PlanetOccurrenceModel,
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
        planet_occurrence_model,
        orbital_hierarchy_model,
        planetary_stability_model,
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
                outcome.claim().map(|claim| claim.value.clone()),
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
                outcome.claim().map(|claim| claim.value.clone()),
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
                outcome.claim().map(|claim| claim.value.clone()),
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
                outcome.claim().map(|claim| claim.value.clone()),
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
    let draws = DeterministicDraws::new(seed);
    for address in first
        .provenance
        .outcomes
        .iter()
        .filter_map(|outcome| outcome.provenance().random_draw_address.as_ref())
    {
        assert!(draws.uniform(address).is_finite());
    }
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
fn orbital_and_planetary_catalog_outputs_have_claim_level_outcomes() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog with orbital and planetary provenance generates");

    let system_count = generated.catalog.systems.len();
    let member_count = generated
        .catalog
        .systems
        .iter()
        .map(|system| system.members.len())
        .sum::<usize>();
    let relative_orbit_count = generated
        .catalog
        .systems
        .iter()
        .filter_map(|system| system.orbital_hierarchy.as_ref().ok())
        .map(|hierarchy| hierarchy.relative_orbits().len())
        .sum::<usize>();
    let count_key = |key: &str| {
        generated
            .provenance
            .outcomes
            .iter()
            .filter(|outcome| outcome.provenance().claim_key == key)
            .count()
    };

    assert_eq!(count_key("stellar_orbital_hierarchy"), system_count);
    assert_eq!(count_key("relative_stellar_orbit"), relative_orbit_count);
    assert_eq!(
        count_key("circumstellar_s_type_stability_zone"),
        member_count
    );
    assert!(
        generated
            .provenance
            .outcomes
            .iter()
            .filter(|outcome| outcome.provenance().claim_key.contains("planet_occurrence"))
            .count()
            >= member_count,
    );

    let expected_accepted_planets = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .map(|member| member.planetary_system.accepted_planets.len())
        .sum::<usize>();
    let expected_rejected_planets = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .map(|member| member.planetary_system.rejected_candidates.len())
        .sum::<usize>();
    let expected_unresolved_populations = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .map(|member| member.planetary_system.unresolved_populations.len())
        .sum::<usize>();
    let accepted_planets = generated
        .provenance
        .outcomes
        .iter()
        .filter(|outcome| {
            matches!(outcome, ClaimOutcome::Accepted(claim, _) if claim.provenance.claim_key == "explicit_planet_candidate")
        })
        .count();
    let rejected_planets = generated
        .provenance
        .outcomes
        .iter()
        .filter(|outcome| {
            matches!(outcome, ClaimOutcome::Rejected(claim, _) if claim.provenance.claim_key == "explicit_planet_candidate")
        })
        .count();
    let unresolved_populations = generated
        .provenance
        .outcomes
        .iter()
        .filter(|outcome| {
            matches!(outcome, ClaimOutcome::Unsupported(provenance, _) if provenance.claim_key == "unresolved_planet_population")
        })
        .count();

    assert_eq!(accepted_planets, expected_accepted_planets);
    assert_eq!(rejected_planets, expected_rejected_planets);
    assert_eq!(unresolved_populations, expected_unresolved_populations);
    assert_eq!(
        count_key("occurrence_source_channel"),
        expected_accepted_planets + expected_rejected_planets
    );
}

#[test]
fn orbital_and_stability_derivations_use_the_actual_mass_inputs() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates");
    let mut saw_current_mass_input = false;

    for system in &generated.catalog.systems {
        let expected = system
            .orbital_member_inputs
            .iter()
            .map(|input| {
                let key = match input.input_source {
                    simulation::StellarOrbitMemberProvenance::CurrentMassAndRadiusFromEvolution => {
                        saw_current_mass_input = true;
                        "current_stellar_mass_msolar"
                    }
                    simulation::StellarOrbitMemberProvenance::SingleMemberInitialMass
                    | simulation::StellarOrbitMemberProvenance::LowMassContactRadiusProxy {
                        ..
                    } => "initial_stellar_mass_msolar",
                };
                format!(
                    "indexed-u64-le:{:016x}/stellar-member:{:016x}/{key}",
                    system.id, input.member_id
                )
            })
            .collect::<Vec<_>>();
        if expected.is_empty() || system.orbital_hierarchy.is_err() {
            continue;
        }
        let hierarchy_object = format!("indexed-u64-le:{:016x}/stellar-system", system.id);
        let hierarchy = generated
            .provenance
            .outcomes
            .iter()
            .filter_map(ClaimOutcome::claim)
            .find(|claim| {
                claim.provenance.object_id.as_str() == hierarchy_object
                    && claim.provenance.claim_key == "stellar_orbital_hierarchy"
            })
            .expect("accepted hierarchy claim exists");
        let derivation = hierarchy
            .provenance
            .derivation
            .as_ref()
            .expect("hierarchy is derived");
        for expected_id in &expected {
            assert!(
                derivation
                    .input_claims
                    .iter()
                    .any(|claim_id| claim_id.as_str() == expected_id)
            );
        }
        let owner_path = format!("stellar-system-owner:{:016x}", system.id);
        for scale in generated
            .provenance
            .outcomes
            .iter()
            .filter_map(ClaimOutcome::claim)
            .filter(|claim| {
                claim.provenance.object_id.as_str().contains(&owner_path)
                    && claim.provenance.claim_key == "relative_stellar_orbit_scale"
            })
        {
            let inputs = &scale
                .provenance
                .derivation
                .as_ref()
                .expect("orbit scale is derived from mass inputs")
                .input_claims;
            for expected_id in &expected {
                assert!(
                    inputs
                        .iter()
                        .any(|claim_id| claim_id.as_str() == expected_id)
                );
            }
        }

        for member in &system.members {
            if member.circumstellar_stability_zone.is_err() {
                continue;
            }
            let member_object = format!(
                "indexed-u64-le:{:016x}/stellar-member:{:016x}",
                system.id, member.birth.id
            );
            let stability = generated
                .provenance
                .outcomes
                .iter()
                .filter_map(ClaimOutcome::claim)
                .find(|claim| {
                    claim.provenance.object_id.as_str() == member_object
                        && claim.provenance.claim_key == "circumstellar_s_type_stability_zone"
                })
                .expect("accepted stability claim exists");
            let inputs = &stability
                .provenance
                .derivation
                .as_ref()
                .expect("stability is derived")
                .input_claims;
            for expected_id in &expected {
                assert!(
                    inputs
                        .iter()
                        .any(|claim_id| claim_id.as_str() == expected_id)
                );
            }
        }
    }
    assert!(saw_current_mass_input);
}

#[test]
fn close_binary_suppression_is_an_explicit_proxy_input() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates");
    let suppression_claims = generated
        .provenance
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .filter(|claim| claim.provenance.claim_key == "close_binary_occurrence_factor")
        .collect::<Vec<_>>();
    assert!(!suppression_claims.is_empty());

    let suppressed_objects = suppression_claims
        .iter()
        .map(|claim| claim.provenance.object_id.clone())
        .collect::<std::collections::BTreeSet<_>>();
    for suppression in suppression_claims {
        let suppression_id = &suppression.id;
        let object_id = &suppression.provenance.object_id;
        let affected = generated.provenance.outcomes.iter().filter(|outcome| {
            !matches!(outcome, ClaimOutcome::Unsupported(_, _))
                && outcome.provenance().object_id == *object_id
                && matches!(
                    outcome.provenance().claim_key.as_str(),
                    "fgk_warm_super_earth_occurrence"
                        | "fgk_warm_sub_neptune_occurrence"
                        | "m_dwarf_small_planet_occurrence"
                        | "m_dwarf_sub_earth_occurrence"
                        | "giant_planet_occurrence"
                )
        });
        for outcome in affected {
            assert_eq!(
                outcome.provenance().evidence_level,
                EvidenceLevel::PhysicalProxy
            );
            assert!(
                outcome
                    .provenance()
                    .derivation
                    .as_ref()
                    .is_some_and(|derivation| derivation.input_claims.contains(suppression_id))
            );
        }
    }
    assert!(generated.provenance.outcomes.iter().any(|outcome| {
        !suppressed_objects.contains(&outcome.provenance().object_id)
            && outcome.provenance().claim_key.ends_with("occurrence")
            && !matches!(outcome, ClaimOutcome::Unsupported(_, _))
            && outcome.provenance().evidence_level == EvidenceLevel::Empirical
    }));
    assert!(generated.provenance.catalog.sources.iter().any(|source| {
        source.id.as_str() == "source.kraus-2016-close-binary-planet-suppression"
    }));
}

#[test]
fn quality_flags_and_m_dwarf_cell_draws_are_retained() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates");
    let expected_hierarchy_flags = generated
        .catalog
        .systems
        .iter()
        .filter_map(|system| system.orbital_hierarchy.as_ref().ok())
        .map(|hierarchy| hierarchy.quality_flags.len())
        .sum::<usize>();
    let expected_stability_flags = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .filter_map(|member| member.circumstellar_stability_zone.as_ref().ok())
        .map(|zone| match zone {
            simulation::CircumstellarSTypeStabilityZone::UnboundedByStellarCompanion { .. } => 0,
            simulation::CircumstellarSTypeStabilityZone::CompanionLimited {
                quality_flags, ..
            } => quality_flags.len(),
        })
        .sum::<usize>();
    let candidates = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .flat_map(|member| {
            member.planetary_system.accepted_planets.iter().chain(
                member
                    .planetary_system
                    .rejected_candidates
                    .iter()
                    .map(|rejected| &rejected.candidate),
            )
        })
        .collect::<Vec<_>>();
    let expected_occurrence_flags = generated
        .catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .map(|member| member.planet_population.quality_flags.len())
        .sum::<usize>();
    let expected_candidate_flags = candidates
        .iter()
        .map(|candidate| candidate.quality_flags.len())
        .sum::<usize>();
    let expected_cells = candidates
        .iter()
        .filter(|candidate| candidate.source_cell_index.is_some())
        .count();
    let count_value = |predicate: fn(&StellarClaimValue) -> bool| {
        generated
            .provenance
            .outcomes
            .iter()
            .filter_map(ClaimOutcome::claim)
            .filter(|claim| predicate(&claim.value))
            .count()
    };
    assert_eq!(
        count_value(|value| matches!(value, StellarClaimValue::StellarOrbitalQualityFlag(_))),
        expected_hierarchy_flags
    );
    assert_eq!(
        count_value(|value| matches!(
            value,
            StellarClaimValue::CircumstellarStabilityQualityFlag(_)
        )),
        expected_stability_flags
    );
    assert_eq!(
        count_value(|value| matches!(value, StellarClaimValue::PlanetOccurrenceQualityFlag(_))),
        expected_occurrence_flags
    );
    assert_eq!(
        count_value(|value| matches!(value, StellarClaimValue::ExplicitPlanetQualityFlag(_))),
        expected_candidate_flags
    );
    assert!(expected_cells > 0);
    assert_eq!(
        count_value(|value| matches!(
            value,
            StellarClaimValue::MDwarfOccurrenceCellSelection { .. }
        )),
        expected_cells
    );
    for claim in generated
        .provenance
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .filter(|claim| {
            matches!(
                claim.value,
                StellarClaimValue::MDwarfOccurrenceCellSelection { .. }
            )
        })
    {
        let address = claim
            .provenance
            .random_draw_address
            .as_ref()
            .expect("cell selection retains its draw address");
        assert!(matches!(
            address.prescription_namespace.as_str(),
            "explicit_planet/m_dwarf_cell/v1" | "explicit_planet/m_dwarf_sub_earth_cell/v1"
        ));
        assert!(DeterministicDraws::new(seed).uniform(address).is_finite());
    }
}

#[test]
fn exhausted_hierarchy_sampling_is_rejected_not_unsupported() {
    let mut orbit_model = StellarOrbitalHierarchyModel::default();
    orbit_model.stability.maximum_sampling_attempts = 1;
    orbit_model.stability.mardling_aarseth_coefficient = 1.0e100;
    for seed in 0..4 {
        let (generator, location) = generator_and_location_with_models(
            seed,
            orbit_model,
            PlanetaryStabilityModel::default(),
        );
        let generated = generator
            .generate_with_provenance(seed, location)
            .expect("catalog generates");
        if let Some((claim, receipt)) =
            generated
                .provenance
                .outcomes
                .iter()
                .find_map(|outcome| match outcome {
                    ClaimOutcome::Rejected(claim, receipt)
                        if matches!(
                            claim.value,
                            StellarClaimValue::OrbitalHierarchySamplingExhaustion {
                                attempted_candidates: 1
                            }
                        ) =>
                    {
                        Some((claim, receipt))
                    }
                    _ => None,
                })
        {
            assert!(receipt.has_failure());
            assert_eq!(claim.provenance.claim_key, "stellar_orbital_hierarchy");
            assert!(!generated.provenance.outcomes.iter().any(|outcome| {
                matches!(outcome, ClaimOutcome::Unsupported(_, reasons) if reasons.iter().any(|reason| reason.code == "stellar_orbits_sampling_exhausted"))
            }));
            return;
        }
    }
    panic!("fixture range produced no exhausted hierarchy");
}

#[test]
fn unit_close_binary_factor_is_still_retained_as_an_applied_proxy() {
    let seed = 42;
    let mut occurrence_model = PlanetOccurrenceModel::default();
    occurrence_model.close_binary_suppression.occurrence_factor = 1.0;
    let (generator, location) = generator_and_location_with_all_models(
        seed,
        StellarOrbitalHierarchyModel::default(),
        PlanetaryStabilityModel::default(),
        occurrence_model,
    );
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("unit suppression factor remains valid provenance");
    let factors = generated
        .provenance
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .filter(|claim| claim.value == StellarClaimValue::CloseBinaryOccurrenceFactor(1.0))
        .collect::<Vec<_>>();
    assert!(!factors.is_empty());
    for factor in factors {
        assert!(
            generated
                .provenance
                .outcomes
                .iter()
                .filter_map(ClaimOutcome::claim)
                .any(|claim| {
                    matches!(claim.value, StellarClaimValue::PlanetOccurrence(_))
                        && claim
                            .provenance
                            .derivation
                            .as_ref()
                            .is_some_and(|derivation| derivation.input_claims.contains(&factor.id))
                })
        );
    }
}

#[test]
fn failed_hierarchy_attempts_are_individually_addressed_and_retained() {
    let mut orbit_model = StellarOrbitalHierarchyModel::default();
    orbit_model.stability.maximum_sampling_attempts = 3;
    orbit_model.stability.mardling_aarseth_coefficient = 1.0e100;
    let seed = 0;
    let (generator, location) =
        generator_and_location_with_models(seed, orbit_model, PlanetaryStabilityModel::default());
    let generated = generator
        .generate_with_provenance(seed, location)
        .expect("exhausted attempts retain valid provenance");
    let exhausted_system = generated
        .catalog
        .systems
        .iter()
        .find(|system| system.orbital_hierarchy_failed_attempts.len() == 3)
        .expect("fixture includes a three-attempt exhaustion");
    assert_eq!(
        exhausted_system
            .orbital_hierarchy_failed_attempts
            .iter()
            .map(|attempt| attempt.attempt)
            .collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
    let mut retained_passed_constraint = false;
    for diagnostic in &exhausted_system.orbital_hierarchy_failed_attempts {
        assert!(!diagnostic.constraints.is_empty());
        assert!(
            diagnostic
                .constraints
                .iter()
                .any(|constraint| !constraint.passed())
        );
        retained_passed_constraint |= diagnostic
            .constraints
            .iter()
            .any(|constraint| constraint.passed());
        let claim = generated
            .provenance
            .outcomes
            .iter()
            .find_map(|outcome| match outcome {
                ClaimOutcome::Rejected(claim, receipt)
                    if matches!(
                        &claim.value,
                        StellarClaimValue::OrbitalHierarchySamplingAttempt(attempt)
                            if attempt.attempt == diagnostic.attempt
                    ) && claim.provenance.object_id.as_str().contains(&format!(
                        "indexed-u64-le:{:016x}/stellar-system/",
                        exhausted_system.id
                    )) =>
                {
                    Some((claim, receipt))
                }
                _ => None,
            })
            .expect("every diagnostic has one rejected claim");
        assert!(claim.1.has_failure());
        let address = claim
            .0
            .provenance
            .random_draw_address
            .as_ref()
            .expect("attempt retains a stable draw address");
        assert_eq!(address.bounded_attempt_index, 0);
        assert_eq!(claim.1.constraints.len(), diagnostic.constraints.len());
    }
    assert!(
        retained_passed_constraint,
        "passed evaluations are retained alongside failures"
    );

    let mut malformed_attempt = generated.provenance.clone();
    let attempt = malformed_attempt
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Rejected(claim, _) => Some(claim),
            _ => None,
        })
        .find_map(|claim| match &mut claim.value {
            StellarClaimValue::OrbitalHierarchySamplingAttempt(attempt)
                if !attempt.candidate_relationships.is_empty() =>
            {
                Some(attempt)
            }
            _ => None,
        })
        .expect("fixture retains a rejected candidate structure");
    attempt.candidate_relationships[0].orbit.period_days *= 1.01;
    let encoded = ron::to_string(&malformed_attempt).expect("mutated attempt serializes");
    assert!(ron::from_str::<ProvenanceDocument<StellarClaimValue>>(&encoded).is_err());
}

#[test]
fn planetary_claim_payload_mutations_fail_deserialization() {
    let seed = 42;
    let (generator, location) = generator_and_location(seed);
    let document = generator
        .generate_with_provenance(seed, location)
        .expect("catalog generates")
        .provenance;

    let assert_invalid = |mut document: ProvenanceDocument<StellarClaimValue>,
                          mutate: fn(&mut StellarClaimValue)| {
        let claim = document
            .outcomes
            .iter_mut()
            .filter_map(|outcome| match outcome {
                ClaimOutcome::Accepted(claim, _) | ClaimOutcome::Rejected(claim, _) => Some(claim),
                _ => None,
            })
            .find(|claim| matches!(claim.value, StellarClaimValue::ExplicitPlanet(_)))
            .expect("fixture has an explicit planet");
        mutate(&mut claim.value);
        let encoded = ron::to_string(&document).expect("mutated document serializes");
        assert!(ron::from_str::<ProvenanceDocument<StellarClaimValue>>(&encoded).is_err());
    };
    assert_invalid(document.clone(), |value| {
        let StellarClaimValue::ExplicitPlanet(planet) = value else {
            unreachable!()
        };
        planet.semimajor_axis_au = -1.0;
    });
    assert_invalid(document.clone(), |value| {
        let StellarClaimValue::ExplicitPlanet(planet) = value else {
            unreachable!()
        };
        planet.host_member_id = u64::MAX;
        planet.orbital_parent_member_id = u64::MAX;
    });
    assert_invalid(document.clone(), |value| {
        let StellarClaimValue::ExplicitPlanet(planet) = value else {
            unreachable!()
        };
        planet.period_days += 1.0;
    });

    let assert_document_invalid = |document: ProvenanceDocument<StellarClaimValue>| {
        let encoded = ron::to_string(&document).expect("mutated document serializes");
        assert!(ron::from_str::<ProvenanceDocument<StellarClaimValue>>(&encoded).is_err());
    };

    let mut zero_occurrence = document.clone();
    let occurrence = zero_occurrence
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) | ClaimOutcome::Rejected(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| {
            matches!(
                claim.value,
                StellarClaimValue::PlanetOccurrence(
                    simulation::PlanetOccurrenceClaim::PlanetCount { .. }
                )
            )
        })
        .expect("fixture has a positive count occurrence");
    let StellarClaimValue::PlanetOccurrence(simulation::PlanetOccurrenceClaim::PlanetCount {
        count,
        ..
    }) = &mut occurrence.value
    else {
        unreachable!()
    };
    *count = 0;
    assert_document_invalid(zero_occurrence);

    let mut count_mismatch = document.clone();
    let hierarchy = count_mismatch
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) | ClaimOutcome::Rejected(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| matches!(claim.value, StellarClaimValue::StellarOrbitalHierarchy(ref hierarchy) if hierarchy.relative_orbit_count > 0))
        .expect("fixture has a multiple hierarchy");
    let StellarClaimValue::StellarOrbitalHierarchy(hierarchy) = &mut hierarchy.value else {
        unreachable!()
    };
    hierarchy.relative_orbit_count += 1;
    assert_document_invalid(count_mismatch);

    let mut metadata_mismatch = document.clone();
    let relationship = metadata_mismatch
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) | ClaimOutcome::Rejected(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(_)))
        .expect("fixture has a relative orbit");
    let StellarClaimValue::RelativeStellarOrbit(relationship) = &mut relationship.value else {
        unreachable!()
    };
    relationship.sampling_slot = relationship.sampling_slot.wrapping_add(1);
    assert_document_invalid(metadata_mismatch);

    let mut non_keplerian_period = document.clone();
    let (orbit_object, wrong_period) = {
        let relationship = non_keplerian_period
            .outcomes
            .iter_mut()
            .filter_map(|outcome| match outcome {
                ClaimOutcome::Accepted(claim, _) => Some(claim),
                _ => None,
            })
            .find(|claim| matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(_)))
            .expect("fixture has a relative orbit");
        let object = relationship.provenance.object_id.clone();
        let StellarClaimValue::RelativeStellarOrbit(relationship) = &mut relationship.value else {
            unreachable!()
        };
        relationship.orbit.period_days *= 1.01;
        (object, relationship.orbit.period_days)
    };
    let scale = non_keplerian_period
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| {
            claim.provenance.object_id == orbit_object
                && matches!(claim.value, StellarClaimValue::RelativeStellarOrbitScale(_))
        })
        .expect("relationship scale input exists");
    let StellarClaimValue::RelativeStellarOrbitScale(scale) = &mut scale.value else {
        unreachable!()
    };
    scale.period_days = wrong_period;
    assert_document_invalid(non_keplerian_period);

    let mut wrong_subtree_mass = document.clone();
    let (orbit_object, wrong_mass, adjusted_period) = {
        let relationship = wrong_subtree_mass
            .outcomes
            .iter_mut()
            .filter_map(|outcome| match outcome {
                ClaimOutcome::Accepted(claim, _) => Some(claim),
                _ => None,
            })
            .find(|claim| matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(_)))
            .expect("fixture has a relative orbit");
        let object = relationship.provenance.object_id.clone();
        let StellarClaimValue::RelativeStellarOrbit(relationship) = &mut relationship.value else {
            unreachable!()
        };
        relationship.orbit.combined_mass_msun *= 1.01;
        relationship.orbit.period_days = 365.25
            * (relationship.orbit.semimajor_axis_au.powi(3)
                / relationship.orbit.combined_mass_msun)
                .sqrt();
        (
            object,
            relationship.orbit.combined_mass_msun,
            relationship.orbit.period_days,
        )
    };
    let scale = wrong_subtree_mass
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| {
            claim.provenance.object_id == orbit_object
                && matches!(claim.value, StellarClaimValue::RelativeStellarOrbitScale(_))
        })
        .expect("relationship scale input exists");
    let StellarClaimValue::RelativeStellarOrbitScale(scale) = &mut scale.value else {
        unreachable!()
    };
    scale.combined_mass_msun = wrong_mass;
    scale.period_days = adjusted_period;
    assert_document_invalid(wrong_subtree_mass);

    let mut malformed_orbit = document.clone();
    let claim = malformed_orbit
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) | ClaimOutcome::Rejected(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(_)))
        .expect("fixture has a relative orbit");
    let StellarClaimValue::RelativeStellarOrbit(relationship) = &mut claim.value else {
        unreachable!()
    };
    relationship.right_child = relationship.left_child;
    assert_document_invalid(malformed_orbit);

    let mut foreign_member = document.clone();
    let owner = foreign_member
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .find(|claim| matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(_)))
        .and_then(|claim| {
            claim
                .provenance
                .object_id
                .as_str()
                .split("stellar-system-owner:")
                .nth(1)
        })
        .and_then(|rest| rest.split('/').next())
        .expect("relationship has an owner")
        .to_owned();
    let foreign_id = foreign_member
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .find_map(|claim| {
            let object = claim.provenance.object_id.as_str();
            let system = object.strip_prefix("indexed-u64-le:")?.split('/').next()?;
            (system != owner)
                .then(|| object.split("stellar-member:").nth(1))
                .flatten()
                .and_then(|member| u64::from_str_radix(member, 16).ok())
        })
        .expect("fixture has a member owned by another system");
    let relationship = foreign_member
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| {
            matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(relationship)
            if relationship.left_child.kind == simulation::OrbitalNodeClaimKind::StellarMember
                || relationship.right_child.kind == simulation::OrbitalNodeClaimKind::StellarMember)
        })
        .expect("fixture has a stellar-member child");
    let StellarClaimValue::RelativeStellarOrbit(relationship) = &mut relationship.value else {
        unreachable!()
    };
    if relationship.left_child.kind == simulation::OrbitalNodeClaimKind::StellarMember {
        relationship.left_child.stable_id = foreign_id;
    } else {
        relationship.right_child.stable_id = foreign_id;
    }
    assert_document_invalid(foreign_member);

    let mut duplicate_slots = document.clone();
    let relationships = duplicate_slots
        .outcomes
        .iter()
        .enumerate()
        .filter_map(|(index, outcome)| {
            outcome.claim().and_then(|claim| match claim.value {
                StellarClaimValue::RelativeStellarOrbit(relationship) => Some((
                    index,
                    claim
                        .provenance
                        .object_id
                        .as_str()
                        .split("stellar-system-owner:")
                        .nth(1)?
                        .split('/')
                        .next()?
                        .to_owned(),
                    relationship.sampling_slot,
                )),
                _ => None,
            })
        })
        .collect::<Vec<_>>();
    let (first, second) = relationships
        .iter()
        .enumerate()
        .find_map(|(index, first)| {
            relationships[index + 1..]
                .iter()
                .find(|second| second.1 == first.1)
                .map(|second| (first, second))
        })
        .expect("fixture has two relationships in one system");
    let claim = match &mut duplicate_slots.outcomes[second.0] {
        ClaimOutcome::Accepted(claim, _) => claim,
        _ => unreachable!(),
    };
    let StellarClaimValue::RelativeStellarOrbit(relationship) = &mut claim.value else {
        unreachable!()
    };
    relationship.sampling_slot = first.2;
    assert_document_invalid(duplicate_slots);

    let mut cyclic_hierarchy = document.clone();
    let root = cyclic_hierarchy
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .find_map(|claim| match claim.value {
            StellarClaimValue::StellarOrbitalHierarchy(hierarchy)
                if hierarchy.relative_orbit_count > 1 =>
            {
                Some(hierarchy.root)
            }
            _ => None,
        })
        .expect("fixture has a nested hierarchy");
    let relationship = cyclic_hierarchy
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| matches!(
            claim.value,
            StellarClaimValue::RelativeStellarOrbit(relationship)
                if relationship.left_child.kind == simulation::OrbitalNodeClaimKind::Barycentre
                    || relationship.right_child.kind == simulation::OrbitalNodeClaimKind::Barycentre
        ))
        .expect("fixture has a relationship with a barycentre child");
    let StellarClaimValue::RelativeStellarOrbit(relationship) = &mut relationship.value else {
        unreachable!()
    };
    if relationship.left_child.kind == simulation::OrbitalNodeClaimKind::Barycentre {
        relationship.left_child = root;
    } else {
        relationship.right_child = root;
    }
    assert_document_invalid(cyclic_hierarchy);

    let mut swapped_inputs = document.clone();
    let relationship_indices = swapped_inputs
        .outcomes
        .iter()
        .enumerate()
        .filter_map(|(index, outcome)| {
            outcome.claim().and_then(|claim| {
                matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(_)).then_some(index)
            })
        })
        .take(2)
        .collect::<Vec<_>>();
    assert_eq!(
        relationship_indices.len(),
        2,
        "fixture has two relative orbits"
    );
    let mut scale_inputs = Vec::new();
    for index in &relationship_indices {
        let claim = match &swapped_inputs.outcomes[*index] {
            ClaimOutcome::Accepted(claim, _) => claim,
            _ => unreachable!(),
        };
        let input = claim
            .provenance
            .derivation
            .as_ref()
            .unwrap()
            .input_claims
            .iter()
            .find(|id| id.as_str().ends_with("relative_stellar_orbit_scale"))
            .unwrap()
            .clone();
        scale_inputs.push(input);
    }
    for (index, replacement) in relationship_indices
        .into_iter()
        .zip(scale_inputs.into_iter().rev())
    {
        let claim = match &mut swapped_inputs.outcomes[index] {
            ClaimOutcome::Accepted(claim, _) => claim,
            _ => unreachable!(),
        };
        let input = claim
            .provenance
            .derivation
            .as_mut()
            .unwrap()
            .input_claims
            .iter_mut()
            .find(|id| id.as_str().ends_with("relative_stellar_orbit_scale"))
            .unwrap();
        *input = replacement;
    }
    assert_document_invalid(swapped_inputs);

    let mut malformed_root = document.clone();
    let hierarchy = malformed_root
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| matches!(claim.value, StellarClaimValue::StellarOrbitalHierarchy(_)))
        .expect("fixture has a hierarchy");
    let StellarClaimValue::StellarOrbitalHierarchy(hierarchy) = &mut hierarchy.value else {
        unreachable!()
    };
    hierarchy.root.kind = simulation::OrbitalNodeClaimKind::Barycentre;
    hierarchy.root.stable_id = u64::MAX;
    let encoded = ron::to_string(&malformed_root).expect("mutated document serializes");
    assert!(ron::from_str::<ProvenanceDocument<StellarClaimValue>>(&encoded).is_err());

    let mut malformed_stability = document.clone();
    let stability = malformed_stability
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| {
            matches!(
                claim.value,
                StellarClaimValue::CircumstellarStability(
                    simulation::CircumstellarStabilityClaim::CompanionLimited { .. }
                )
            )
        })
        .expect("fixture has companion-limited stability");
    let StellarClaimValue::CircumstellarStability(
        simulation::CircumstellarStabilityClaim::CompanionLimited {
            limiting_barycentre_id,
            ..
        },
    ) = &mut stability.value
    else {
        unreachable!()
    };
    *limiting_barycentre_id = u64::MAX;
    assert_document_invalid(malformed_stability);

    let mut limiting_orbit_mismatch = document.clone();
    let alternative_orbit_id = limiting_orbit_mismatch
        .outcomes
        .iter()
        .filter_map(ClaimOutcome::claim)
        .find(|claim| matches!(claim.value, StellarClaimValue::RelativeStellarOrbit(_)))
        .expect("fixture has a relative orbit")
        .id
        .clone();
    let stability = limiting_orbit_mismatch
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| {
            matches!(
                claim.value,
                StellarClaimValue::CircumstellarStability(
                    simulation::CircumstellarStabilityClaim::CompanionLimited { .. }
                )
            ) && !claim
                .provenance
                .derivation
                .as_ref()
                .unwrap()
                .input_claims
                .contains(&alternative_orbit_id)
        })
        .expect("fixture has stability limited by another orbit");
    let input = stability
        .provenance
        .derivation
        .as_mut()
        .unwrap()
        .input_claims
        .iter_mut()
        .find(|id| id.as_str().ends_with("relative_stellar_orbit"))
        .unwrap();
    *input = alternative_orbit_id;
    assert_document_invalid(limiting_orbit_mismatch);

    let mut malformed_cell = document;
    let cell = malformed_cell
        .outcomes
        .iter_mut()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => Some(claim),
            _ => None,
        })
        .find(|claim| {
            matches!(
                claim.value,
                StellarClaimValue::MDwarfOccurrenceCellSelection { .. }
            )
        })
        .expect("fixture has an M-dwarf cell selection");
    let StellarClaimValue::MDwarfOccurrenceCellSelection { index, cell_count } = &mut cell.value
    else {
        unreachable!()
    };
    *index = *cell_count;
    let encoded = ron::to_string(&malformed_cell).expect("mutated document serializes");
    assert!(ron::from_str::<ProvenanceDocument<StellarClaimValue>>(&encoded).is_err());
}

#[test]
fn planetary_provenance_distinguishes_absence_coverage_rejection_and_mixed_evidence() {
    let mut stability_model = PlanetaryStabilityModel::default();
    stability_model.s_type.fit_residual_lower_factor = 0.0;

    let mut saw_accepted = false;
    let mut saw_rejected = false;
    let mut saw_not_selected = false;
    let mut saw_unsupported = false;
    let mut saw_mixed_planet_summary = false;

    // Seed 0 deterministically exercises all four outcome classes with the zero-margin model.
    for seed in 0..1 {
        let (generator, location) = generator_and_location_with_stability(seed, stability_model);
        let generated = generator
            .generate_with_provenance(seed, location)
            .expect("bounded fixture catalog generates");

        for outcome in &generated.provenance.outcomes {
            match outcome {
                ClaimOutcome::Accepted(claim, receipt)
                    if claim.provenance.claim_key == "explicit_planet_candidate" =>
                {
                    saw_accepted = true;
                    assert!(receipt.is_successful());
                }
                ClaimOutcome::Rejected(claim, receipt)
                    if claim.provenance.claim_key == "explicit_planet_candidate" =>
                {
                    saw_rejected = true;
                    assert!(receipt.has_failure());
                }
                ClaimOutcome::NotSelected(provenance, _)
                    if provenance.claim_key.contains("occurrence") =>
                {
                    saw_not_selected = true;
                }
                ClaimOutcome::Unsupported(provenance, reasons)
                    if provenance.claim_key.contains("occurrence")
                        || provenance.claim_key == "unresolved_planet_population" =>
                {
                    saw_unsupported = true;
                    assert!(!reasons.is_empty());
                }
                _ => {}
            }
        }

        saw_mixed_planet_summary |= generated.provenance.object_summaries.iter().any(|summary| {
            summary.object_id.as_str().contains("/explicit-planet")
                && summary
                    .claim_counts_by_evidence_level
                    .get(&EvidenceLevel::Empirical)
                    .is_some_and(|count| *count > 0)
                && summary
                    .claim_counts_by_evidence_level
                    .get(&EvidenceLevel::PhysicalProxy)
                    .is_some_and(|count| *count > 0)
        });

        if saw_accepted
            && saw_rejected
            && saw_not_selected
            && saw_unsupported
            && saw_mixed_planet_summary
        {
            break;
        }
    }

    assert!(
        saw_accepted,
        "fixture range produces an accepted explicit candidate"
    );
    assert!(
        saw_rejected,
        "zero-margin fixture rejects a candidate without redrawing"
    );
    assert!(saw_not_selected, "fixture range records empirical absence");
    assert!(saw_unsupported, "fixture range records typed coverage gaps");
    assert!(
        saw_mixed_planet_summary,
        "an explicit planet keeps empirical source and physical-proxy realization claims separate"
    );
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
