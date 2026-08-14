use simulation::{
    ClaimOutcome, DeterministicDraws, ExplicitPlanetModel, GalacticLocationSampler,
    GalacticSamplingVolume, GalaxyModel, PlanetOccurrenceModel, PlanetaryStabilityModel,
    PopulationHistoryModel, ProvenanceDocument, SourceId, StellarBirthMassModel,
    StellarCatalogGenerator, StellarClaimValue, StellarEvolutionModel,
    StellarOrbitalHierarchyModel,
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

    let primary_masses: Vec<_> = generated
        .catalog
        .systems
        .iter()
        .map(|system| system.members[0].birth.initial_mass_msun)
        .collect();
    let claimed_primary_masses: Vec<_> = generated
        .provenance
        .outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            ClaimOutcome::Accepted(claim, _) => match claim.value {
                StellarClaimValue::InitialStellarMassMsolar(value) => Some(value),
            },
            ClaimOutcome::NotSelected(_, _)
            | ClaimOutcome::Rejected(_, _)
            | ClaimOutcome::Unsupported(_, _) => None,
        })
        .collect();

    assert!(!primary_masses.is_empty());
    assert_eq!(claimed_primary_masses, primary_masses);
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
    assert!(
        first
            .provenance
            .outcomes
            .iter()
            .all(|outcome| match outcome {
                ClaimOutcome::Accepted(claim, _) | ClaimOutcome::Rejected(claim, _) => {
                    claim.provenance.random_draw_address.is_some()
                }
                ClaimOutcome::NotSelected(provenance, _)
                | ClaimOutcome::Unsupported(provenance, _) => {
                    provenance.random_draw_address.is_some()
                }
            })
    );
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
    let ClaimOutcome::Accepted(claim, _) = &generated.provenance.outcomes[0] else {
        panic!("primary mass claim is accepted");
    };
    let address = claim
        .provenance
        .random_draw_address
        .as_ref()
        .expect("stochastic claim retains its draw address");
    let StellarClaimValue::InitialStellarMassMsolar(actual_mass) = claim.value;

    let draws = DeterministicDraws::new(seed);
    let segment_draw = draws.uniform(address);
    let mut conditional_mass_address = address.clone();
    conditional_mass_address.bounded_attempt_index = 1;
    let mass_draw = draws.uniform(&conditional_mass_address);
    let replayed_mass = primary_mass_from_uniforms(segment_draw, mass_draw);

    assert!((replayed_mass - actual_mass).abs() < 1e-12);
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
    let ClaimOutcome::Accepted(claim, _) = &mut document.outcomes[0] else {
        panic!("initial stellar mass is accepted");
    };
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
