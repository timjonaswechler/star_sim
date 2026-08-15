//! Coherent stellar-region and stellar-catalog generation.

use super::*;

mod provenance;
mod provenance_values;

pub use provenance_values::*;

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
    /// Every immutable bounded placement attempt rejected before the accepted result or exhaustion.
    pub orbital_hierarchy_failed_attempts: Vec<StellarOrbitalHierarchyAttemptDiagnostic>,
    /// Inputs actually supplied to orbital hierarchy generation when construction reached that seam.
    pub orbital_member_inputs: Vec<StellarOrbitMemberInputProvenance>,
    pub members: Vec<StellarCatalogMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedStellarCatalog {
    pub centre: GalacticPosition,
    pub radius_pc: f64,
    pub expected_system_count: f64,
    pub systems: Vec<StellarCatalogSystem>,
}

/// A generated catalog paired with its validated scientific provenance graph.
#[derive(Debug, Clone, PartialEq)]
pub struct ProvenanceBearingStellarCatalog {
    pub catalog: GeneratedStellarCatalog,
    pub provenance: ProvenanceDocument<StellarClaimValue>,
}

#[derive(Debug, Error)]
pub enum StellarCatalogGenerationError {
    #[error(transparent)]
    GenerateCatalog(#[from] StellarRegionError),
    #[error(transparent)]
    BuildProvenance(#[from] ProvenanceError),
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
    birth_mass_model: StellarBirthMassModel,
    region_generator: StellarRegionGenerator,
    history_sampler: PopulationHistorySampler,
    population_history_model: PopulationHistoryModel,
    evolution_evaluator: StellarEvolutionEvaluator,
    evolution_model_fingerprint: String,
    planet_occurrence_model: PlanetOccurrenceModel,
    planet_occurrence_sampler: PlanetOccurrenceSampler,
    orbital_hierarchy_model: StellarOrbitalHierarchyModel,
    orbital_hierarchy_sampler: StellarOrbitalHierarchySampler,
    planetary_stability_model: PlanetaryStabilityModel,
    planetary_stability_evaluator: PlanetaryStabilityEvaluator,
    explicit_planet_model: ExplicitPlanetModel,
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
        let evolution_model_fingerprint = scientific_model_fingerprint(&evolution_model);
        Ok(Self {
            birth_mass_model: birth_mass_model.clone(),
            region_generator: StellarRegionGenerator::new(birth_mass_model)?,
            history_sampler: PopulationHistorySampler::new(population_history_model)?,
            population_history_model,
            evolution_evaluator: StellarEvolutionEvaluator::new(evolution_model)?,
            evolution_model_fingerprint,
            planet_occurrence_model,
            planet_occurrence_sampler: PlanetOccurrenceSampler::new(planet_occurrence_model)?,
            orbital_hierarchy_model,
            orbital_hierarchy_sampler: StellarOrbitalHierarchySampler::new(
                orbital_hierarchy_model,
            )?,
            planetary_stability_model,
            planetary_stability_evaluator: PlanetaryStabilityEvaluator::new(
                planetary_stability_model,
            )?,
            explicit_planet_model: explicit_planet_model.clone(),
            explicit_planet_generator: ExplicitPlanetGenerator::new(explicit_planet_model)?,
        })
    }

    pub fn with_white_dwarf_cooling(
        mut self,
        model: WhiteDwarfCoolingModel,
    ) -> Result<Self, WhiteDwarfCoolingError> {
        self.evolution_model_fingerprint =
            scientific_model_fingerprint(&(self.evolution_model_fingerprint.as_str(), &model));
        self.evolution_evaluator = self.evolution_evaluator.with_white_dwarf_cooling(model)?;
        Ok(self)
    }

    /// Generates a catalog and publishes claim-level provenance at the catalog seam.
    ///
    /// Internal generators remain provenance-agnostic; this method translates their
    /// domain outputs into claims after ordinary catalog generation has completed.
    pub fn generate_with_provenance(
        &self,
        seed: u64,
        location: SampledGalacticLocation,
    ) -> Result<ProvenanceBearingStellarCatalog, StellarCatalogGenerationError> {
        let catalog = self.generate(seed, location)?;
        let provenance = provenance::generate(
            seed,
            &catalog,
            &self.birth_mass_model,
            &self.population_history_model,
            &self.evolution_model_fingerprint,
            &self.orbital_hierarchy_model,
            &self.planetary_stability_model,
            &self.planet_occurrence_model,
            &self.explicit_planet_model,
        )?;
        Ok(ProvenanceBearingStellarCatalog {
            catalog,
            provenance,
        })
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
                                provenance: StellarOrbitMemberProvenance::SingleMemberInitialMass,
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
                            provenance:
                                StellarOrbitMemberProvenance::CurrentMassAndRadiusFromEvolution,
                        })
                    })
                    .collect();
                let (orbital_member_inputs, orbital_hierarchy, orbital_hierarchy_failed_attempts) =
                    match orbit_inputs {
                        Ok(inputs) => {
                            let provenance = inputs
                                .iter()
                                .map(|input| StellarOrbitMemberInputProvenance {
                                    member_id: input.id,
                                    input_source: input.provenance,
                                })
                                .collect();
                            let (hierarchy, failed_attempts) = self
                                .orbital_hierarchy_sampler
                                .generate_with_diagnostics(seed, system.id, &inputs);
                            (provenance, hierarchy, failed_attempts)
                        }
                        Err(error) => (Vec::new(), Err(error), Vec::new()),
                    };
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
                    orbital_hierarchy_failed_attempts,
                    orbital_member_inputs,
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

fn scientific_model_fingerprint(model: &impl std::fmt::Debug) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(format!("{model:?}").as_bytes());
    hasher.finalize().to_hex()[..16].to_owned()
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
            let system_id = stable_system_id(seed, index);
            let population_draw_scope = RandomDrawScope::new(
                "stellar_region/system_population/v1",
                ObjectId::from(format!("indexed-u64-le:{system_id:016x}/stellar-system")),
                "stellar_population",
            )
            .expect("static Stellar Population draw identity is valid");
            let population = sample_population_from_uniform(
                location.local_density,
                DeterministicDraws::new(seed).uniform(&population_draw_scope.at(0)),
            );
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

fn sample_population_from_uniform(
    density: PopulationDensity,
    uniform_draw: f64,
) -> StellarPopulation {
    let draw = uniform_draw * density.total();
    if draw < density.thin_disk {
        StellarPopulation::ThinDisk
    } else if draw < density.thin_disk + density.thick_disk {
        StellarPopulation::ThickDisk
    } else {
        StellarPopulation::Halo
    }
}
