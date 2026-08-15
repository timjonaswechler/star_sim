//! Bevy-independent scientific model and deterministic generation logic.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal, Poisson};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod provenance;

pub use provenance::*;

mod catalog;
mod explicit_planets;
mod galactic_sampling;
mod galaxy;
mod planet_occurrence;
mod planetary_stability;
mod population_history;
mod random;
mod stellar_birth;
mod stellar_evolution;
mod stellar_orbits;

pub use catalog::*;
pub use explicit_planets::*;
pub use galactic_sampling::*;
pub use galaxy::*;
pub use planet_occurrence::*;
pub use planetary_stability::*;
pub use population_history::*;
pub use random::*;
pub use stellar_birth::*;
pub use stellar_evolution::*;
pub use stellar_orbits::*;

pub(crate) use explicit_planets::ExplicitPlanetGenerator;
pub(crate) use planetary_stability::PlanetaryStabilityEvaluator;
pub(crate) use stellar_orbits::{
    StellarOrbitMemberInput, low_mass_contact_radius_input, semimajor_axis_from_period_days,
};

const fn population_index(population: StellarPopulation) -> usize {
    match population {
        StellarPopulation::ThinDisk => 0,
        StellarPopulation::ThickDisk => 1,
        StellarPopulation::Halo => 2,
    }
}

fn position_rng(seed: u64) -> ChaCha8Rng {
    domain_rng(seed, b"galactic_position/v1", None)
}

fn domain_rng(seed: u64, domain: &[u8], index: Option<u64>) -> ChaCha8Rng {
    let mut input = Vec::with_capacity(64);
    input.extend_from_slice(b"star_sim/");
    input.extend_from_slice(domain);
    input.extend_from_slice(&seed.to_le_bytes());
    if let Some(index) = index {
        input.extend_from_slice(&index.to_le_bytes());
    }
    ChaCha8Rng::from_seed(*blake3::hash(&input).as_bytes())
}

fn stable_system_id(seed: u64, index: u64) -> u64 {
    let mut input = Vec::with_capacity(64);
    input.extend_from_slice(b"star_sim/stellar_system_id/v1");
    input.extend_from_slice(&seed.to_le_bytes());
    input.extend_from_slice(&index.to_le_bytes());
    let hash = blake3::hash(&input);
    u64::from_le_bytes(
        hash.as_bytes()[..8]
            .try_into()
            .expect("eight-byte hash prefix"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_birth_mass_model(fractions: [f64; 4]) -> StellarBirthMassModel {
        StellarBirthMassModel {
            initial_mass_function: KroupaInitialMassFunction {
                minimum_mass_msun: 0.08,
                break_mass_msun: 0.5,
                maximum_mass_msun: 100.0,
                low_mass_exponent: 1.3,
                high_mass_exponent: 2.3,
            },
            minimum_companion_mass_msun: 0.08,
            multiplicity_bins: vec![MassConditionedMultiplicity {
                minimum_primary_mass_msun: 0.08,
                maximum_primary_mass_msun: 100.0,
                single_system_fraction: fractions[0],
                binary_system_fraction: fractions[1],
                triple_system_fraction: fractions[2],
                higher_order_system_fraction: fractions[3],
                representative_higher_order_members: 4,
                minimum_mass_ratio: 0.1,
                mass_ratio_power: 0.25,
            }],
        }
    }

    fn test_population_history_model() -> PopulationHistoryModel {
        PopulationHistoryModel {
            model_version: PopulationHistoryModelVersion::SpatialIronAndAlphaV2,
            chemical_composition: ChemicalCompositionModel {
                protosolar_hydrogen_mass_fraction_x: 0.7154,
                solar_metal_mass_fraction_z: 0.0142,
                primordial_helium_mass_fraction_y: 0.249,
                helium_to_metal_enrichment_ratio: 1.5,
                alpha_mixture_fraction: 0.638,
            },
            thin_disk: PopulationHistoryDistribution {
                age_gyr: TruncatedNormal {
                    mean: 5.0,
                    standard_deviation: 2.0,
                    minimum: 0.0,
                    maximum: 10.0,
                },
                iron_abundance_feh: TruncatedNormal {
                    mean: -0.1,
                    standard_deviation: 0.2,
                    minimum: -1.0,
                    maximum: 0.5,
                },
                iron_abundance_radial_gradient: RadialMetallicityGradient {
                    reference_radius_pc: 8_178.0,
                    dex_per_kpc: -0.06,
                    calibration_min_radius_pc: 5_000.0,
                    calibration_max_radius_pc: 19_000.0,
                },
                alpha_enhancement: AlphaEnhancementDistribution {
                    mean_at_solar_iron: 0.035,
                    mean_slope_per_feh: -0.16,
                    mean_minimum: 0.03,
                    mean_maximum: 0.11,
                    standard_deviation: 0.04,
                    minimum: -0.05,
                    maximum: 0.25,
                },
            },
            thick_disk: PopulationHistoryDistribution {
                age_gyr: TruncatedNormal {
                    mean: 10.0,
                    standard_deviation: 1.5,
                    minimum: 7.0,
                    maximum: 13.5,
                },
                iron_abundance_feh: TruncatedNormal {
                    mean: -0.6,
                    standard_deviation: 0.3,
                    minimum: -1.5,
                    maximum: 0.2,
                },
                iron_abundance_radial_gradient: RadialMetallicityGradient {
                    reference_radius_pc: 8_178.0,
                    dex_per_kpc: 0.0,
                    calibration_min_radius_pc: 5_000.0,
                    calibration_max_radius_pc: 19_000.0,
                },
                alpha_enhancement: AlphaEnhancementDistribution {
                    mean_at_solar_iron: 0.160,
                    mean_slope_per_feh: -0.27,
                    mean_minimum: 0.12,
                    mean_maximum: 0.30,
                    standard_deviation: 0.04,
                    minimum: 0.05,
                    maximum: 0.45,
                },
            },
            halo: PopulationHistoryDistribution {
                age_gyr: TruncatedNormal {
                    mean: 12.5,
                    standard_deviation: 0.8,
                    minimum: 9.0,
                    maximum: 13.8,
                },
                iron_abundance_feh: TruncatedNormal {
                    mean: -1.5,
                    standard_deviation: 0.5,
                    minimum: -3.5,
                    maximum: -0.3,
                },
                iron_abundance_radial_gradient: RadialMetallicityGradient {
                    reference_radius_pc: 8_178.0,
                    dex_per_kpc: 0.0,
                    calibration_min_radius_pc: 5_000.0,
                    calibration_max_radius_pc: 19_000.0,
                },
                alpha_enhancement: AlphaEnhancementDistribution {
                    mean_at_solar_iron: 0.25,
                    mean_slope_per_feh: 0.0,
                    mean_minimum: 0.25,
                    mean_maximum: 0.25,
                    standard_deviation: 0.12,
                    minimum: -0.10,
                    maximum: 0.60,
                },
            },
        }
    }

    #[test]
    fn region_radius_must_be_positive_and_finite() {
        let centre = GalacticPosition {
            radius_pc: 8_200.0,
            azimuth_rad: 0.0,
            height_pc: 20.0,
        };

        assert!(RegionRequest::new(42, centre, 10.0).is_some());
        assert!(RegionRequest::new(42, centre, 0.0).is_none());
        assert!(RegionRequest::new(42, centre, f64::NAN).is_none());
    }

    #[test]
    fn density_falls_with_height_above_the_disk() {
        let galaxy = GalaxyModel::default();
        let in_plane = galaxy
            .stellar_number_density_at(GalacticPosition {
                radius_pc: galaxy.solar_radius_pc,
                azimuth_rad: 0.0,
                height_pc: 0.0,
            })
            .total();
        let above_plane = galaxy
            .stellar_number_density_at(GalacticPosition {
                radius_pc: galaxy.solar_radius_pc,
                azimuth_rad: 0.0,
                height_pc: 2_000.0,
            })
            .total();

        assert!(in_plane > above_plane);
    }

    #[test]
    fn calibrated_local_components_sum_to_cns5_stellar_density() {
        let galaxy = GalaxyModel::default();
        let local_density = galaxy
            .stellar_number_density_at(GalacticPosition {
                radius_pc: galaxy.solar_radius_pc,
                azimuth_rad: 0.0,
                height_pc: 0.0,
            })
            .total();

        assert!((local_density - 0.079895).abs() < 1e-6);
    }

    #[test]
    fn rejects_invalid_model_parameters() {
        let mut galaxy = GalaxyModel::default();
        galaxy.halo.flattening = 0.0;

        assert_eq!(
            galaxy.validate(),
            Err(GalaxyModelError::InvalidPositiveParameter {
                field: "halo.flattening"
            })
        );
    }

    #[test]
    fn galactic_location_is_reproducible_and_bounded() {
        let volume = GalacticSamplingVolume {
            radial_bins: 80,
            vertical_bins: 60,
            ..Default::default()
        };
        let sampler = GalacticLocationSampler::new(GalaxyModel::default(), volume).unwrap();

        let first = sampler.sample(42);
        let repeated = sampler.sample(42);
        let different = sampler.sample(43);

        assert_eq!(first, repeated);
        assert_ne!(first.position, different.position);
        assert!(first.position.radius_pc <= volume.max_radius_pc);
        assert!(first.position.height_pc.abs() <= volume.max_abs_height_pc);
    }

    #[test]
    fn sampled_population_frequencies_follow_integrated_density() {
        let sampler = GalacticLocationSampler::new(
            GalaxyModel::default(),
            GalacticSamplingVolume {
                radial_bins: 80,
                vertical_bins: 60,
                ..Default::default()
            },
        )
        .unwrap();
        let mut counts = [0_usize; 3];
        let sample_count = 100_000;
        for seed in 0..sample_count as u64 {
            counts[population_index(sampler.sample(seed).sampled_population)] += 1;
        }

        for (population, expected) in sampler.population_probabilities() {
            let observed = counts[population_index(population)] as f64 / sample_count as f64;
            assert!((observed - expected).abs() < 0.007);
        }
    }

    #[test]
    fn system_density_accounts_for_multiple_stars_per_system() {
        let multiplicity = SystemMultiplicityModel {
            observed_multiplicity_fraction: 0.4,
            observed_companion_frequency: 0.5,
            single_system_fraction: 0.6,
            binary_system_fraction: 0.3,
            triple_system_fraction: 0.1,
            higher_order_system_fraction: 0.0,
            representative_higher_order_members: 4,
        };

        // 60 singles + 30 binaries + 10 triples contain 150 stars in 100 systems.
        assert_eq!(multiplicity.mean_stars_per_system().unwrap(), 1.5);
        assert!((multiplicity.system_density(0.15).unwrap() - 0.1).abs() < 1e-12);
    }

    #[test]
    fn generated_region_is_reproducible_and_bounded_by_its_sphere() {
        let generator =
            StellarRegionGenerator::new(test_birth_mass_model([0.6, 0.3, 0.1, 0.0])).unwrap();
        let centre = SampledGalacticLocation {
            position: GalacticPosition {
                radius_pc: 8_178.0,
                azimuth_rad: 0.0,
                height_pc: 0.0,
            },
            sampled_population: StellarPopulation::ThinDisk,
            local_density: PopulationDensity {
                thin_disk: 0.14,
                thick_disk: 0.01,
                halo: 0.0,
            },
        };

        let first = generator.generate(42, centre, 10.0).unwrap();
        let repeated = generator.generate(42, centre, 10.0).unwrap();

        assert_eq!(first, repeated);
        assert!(!first.systems.is_empty());
        assert!((first.expected_system_count - 418.879_020_478_639_06).abs() < 1e-9);
        assert!(first.systems.iter().all(|system| {
            let [x, y, z] = system.offset_pc;
            x * x + y * y + z * z <= first.radius_pc.powi(2)
        }));
    }

    #[test]
    fn stellar_catalog_is_a_reproducible_coherent_ten_parsec_region() {
        let location =
            GalacticLocationSampler::new(GalaxyModel::default(), GalacticSamplingVolume::default())
                .unwrap()
                .sample(42);
        let birth_mass_model: StellarBirthMassModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_birth_masses.ron"
        ))
        .unwrap();
        let population_history_model: PopulationHistoryModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_population_history.ron"
        ))
        .unwrap();
        let evolution_model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let generator = StellarCatalogGenerator::new(
            birth_mass_model,
            population_history_model,
            evolution_model,
            PlanetOccurrenceModel::default(),
            StellarOrbitalHierarchyModel::default(),
            PlanetaryStabilityModel::default(),
            ExplicitPlanetModel::default(),
        )
        .unwrap();

        let first = generator.generate(42, location).unwrap();
        let repeated = generator.generate(42, location).unwrap();

        assert_eq!(first, repeated);
        let realized_or_retained_planet_outcomes: usize = first
            .systems
            .iter()
            .flat_map(|system| &system.members)
            .map(|member| {
                member.planetary_system.accepted_planets.len()
                    + member.planetary_system.rejected_candidates.len()
                    + member.planetary_system.unresolved_populations.len()
            })
            .sum();
        assert!(realized_or_retained_planet_outcomes > 0);
        let mut explicit_planet_ids: Vec<_> = first
            .systems
            .iter()
            .flat_map(|system| &system.members)
            .flat_map(|member| &member.planetary_system.accepted_planets)
            .map(|planet| planet.id)
            .collect();
        let explicit_planet_count = explicit_planet_ids.len();
        explicit_planet_ids.sort_unstable();
        explicit_planet_ids.dedup();
        assert_eq!(explicit_planet_ids.len(), explicit_planet_count);
        let drawn_small_planet_count: usize = first
            .systems
            .iter()
            .flat_map(|system| &system.members)
            .filter_map(|member| member.planet_population.small_planets.as_ref().ok())
            .map(|occurrence| match occurrence {
                SmallPlanetOccurrence::FgkWarm {
                    warm_super_earth_count,
                    warm_sub_neptune_count,
                } => (*warm_super_earth_count + *warm_sub_neptune_count) as usize,
                SmallPlanetOccurrence::MDwarfAggregate {
                    small_planet_count,
                    sub_earth_count,
                } => (*small_planet_count + *sub_earth_count) as usize,
            })
            .sum();
        let retained_small_planet_candidates = first
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
            .filter(|candidate| {
                matches!(
                    candidate.properties,
                    ExplicitPlanetProperties::TransitRadius { .. }
                )
            })
            .count();
        assert_eq!(retained_small_planet_candidates, drawn_small_planet_count);
        let retained_sub_earths: Vec<_> = first
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
            .filter(|candidate| {
                candidate.source_channel == ExplicitPlanetSourceChannel::MDwarfSubEarth
            })
            .collect();
        assert!(!retained_sub_earths.is_empty());
        assert!(retained_sub_earths.iter().all(|candidate| {
            matches!(
                candidate.properties,
                ExplicitPlanetProperties::TransitRadius { radius_rearth }
                    if (0.5..1.0).contains(&radius_rearth)
            ) && (0.5..=18.2).contains(&candidate.period_days)
                && candidate.quality_flags.contains(
                    &ExplicitPlanetQualityFlag::MDwarfSubEarthOccurrenceLimitedToMeasuredCells,
                )
        }));
        for system in &first.systems {
            for member in &system.members {
                assert!(
                    member
                        .planetary_system
                        .accepted_planets
                        .windows(2)
                        .all(|pair| pair[0].period_days <= pair[1].period_days)
                );
                for planet in &member.planetary_system.accepted_planets {
                    assert_eq!(planet.host_member_id, member.birth.id);
                    assert!(planet.period_days > 0.0);
                    assert!(planet.semimajor_axis_au > 0.0);
                    if let Ok(CircumstellarSTypeStabilityZone::CompanionLimited {
                        fit_residual_lower_semimajor_axis_au,
                        ..
                    }) = &member.circumstellar_stability_zone
                    {
                        assert!(planet.semimajor_axis_au <= *fit_residual_lower_semimajor_axis_au);
                    }
                }
            }
        }
        assert_eq!(first.radius_pc, LOCAL_STELLAR_REGION_RADIUS_PC);
        assert!(!first.systems.is_empty());
        assert!(
            first
                .systems
                .iter()
                .flat_map(|system| &system.members)
                .any(|member| member.planet_population.small_planets.is_ok()
                    || member.planet_population.giant_planets.is_ok())
        );
        let mut companion_limited_zones = 0;
        let mut explicit_stability_coverage_errors = 0;
        for system in &first.systems {
            for member in &system.members {
                match member.circumstellar_stability_zone.as_ref() {
                    Ok(CircumstellarSTypeStabilityZone::UnboundedByStellarCompanion { .. }) => {
                        assert_eq!(system.members.len(), 1);
                    }
                    Ok(CircumstellarSTypeStabilityZone::CompanionLimited {
                        nominal_outer_critical_semimajor_axis_au,
                        limiting_relative_orbit,
                        ..
                    }) => {
                        assert!(system.members.len() > 1);
                        assert!(*nominal_outer_critical_semimajor_axis_au > 0.0);
                        assert!(
                            *nominal_outer_critical_semimajor_axis_au
                                < limiting_relative_orbit.semimajor_axis_au
                                    * (1.0 - limiting_relative_orbit.eccentricity)
                        );
                        companion_limited_zones += 1;
                    }
                    Err(PlanetaryStabilityError::OutsideEccentricityCalibration { .. }) => {
                        assert!(system.members.len() > 1);
                        explicit_stability_coverage_errors += 1;
                    }
                    Err(error) => panic!("unexpected stability-zone result: {error}"),
                }
            }
        }
        assert!(companion_limited_zones > 0);
        assert!(explicit_stability_coverage_errors > 0);
        let quadruple = first
            .systems
            .iter()
            .find(|system| system.members.len() == 4)
            .expect("seed 42 catalog fixture contains one quadruple");
        let quadruple_hierarchy = quadruple.orbital_hierarchy.as_ref().unwrap();
        assert!(
            quadruple
                .members
                .iter()
                .all(|member| member.circumstellar_stability_zone.is_ok())
        );
        assert!(
            quadruple_hierarchy
                .quality_flags
                .contains(&StellarOrbitalHierarchyQualityFlag::LowMassContactRadiusProxy)
        );
        assert!(
            quadruple_hierarchy
                .quality_flags
                .contains(&StellarOrbitalHierarchyQualityFlag::BirthMassUsedAsDynamicalMass)
        );
        let uncovered_low_mass_member = quadruple
            .members
            .iter()
            .find(|member| member.birth.initial_mass_msun < 0.10)
            .expect("seed 42 quadruple contains a sub-0.10 Msun member");
        assert!(matches!(
            uncovered_low_mass_member.evolution,
            Err(StellarEvolutionError::OutsideMassGrid { .. })
        ));
        for system in &first.systems {
            assert!(!system.members.is_empty());
            if let Ok(hierarchy) = &system.orbital_hierarchy {
                let mut hierarchy_member_ids = hierarchy.member_ids();
                let mut catalog_member_ids: Vec<_> = system
                    .members
                    .iter()
                    .map(|member| member.birth.id)
                    .collect();
                hierarchy_member_ids.sort_unstable();
                catalog_member_ids.sort_unstable();
                assert_eq!(hierarchy_member_ids, catalog_member_ids);
                if system.members.len() > 1 {
                    assert!(system.members.iter().all(|member| {
                        hierarchy
                            .nearest_companion_semimajor_axis_au(member.birth.id)
                            .is_some_and(|value| value > 0.0)
                            && !matches!(
                                member.planet_population.small_planets,
                                Err(PlanetOccurrenceError::MultiplicitySeparationRequired)
                            )
                    }));
                }
            }
            assert!(system.members.iter().all(|member| {
                member.evolution.as_ref().map_or(true, |snapshot| {
                    snapshot.initial_mass_msun == member.birth.initial_mass_msun
                        && snapshot.age_gyr == system.history.age_gyr
                })
            }));
            assert!(
                system
                    .members
                    .iter()
                    .all(|member| member.planet_population.model_version
                        == PlanetOccurrenceModelVersion::EmpiricalOccurrenceV1)
            );
            if system.members.len() > 1 {
                assert!(
                    system
                        .members
                        .iter()
                        .filter_map(|member| member.evolution.as_ref().ok())
                        .all(|snapshot| snapshot
                            .quality_flags
                            .contains(&StellarEvolutionQualityFlag::BinaryInteractionIgnored))
                );
            }
            if system.members.len() > 1 && system.orbital_hierarchy.is_err() {
                assert!(system.members.iter().all(|member| matches!(
                    member.planet_population.small_planets,
                    Err(PlanetOccurrenceError::MultiplicitySeparationRequired)
                )));
            }
        }
    }

    #[test]
    fn generated_system_multiplicities_follow_the_configured_distribution() {
        let generator =
            StellarRegionGenerator::new(test_birth_mass_model([0.732, 0.216, 0.048, 0.004]))
                .unwrap();
        let location = SampledGalacticLocation {
            position: GalacticPosition {
                radius_pc: 8_178.0,
                azimuth_rad: 0.0,
                height_pc: 0.0,
            },
            sampled_population: StellarPopulation::ThinDisk,
            local_density: PopulationDensity {
                thin_disk: 0.14,
                thick_disk: 0.01,
                halo: 0.0,
            },
        };
        let mut counts = [0_usize; 4];
        for seed in 0..25_u64 {
            for system in generator.generate(seed, location, 10.0).unwrap().systems {
                counts[system.birth_masses.members.len() - 1] += 1;
            }
        }
        let total: usize = counts.iter().sum();
        let expected = [0.732, 0.216, 0.048, 0.004];
        for (count, expected_fraction) in counts.into_iter().zip(expected) {
            let observed_fraction = count as f64 / total as f64;
            assert!((observed_fraction - expected_fraction).abs() < 0.01);
        }
    }

    #[test]
    fn population_history_is_reproducible_and_respects_configured_ranges() {
        let model = test_population_history_model();
        let sampler = PopulationHistorySampler::new(model).unwrap();

        let position = GalacticPosition {
            radius_pc: 8_178.0,
            azimuth_rad: 0.0,
            height_pc: 0.0,
        };
        let first = sampler.sample(42, 7, StellarPopulation::Halo, position);
        let repeated = sampler.sample(42, 7, StellarPopulation::Halo, position);

        assert_eq!(first, repeated);
        assert!((9.0..=13.8).contains(&first.age_gyr));
        assert!((-3.5..=-0.3).contains(&first.chemistry.iron_abundance_feh));
    }

    #[test]
    fn sampled_history_contains_a_coherent_stellar_chemistry() {
        let sampler = PopulationHistorySampler::new(test_population_history_model()).unwrap();
        let history = sampler.sample(
            42,
            7,
            StellarPopulation::Halo,
            GalacticPosition {
                radius_pc: 8_178.0,
                azimuth_rad: 0.0,
                height_pc: 0.0,
            },
        );
        let chemistry = history.chemistry;

        assert!((-3.5..=-0.3).contains(&chemistry.iron_abundance_feh));
        assert!((0.0..=0.7).contains(&chemistry.alpha_enhancement_alpha_fe));
        assert!(chemistry.global_metallicity_mh > chemistry.iron_abundance_feh);
        assert!((0.0..1.0).contains(&chemistry.metal_mass_fraction_z));
        assert!((0.0..1.0).contains(&chemistry.helium_mass_fraction_y));
        assert!((0.0..1.0).contains(&chemistry.hydrogen_mass_fraction_x));
        let mass_fraction_sum = chemistry.hydrogen_mass_fraction_x
            + chemistry.helium_mass_fraction_y
            + chemistry.metal_mass_fraction_z;
        assert!((mass_fraction_sum - 1.0).abs() < 1e-12);
    }

    #[test]
    fn disc_alpha_enhancement_is_conditioned_on_iron_and_population() {
        let sampler = PopulationHistorySampler::new(test_population_history_model()).unwrap();
        let position = GalacticPosition {
            radius_pc: 8_178.0,
            azimuth_rad: 0.0,
            height_pc: 0.0,
        };
        let mut iron_poor_thin_alpha = Vec::new();
        let mut iron_rich_thin_alpha = Vec::new();
        let mut thin_alpha_sum = 0.0;
        let mut thick_alpha_sum = 0.0;
        let sample_count = 20_000_u64;

        for system_id in 0..sample_count {
            let thin = sampler
                .sample(42, system_id, StellarPopulation::ThinDisk, position)
                .chemistry;
            let thick = sampler
                .sample(42, system_id, StellarPopulation::ThickDisk, position)
                .chemistry;
            thin_alpha_sum += thin.alpha_enhancement_alpha_fe;
            thick_alpha_sum += thick.alpha_enhancement_alpha_fe;
            if thin.iron_abundance_feh < -0.3 {
                iron_poor_thin_alpha.push(thin.alpha_enhancement_alpha_fe);
            } else if thin.iron_abundance_feh > 0.0 {
                iron_rich_thin_alpha.push(thin.alpha_enhancement_alpha_fe);
            }
        }

        let mean = |values: &[f64]| values.iter().sum::<f64>() / values.len() as f64;
        assert!(mean(&iron_poor_thin_alpha) > mean(&iron_rich_thin_alpha) + 0.03);
        assert!(thick_alpha_sum / sample_count as f64 > thin_alpha_sum / sample_count as f64 + 0.1);
    }

    #[test]
    fn outer_thin_disk_is_statistically_more_iron_poor_than_inner_thin_disk() {
        let model = test_population_history_model();
        let sampler = PopulationHistorySampler::new(model).unwrap();
        let inner = GalacticPosition {
            radius_pc: 5_000.0,
            azimuth_rad: 0.0,
            height_pc: 0.0,
        };
        let outer = GalacticPosition {
            radius_pc: 12_000.0,
            ..inner
        };
        let sample_count = 10_000_u64;
        let inner_mean = (0..sample_count)
            .map(|id| {
                sampler
                    .sample(42, id, StellarPopulation::ThinDisk, inner)
                    .chemistry
                    .iron_abundance_feh
            })
            .sum::<f64>()
            / sample_count as f64;
        let outer_mean = (0..sample_count)
            .map(|id| {
                sampler
                    .sample(42, id, StellarPopulation::ThinDisk, outer)
                    .chemistry
                    .iron_abundance_feh
            })
            .sum::<f64>()
            / sample_count as f64;

        assert!(inner_mean - outer_mean > 0.35);
    }

    #[test]
    fn radial_metallicity_correction_stops_at_its_calibration_edges() {
        let sampler = PopulationHistorySampler::new(test_population_history_model()).unwrap();
        let at_inner_edge = GalacticPosition {
            radius_pc: 5_000.0,
            azimuth_rad: 0.0,
            height_pc: 0.0,
        };
        let inside_centre = GalacticPosition {
            radius_pc: 0.0,
            ..at_inner_edge
        };

        assert_eq!(
            sampler.sample(42, 7, StellarPopulation::ThinDisk, at_inner_edge),
            sampler.sample(42, 7, StellarPopulation::ThinDisk, inside_centre)
        );
    }

    #[test]
    fn stellar_birth_mass_sampler_creates_reproducible_primary_constrained_systems() {
        let model = StellarBirthMassModel {
            initial_mass_function: KroupaInitialMassFunction {
                minimum_mass_msun: 0.08,
                break_mass_msun: 0.5,
                maximum_mass_msun: 100.0,
                low_mass_exponent: 1.3,
                high_mass_exponent: 2.3,
            },
            minimum_companion_mass_msun: 0.08,
            multiplicity_bins: vec![MassConditionedMultiplicity {
                minimum_primary_mass_msun: 0.08,
                maximum_primary_mass_msun: 100.0,
                single_system_fraction: 0.0,
                binary_system_fraction: 1.0,
                triple_system_fraction: 0.0,
                higher_order_system_fraction: 0.0,
                representative_higher_order_members: 4,
                minimum_mass_ratio: 0.1,
                mass_ratio_power: 0.0,
            }],
        };
        let sampler = StellarBirthMassSampler::new(model).unwrap();
        assert!((sampler.expected_members_per_system() - 2.0).abs() < 1e-12);

        let first = sampler.sample(42, 7);
        let repeated = sampler.sample(42, 7);

        assert_eq!(first, repeated);
        assert_eq!(first.members.len(), 2);
        assert_eq!(first.members[0].role, StellarMemberRole::Primary);
        assert!((0.08..=100.0).contains(&first.members[0].initial_mass_msun));
        assert_eq!(first.members[1].role, StellarMemberRole::Companion);
        assert!(
            (0.08..=first.members[0].initial_mass_msun)
                .contains(&first.members[1].initial_mass_msun)
        );
        let stored_mass_ratio = first.members[1].mass_ratio_to_primary.unwrap();
        let derived_mass_ratio =
            first.members[1].initial_mass_msun / first.members[0].initial_mass_msun;
        assert!((stored_mass_ratio - derived_mass_ratio).abs() < 1e-12);
    }

    #[test]
    fn sampled_primary_masses_recover_the_kroupa_segment_weight() {
        let sampler =
            StellarBirthMassSampler::new(test_birth_mass_model([1.0, 0.0, 0.0, 0.0])).unwrap();
        let sample_count = 5_000_u64;
        let low_mass_count = (0..sample_count)
            .filter(|system_id| sampler.sample(42, *system_id).members[0].initial_mass_msun < 0.5)
            .count();
        let observed_low_mass_fraction = low_mass_count as f64 / sample_count as f64;

        // Analytic canonical-Kroupa number fraction for 0.08--0.5 Msun.
        assert!((observed_low_mass_fraction - 0.760_707_090_3).abs() < 0.025);
    }

    #[test]
    fn solar_analogue_is_a_main_sequence_star_at_the_suns_age() {
        let evaluator = StellarEvolutionEvaluator::new(StellarEvolutionModel::default()).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 4.6, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::MainSequence);
        assert!((9.0..=11.5).contains(&snapshot.main_sequence_lifetime_gyr));
        assert!((0.9..=1.0).contains(&snapshot.current_mass_msun));
        assert!((0.7..=2.0).contains(&snapshot.luminosity_lsun.unwrap()));
        assert!((5_000.0..=6_500.0).contains(&snapshot.effective_temperature_k.unwrap()));
    }

    #[test]
    fn mist_eep_boundaries_classify_zams_and_tams_exactly() {
        let evaluator = StellarEvolutionEvaluator::new(StellarEvolutionModel::default()).unwrap();
        let zams = evaluator
            .evaluate(1.0, 0.041_873_472_329_859_9, solar_test_chemistry())
            .unwrap();
        let tams = evaluator
            .evaluate(1.0, 9.919_423_942_744_94, solar_test_chemistry())
            .unwrap();

        assert_eq!(zams.raw_eep, 202.0);
        assert_eq!(zams.state, EvolutionaryState::MainSequence);
        assert!((zams.luminosity_lsun.unwrap().log10() + 0.127_208_577_190_252).abs() < 1e-12);
        assert_eq!(tams.raw_eep, 454.0);
        assert_eq!(tams.raw_phase, 2);
        assert_eq!(tams.state, EvolutionaryState::SubgiantAndRedGiantBranch);
    }

    #[test]
    fn unsupported_evolution_ranges_are_never_clamped_to_the_grid() {
        let evaluator = StellarEvolutionEvaluator::new(StellarEvolutionModel::default()).unwrap();

        assert!(matches!(
            evaluator.evaluate(0.09, 1.0, solar_test_chemistry()),
            Err(StellarEvolutionError::OutsideMassGrid { .. })
        ));
        assert!(matches!(
            evaluator.evaluate(1.0, 10.0, solar_test_chemistry()),
            Err(StellarEvolutionError::PostMainSequenceNotBundled { .. })
        ));
    }

    #[test]
    fn bundled_mist_grid_reproduces_solar_track_oracles() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let solar_age = evaluator
            .evaluate(1.0, 4.568, solar_test_chemistry())
            .unwrap();
        assert!((solar_age.current_mass_msun - 0.999_839_456).abs() < 1e-8);
        assert!((solar_age.luminosity_lsun.unwrap().log10() - 0.043_819_491).abs() < 1e-8);
        assert!((solar_age.effective_temperature_k.unwrap().log10() - 3.767_025_909).abs() < 1e-8);

        let lifetimes = [(0.5, 1.0), (1.0, 1.0), (2.0, 0.1), (5.0, 0.01)].map(|(mass, age)| {
            evaluator
                .evaluate(mass, age, solar_test_chemistry())
                .unwrap()
                .tams_age_gyr
        });
        assert!(lifetimes.windows(2).all(|pair| pair[0] > pair[1]));
        for (actual, expected) in
            lifetimes
                .into_iter()
                .zip([95.797_163_5, 9.919_423_94, 1.062_776_94, 0.099_997_577_6])
        {
            assert!((actual - expected).abs() < 1e-7 * expected.max(1.0));
        }
    }

    #[test]
    fn solar_mass_star_reaches_the_giant_branch_after_tams() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 10.5, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::SubgiantAndRedGiantBranch);
        assert!((454.0..605.0).contains(&snapshot.raw_eep));
        assert!((1.5..=4.0).contains(&snapshot.luminosity_lsun.unwrap()));
        assert!((4_500.0..=5_500.0).contains(&snapshot.effective_temperature_k.unwrap()));
    }

    #[test]
    fn solar_mass_star_reaches_core_helium_burning() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 11.4, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::CoreHeliumBurning);
        assert_eq!(snapshot.raw_phase, 3);
        assert!((10.0..=150.0).contains(&snapshot.luminosity_lsun.unwrap()));
    }

    #[test]
    fn rgb_tip_enters_the_helium_ignition_transition_at_eep_605() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 11.336_176_291_479_41, solar_test_chemistry())
            .unwrap();

        assert!((snapshot.raw_eep - 605.0).abs() < 1e-6);
        assert_eq!(snapshot.state, EvolutionaryState::HeliumIgnitionTransition);
        assert!((snapshot.current_mass_msun - 0.953_602_324_825_483).abs() < 1e-10);
    }

    #[test]
    fn solar_mass_star_reaches_the_early_asymptotic_giant_branch() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 11.46, solar_test_chemistry())
            .unwrap();

        assert_eq!(
            snapshot.state,
            EvolutionaryState::EarlyAsymptoticGiantBranch
        );
        assert_eq!(snapshot.raw_phase, 4);
        assert!(snapshot.radius_rsun.unwrap() > 20.0);
    }

    #[test]
    fn solar_mass_star_reaches_the_thermally_pulsing_agb() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 11.462, solar_test_chemistry())
            .unwrap();

        assert_eq!(
            snapshot.state,
            EvolutionaryState::ThermallyPulsingAsymptoticGiantBranch
        );
        assert_eq!(snapshot.raw_phase, 5);
        assert!(snapshot.luminosity_lsun.unwrap() > 100.0);
    }

    #[test]
    fn solar_mass_star_enters_post_agb_before_white_dwarf_cooling() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 11.462_93, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::PostAsymptoticGiantBranch);
        assert_eq!(snapshot.raw_phase, 6);
        assert!(snapshot.luminosity_lsun.unwrap() > 1_000.0);
    }

    #[test]
    fn solar_mass_track_hands_off_to_a_cooling_white_dwarf() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 11.464, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::WhiteDwarf);
        assert!(snapshot.white_dwarf_handoff_age_gyr.is_some());
        assert!(snapshot.cooling_age_gyr.unwrap() > 0.0);
        assert!((0.50..=0.65).contains(&snapshot.current_mass_msun));
        assert_eq!(snapshot.remnant_mass_msun, Some(snapshot.current_mass_msun));
        assert!(snapshot.luminosity_lsun.is_none());
        assert!(snapshot.radius_rsun.is_none());
        assert!(snapshot.effective_temperature_k.is_none());
        assert!(
            snapshot
                .quality_flags
                .contains(&StellarEvolutionQualityFlag::WhiteDwarfCoolingNotBundled)
        );
    }

    #[test]
    fn white_dwarf_owns_the_exact_temperature_knee() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 11.462_959_790_001_43, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::WhiteDwarf);
        assert!(snapshot.cooling_age_gyr.unwrap().abs() < 1e-10);
        assert!((snapshot.current_mass_msun - 0.539_831_842_052_712).abs() < 1e-9);
    }

    #[test]
    fn white_dwarf_identity_survives_beyond_the_bundled_track_tail() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(1.0, 12.0, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::WhiteDwarf);
        assert!(snapshot.cooling_age_gyr.unwrap() > 0.5);
        assert!(snapshot.luminosity_lsun.is_none());
        assert!(snapshot.effective_temperature_k.is_none());
    }

    #[test]
    fn montreal_backend_populates_white_dwarf_photospheric_observables() {
        let point =
            |cooling_age_gyr, luminosity_lsun, radius_rsun, effective_temperature_k, logg| {
                WhiteDwarfCoolingPoint {
                    cooling_age_gyr,
                    luminosity_lsun,
                    radius_rsun,
                    effective_temperature_k,
                    surface_gravity_log10_cgs: logg,
                }
            };
        let cooling_model = WhiteDwarfCoolingModel {
            model_version: WhiteDwarfCoolingModelVersion::MontrealBedard2020ThickHydrogenV1,
            sequences: vec![
                WhiteDwarfCoolingSequence {
                    mass_msun: 0.5,
                    points: vec![
                        point(0.0, 50.0, 0.020, 90_000.0, 7.4),
                        point(1.0, 0.001, 0.014, 5_000.0, 7.9),
                    ],
                },
                WhiteDwarfCoolingSequence {
                    mass_msun: 0.6,
                    points: vec![
                        point(0.0, 56.25, 0.018, 100_000.0, 7.6),
                        point(1.0, 0.0008, 0.012, 4_800.0, 8.1),
                    ],
                },
            ],
        };
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model)
            .unwrap()
            .with_white_dwarf_cooling(cooling_model)
            .unwrap();

        let snapshot = evaluator
            .evaluate(1.0, 12.0, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::WhiteDwarf);
        assert_eq!(
            snapshot.white_dwarf_cooling_model_version,
            Some(WhiteDwarfCoolingModelVersion::MontrealBedard2020ThickHydrogenV1)
        );
        assert!(snapshot.luminosity_lsun.is_some_and(|value| value > 0.0));
        assert!(snapshot.radius_rsun.is_some_and(|value| value > 0.0));
        assert!(
            snapshot
                .effective_temperature_k
                .is_some_and(|value| value > 0.0)
        );
        assert!(
            !snapshot
                .quality_flags
                .contains(&StellarEvolutionQualityFlag::WhiteDwarfCoolingNotBundled)
        );
        assert!(
            snapshot
                .quality_flags
                .contains(&StellarEvolutionQualityFlag::MontrealCoolingHybridModel)
        );
    }

    #[test]
    fn massive_track_end_reports_unsupported_core_collapse() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let error = evaluator
            .evaluate(15.0, 0.02, solar_test_chemistry())
            .unwrap_err();

        assert!(matches!(
            error,
            StellarEvolutionError::UnsupportedCoreCollapse {
                last_current_mass_msun,
                last_carbon_oxygen_core_mass_msun,
                ..
            } if last_current_mass_msun > 0.0
                && last_carbon_oxygen_core_mass_msun > 0.0
        ));
    }

    #[test]
    fn massive_eep_808_is_not_misclassified_as_tp_agb() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(15.0, 0.013_822_753_672_319_613, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::AdvancedBurningTrackEnd);
        assert_eq!(snapshot.raw_phase, 5);
    }

    #[test]
    fn massive_track_can_enter_a_wolf_rayet_state() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(100.0, 0.0031, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::WolfRayet);
        assert_eq!(snapshot.raw_phase, 9);
        assert!(snapshot.current_mass_msun < 100.0);
    }

    #[test]
    fn post_agb_track_without_a_temperature_knee_reports_incomplete() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();

        assert!(matches!(
            evaluator.evaluate(3.0, 0.5, solar_test_chemistry()),
            Err(StellarEvolutionError::PostAgbTrackIncomplete { last_eep: 1409, .. })
        ));
    }

    #[test]
    fn prematurely_terminated_massive_track_is_not_called_core_collapse() {
        let model: StellarEvolutionModel = ron::from_str(include_str!(
            "../../../../assets/scientific_models/stellar_evolution.ron"
        ))
        .unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();

        assert!(matches!(
            evaluator.evaluate(40.0, 0.01, solar_test_chemistry()),
            Err(StellarEvolutionError::TrackEndedBeforeExpectedEndpoint { last_eep: 631, .. })
        ));
    }

    fn solar_test_chemistry() -> StellarChemistry {
        StellarChemistry {
            iron_abundance_feh: 0.0,
            alpha_enhancement_alpha_fe: 0.0,
            global_metallicity_mh: 0.0,
            hydrogen_mass_fraction_x: 0.7154,
            helium_mass_fraction_y: 0.2703,
            metal_mass_fraction_z: 0.0142,
        }
    }

    #[test]
    fn generated_system_populations_follow_local_density_fractions() {
        let generator =
            StellarRegionGenerator::new(test_birth_mass_model([1.0, 0.0, 0.0, 0.0])).unwrap();
        let location = SampledGalacticLocation {
            position: GalacticPosition {
                radius_pc: 8_178.0,
                azimuth_rad: 0.0,
                height_pc: 0.0,
            },
            sampled_population: StellarPopulation::ThinDisk,
            local_density: PopulationDensity {
                thin_disk: 0.07,
                thick_disk: 0.02,
                halo: 0.01,
            },
        };
        let mut counts = [0_usize; 3];
        for seed in 0..25_u64 {
            for system in generator.generate(seed, location, 10.0).unwrap().systems {
                counts[population_index(system.population)] += 1;
            }
        }
        let total: usize = counts.iter().sum();
        for (observed, expected) in counts.into_iter().zip([0.7, 0.2, 0.1]) {
            assert!((observed as f64 / total as f64 - expected).abs() < 0.015);
        }
    }
}
