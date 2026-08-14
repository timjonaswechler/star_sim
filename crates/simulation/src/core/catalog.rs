//! Coherent stellar-region and stellar-catalog generation.

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedStellarSystem {
    pub id: u64,
    /// Cartesian offset from the region centre, in parsecs.
    pub offset_pc: [f64; 3],
    pub population: StellarPopulation,
    pub birth_masses: StellarBirthSystem,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedStellarRegion {
    pub centre: GalacticPosition,
    pub radius_pc: f64,
    pub expected_system_count: f64,
    pub systems: Vec<GeneratedStellarSystem>,
}

/// The intentionally fixed spatial extent of the materialised local catalog.
pub const LOCAL_STELLAR_REGION_RADIUS_PC: f64 = 10.0;

#[derive(Debug, Clone, PartialEq)]
pub struct StellarCatalogMember {
    pub birth: StellarBirthMember,
    /// A present-day snapshot or an explicit statement that the bundled model does not cover it.
    pub evolution: Result<StellarEvolutionSnapshot, StellarEvolutionError>,
    pub circumstellar_stability_zone:
        Result<CircumstellarSTypeStabilityZone, PlanetaryStabilityError>,
    pub planet_population: PlanetPopulationSummary,
    pub planetary_system: PlanetarySystemRealization,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StellarCatalogSystem {
    pub id: u64,
    /// Cartesian offset from the catalog centre, in parsecs.
    pub offset_pc: [f64; 3],
    pub population: StellarPopulation,
    /// Formation age and chemistry shared by all coeval members of the system.
    pub history: StellarPopulationHistory,
    pub orbital_hierarchy: Result<StellarOrbitalHierarchy, StellarOrbitalHierarchyError>,
    pub members: Vec<StellarCatalogMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedStellarCatalog {
    pub centre: GalacticPosition,
    pub radius_pc: f64,
    pub expected_system_count: f64,
    pub systems: Vec<StellarCatalogSystem>,
}

#[derive(Debug, Error)]
pub enum StellarCatalogModelError {
    #[error(transparent)]
    InvalidBirthMassModel(#[from] StellarBirthMassError),
    #[error(transparent)]
    InvalidPopulationHistoryModel(#[from] PopulationHistoryError),
    #[error(transparent)]
    InvalidEvolutionModel(#[from] StellarEvolutionError),
    #[error(transparent)]
    InvalidPlanetOccurrenceModel(#[from] PlanetOccurrenceError),
    #[error(transparent)]
    InvalidOrbitalHierarchyModel(#[from] StellarOrbitalHierarchyError),
    #[error(transparent)]
    InvalidPlanetaryStabilityModel(#[from] PlanetaryStabilityError),
    #[error(transparent)]
    InvalidExplicitPlanetModel(#[from] ExplicitPlanetModelError),
}

/// Generates one coherent present-day stellar catalog for the local 10-parsec sphere.
#[derive(Debug, Clone)]
pub struct StellarCatalogGenerator {
    region_generator: StellarRegionGenerator,
    history_sampler: PopulationHistorySampler,
    evolution_evaluator: StellarEvolutionEvaluator,
    planet_occurrence_sampler: PlanetOccurrenceSampler,
    orbital_hierarchy_sampler: StellarOrbitalHierarchySampler,
    planetary_stability_evaluator: PlanetaryStabilityEvaluator,
    explicit_planet_generator: ExplicitPlanetGenerator,
}

impl StellarCatalogGenerator {
    pub fn new(
        birth_mass_model: StellarBirthMassModel,
        population_history_model: PopulationHistoryModel,
        evolution_model: StellarEvolutionModel,
        planet_occurrence_model: PlanetOccurrenceModel,
        orbital_hierarchy_model: StellarOrbitalHierarchyModel,
        planetary_stability_model: PlanetaryStabilityModel,
        explicit_planet_model: ExplicitPlanetModel,
    ) -> Result<Self, StellarCatalogModelError> {
        Ok(Self {
            region_generator: StellarRegionGenerator::new(birth_mass_model)?,
            history_sampler: PopulationHistorySampler::new(population_history_model)?,
            evolution_evaluator: StellarEvolutionEvaluator::new(evolution_model)?,
            planet_occurrence_sampler: PlanetOccurrenceSampler::new(planet_occurrence_model)?,
            orbital_hierarchy_sampler: StellarOrbitalHierarchySampler::new(
                orbital_hierarchy_model,
            )?,
            planetary_stability_evaluator: PlanetaryStabilityEvaluator::new(
                planetary_stability_model,
            )?,
            explicit_planet_generator: ExplicitPlanetGenerator::new(explicit_planet_model)?,
        })
    }

    pub fn with_white_dwarf_cooling(
        mut self,
        model: WhiteDwarfCoolingModel,
    ) -> Result<Self, WhiteDwarfCoolingError> {
        self.evolution_evaluator = self.evolution_evaluator.with_white_dwarf_cooling(model)?;
        Ok(self)
    }

    pub fn generate(
        &self,
        seed: u64,
        location: SampledGalacticLocation,
    ) -> Result<GeneratedStellarCatalog, StellarRegionError> {
        let region =
            self.region_generator
                .generate(seed, location, LOCAL_STELLAR_REGION_RADIUS_PC)?;
        let systems = region
            .systems
            .into_iter()
            .map(|system| {
                let position = region.centre.with_local_offset(system.offset_pc);
                let history =
                    self.history_sampler
                        .sample(seed, system.id, system.population, position);
                let system_member_count = system.birth_masses.members.len();
                let multiple_system = system_member_count > 1;
                let evolved_births: Vec<_> = system
                    .birth_masses
                    .members
                    .into_iter()
                    .map(|birth| {
                        let mut evolution = self.evolution_evaluator.evaluate(
                            birth.initial_mass_msun,
                            history.age_gyr,
                            history.chemistry,
                        );
                        if multiple_system && let Ok(snapshot) = &mut evolution {
                            snapshot
                                .quality_flags
                                .push(StellarEvolutionQualityFlag::BinaryInteractionIgnored);
                        }
                        (birth, evolution)
                    })
                    .collect();
                let orbit_inputs: Result<Vec<_>, _> = evolved_births
                    .iter()
                    .map(|(birth, evolution)| {
                        if !multiple_system {
                            return Ok(StellarOrbitMemberInput {
                                id: birth.id,
                                role: birth.role,
                                mass_msun: birth.initial_mass_msun,
                                radius_rsun: 0.0,
                                provenance: StellarOrbitMemberProvenance::EvolutionSnapshot,
                            });
                        }
                        let snapshot = match evolution.as_ref() {
                            Ok(snapshot) => snapshot,
                            Err(error) => {
                                return low_mass_contact_radius_input(
                                    self.orbital_hierarchy_sampler
                                        .model
                                        .low_mass_contact_radius_proxy,
                                    birth,
                                    history,
                                    error,
                                )
                                .ok_or(StellarOrbitalHierarchyError::MissingStellarEvolution);
                            }
                        };
                        if snapshot.state != EvolutionaryState::MainSequence {
                            return Err(StellarOrbitalHierarchyError::OrbitalEvolutionNotModeled {
                                state: snapshot.state,
                            });
                        }
                        Ok(StellarOrbitMemberInput {
                            id: birth.id,
                            role: birth.role,
                            mass_msun: snapshot.current_mass_msun,
                            radius_rsun: snapshot
                                .radius_rsun
                                .ok_or(StellarOrbitalHierarchyError::MissingStellarRadius)?,
                            provenance: StellarOrbitMemberProvenance::EvolutionSnapshot,
                        })
                    })
                    .collect();
                let orbital_hierarchy = orbit_inputs.and_then(|inputs| {
                    self.orbital_hierarchy_sampler
                        .generate(seed, system.id, &inputs)
                });
                let members = evolved_births
                    .into_iter()
                    .map(|(birth, evolution)| {
                        let multiplicity = if !multiple_system {
                            StellarMultiplicityEnvironment::Single
                        } else if let Ok(hierarchy) = &orbital_hierarchy {
                            hierarchy
                                .nearest_companion_semimajor_axis_au(birth.id)
                                .map(|semimajor_axis_au| {
                                    StellarMultiplicityEnvironment::KnownCompanionSeparation {
                                        semimajor_axis_au,
                                    }
                                })
                                .unwrap_or(StellarMultiplicityEnvironment::SeparationUnknown)
                        } else {
                            StellarMultiplicityEnvironment::SeparationUnknown
                        };
                        let planet_population = self.planet_occurrence_sampler.sample(
                            seed,
                            system.id,
                            birth.id,
                            history,
                            &evolution,
                            multiplicity,
                        );
                        let circumstellar_stability_zone = self
                            .planetary_stability_evaluator
                            .evaluate(birth.id, system_member_count, &orbital_hierarchy);
                        let planetary_system = self.explicit_planet_generator.generate(
                            seed,
                            system.id,
                            birth.id,
                            &evolution,
                            &planet_population,
                            &circumstellar_stability_zone,
                        );
                        StellarCatalogMember {
                            birth,
                            evolution,
                            circumstellar_stability_zone,
                            planet_population,
                            planetary_system,
                        }
                    })
                    .collect();
                StellarCatalogSystem {
                    id: system.id,
                    offset_pc: system.offset_pc,
                    population: system.population,
                    history,
                    orbital_hierarchy,
                    members,
                }
            })
            .collect();

        Ok(GeneratedStellarCatalog {
            centre: region.centre,
            radius_pc: region.radius_pc,
            expected_system_count: region.expected_system_count,
            systems,
        })
    }
}

#[derive(Debug, Error)]
pub enum StellarRegionError {
    #[error("region radius must be finite and greater than zero")]
    InvalidRadius,
    #[error("expected system count is outside the supported numeric range")]
    InvalidExpectedSystemCount,
}

#[derive(Debug, Clone)]
pub struct StellarRegionGenerator {
    birth_mass_sampler: StellarBirthMassSampler,
}

impl StellarRegionGenerator {
    pub fn new(model: StellarBirthMassModel) -> Result<Self, StellarBirthMassError> {
        Ok(Self {
            birth_mass_sampler: StellarBirthMassSampler::new(model)?,
        })
    }

    pub fn generate(
        &self,
        seed: u64,
        location: SampledGalacticLocation,
        radius_pc: f64,
    ) -> Result<GeneratedStellarRegion, StellarRegionError> {
        if !radius_pc.is_finite() || radius_pc <= 0.0 {
            return Err(StellarRegionError::InvalidRadius);
        }
        let sphere_volume = 4.0 / 3.0 * std::f64::consts::PI * radius_pc.powi(3);
        let system_density =
            location.local_density.total() / self.birth_mass_sampler.expected_members_per_system();
        let expected_system_count = system_density * sphere_volume;
        if !expected_system_count.is_finite() || expected_system_count < 0.0 {
            return Err(StellarRegionError::InvalidExpectedSystemCount);
        }

        let system_count = if expected_system_count == 0.0 {
            0
        } else {
            let poisson = Poisson::new(expected_system_count)
                .map_err(|_| StellarRegionError::InvalidExpectedSystemCount)?;
            poisson.sample(&mut domain_rng(
                seed,
                b"stellar_region/system_count/v1",
                None,
            )) as usize
        };

        let mut systems = Vec::with_capacity(system_count);
        for index in 0..system_count {
            let index = index as u64;
            let mut position_rng =
                domain_rng(seed, b"stellar_region/system_position/v1", Some(index));
            let radial_fraction: f64 = position_rng.gen_range(0.0_f64..1.0_f64);
            let distance = radius_pc * radial_fraction.cbrt();
            let cos_theta: f64 = position_rng.gen_range(-1.0_f64..1.0_f64);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            let phi = position_rng.gen_range(0.0..std::f64::consts::TAU);
            let offset_pc = [
                distance * sin_theta * phi.cos(),
                distance * sin_theta * phi.sin(),
                distance * cos_theta,
            ];
            let population = sample_population(
                location.local_density,
                &mut domain_rng(seed, b"stellar_region/system_population/v1", Some(index)),
            );
            let system_id = stable_system_id(seed, index);
            let birth_masses = self.birth_mass_sampler.sample(seed, system_id);

            systems.push(GeneratedStellarSystem {
                id: system_id,
                offset_pc,
                population,
                birth_masses,
            });
        }

        Ok(GeneratedStellarRegion {
            centre: location.position,
            radius_pc,
            expected_system_count,
            systems,
        })
    }
}

fn sample_population(density: PopulationDensity, rng: &mut ChaCha8Rng) -> StellarPopulation {
    let draw = rng.gen_range(0.0..density.total());
    if draw < density.thin_disk {
        StellarPopulation::ThinDisk
    } else if draw < density.thin_disk + density.thick_disk {
        StellarPopulation::ThickDisk
    } else {
        StellarPopulation::Halo
    }
}
