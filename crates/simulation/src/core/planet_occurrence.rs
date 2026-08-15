//! Empirical planet-occurrence models and deterministic summaries.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanetOccurrenceModelVersion {
    EmpiricalOccurrenceV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FgkSmallPlanetOccurrenceModel {
    pub minimum_effective_temperature_k: f64,
    pub maximum_effective_temperature_k: f64,
    pub minimum_surface_gravity_log10_cgs: f64,
    pub maximum_surface_gravity_log10_cgs: f64,
    pub minimum_iron_abundance_feh: f64,
    pub maximum_iron_abundance_feh: f64,
    pub warm_super_earth_mean: f64,
    pub warm_sub_neptune_solar_mean: f64,
    pub warm_sub_neptune_metallicity_exponent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MDwarfSmallPlanetOccurrenceModel {
    pub minimum_effective_temperature_k: f64,
    pub maximum_effective_temperature_k: f64,
    pub minimum_surface_gravity_log10_cgs: f64,
    pub small_planet_mean: f64,
    pub sub_earth_mean: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GiantPlanetOccurrenceModel {
    pub minimum_host_mass_msun: f64,
    pub maximum_host_mass_msun: f64,
    pub minimum_iron_abundance_feh: f64,
    pub maximum_iron_abundance_feh: f64,
    pub normalization: f64,
    pub host_mass_exponent: f64,
    pub iron_abundance_exponent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CloseBinaryPlanetSuppressionModel {
    pub maximum_semimajor_axis_au: f64,
    pub occurrence_factor: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlanetOccurrenceModel {
    pub model_version: PlanetOccurrenceModelVersion,
    pub fgk_small_planets: FgkSmallPlanetOccurrenceModel,
    pub m_dwarf_small_planets: MDwarfSmallPlanetOccurrenceModel,
    pub giant_planets: GiantPlanetOccurrenceModel,
    pub close_binary_suppression: CloseBinaryPlanetSuppressionModel,
}

impl Default for PlanetOccurrenceModel {
    fn default() -> Self {
        Self {
            model_version: PlanetOccurrenceModelVersion::EmpiricalOccurrenceV1,
            fgk_small_planets: FgkSmallPlanetOccurrenceModel {
                minimum_effective_temperature_k: 4_700.0,
                maximum_effective_temperature_k: 6_500.0,
                minimum_surface_gravity_log10_cgs: 3.9,
                maximum_surface_gravity_log10_cgs: 5.0,
                minimum_iron_abundance_feh: -0.4,
                maximum_iron_abundance_feh: 0.4,
                warm_super_earth_mean: 0.20,
                warm_sub_neptune_solar_mean: 0.282_842_7,
                warm_sub_neptune_metallicity_exponent: 0.376_287_494_6,
            },
            m_dwarf_small_planets: MDwarfSmallPlanetOccurrenceModel {
                minimum_effective_temperature_k: 2_661.0,
                maximum_effective_temperature_k: 3_999.0,
                minimum_surface_gravity_log10_cgs: 3.0,
                small_planet_mean: 2.5,
                sub_earth_mean: 0.3039,
            },
            giant_planets: GiantPlanetOccurrenceModel {
                minimum_host_mass_msun: 0.2,
                maximum_host_mass_msun: 2.0,
                minimum_iron_abundance_feh: -1.0,
                maximum_iron_abundance_feh: 0.55,
                normalization: 0.07,
                host_mass_exponent: 1.0,
                iron_abundance_exponent: 1.2,
            },
            close_binary_suppression: CloseBinaryPlanetSuppressionModel {
                maximum_semimajor_axis_au: 47.0,
                occurrence_factor: 0.34,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StellarMultiplicityEnvironment {
    Single,
    KnownWide,
    KnownCompanionSeparation { semimajor_axis_au: f64 },
    SeparationUnknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SmallPlanetOccurrence {
    FgkWarm {
        warm_super_earth_count: u32,
        warm_sub_neptune_count: u32,
    },
    MDwarfAggregate {
        small_planet_count: u32,
        sub_earth_count: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GiantPlanetOccurrence {
    pub has_at_least_one_cps_giant: bool,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PlanetOccurrenceError {
    #[error("planet occurrence model is invalid")]
    InvalidModel,
    #[error("stellar evolution is unavailable for this host")]
    MissingStellarEvolution,
    #[error("planet occurrence is unsupported for evolutionary state {state:?}")]
    UnsupportedEvolutionaryState { state: EvolutionaryState },
    #[error("stellar observable `{field}` is required by the occurrence calibration")]
    MissingStellarObservable { field: &'static str },
    #[error("host is outside the selected occurrence calibration")]
    OutsideHostCalibration,
    #[error("host [Fe/H] is outside the selected occurrence calibration")]
    OutsideMetallicityCalibration,
    #[error("stellar companion separation is required for planet occurrence")]
    MultiplicitySeparationRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanetOccurrenceQualityFlag {
    PoissonIndependenceApproximation,
    HostAgeDependenceNotModeled,
    MultiplicitySuppressionExtrapolated,
    PlanetPropertiesNotGenerated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanetPopulationSummary {
    pub model_version: PlanetOccurrenceModelVersion,
    /// Applied close-binary suppression factor, when the transferred Kraus step was used.
    pub close_binary_occurrence_factor: Option<f64>,
    pub small_planets: Result<SmallPlanetOccurrence, PlanetOccurrenceError>,
    pub giant_planets: Result<GiantPlanetOccurrence, PlanetOccurrenceError>,
    pub quality_flags: Vec<PlanetOccurrenceQualityFlag>,
}

#[derive(Debug, Clone, Copy)]
pub struct PlanetOccurrenceSampler {
    model: PlanetOccurrenceModel,
}

impl PlanetOccurrenceSampler {
    pub fn new(model: PlanetOccurrenceModel) -> Result<Self, PlanetOccurrenceError> {
        if !valid_planet_occurrence_model(model) {
            return Err(PlanetOccurrenceError::InvalidModel);
        }
        Ok(Self { model })
    }

    pub fn sample(
        &self,
        seed: u64,
        system_id: u64,
        member_id: u64,
        history: StellarPopulationHistory,
        evolution: &Result<StellarEvolutionSnapshot, StellarEvolutionError>,
        multiplicity: StellarMultiplicityEnvironment,
    ) -> PlanetPopulationSummary {
        let mut quality_flags = vec![
            PlanetOccurrenceQualityFlag::HostAgeDependenceNotModeled,
            PlanetOccurrenceQualityFlag::PlanetPropertiesNotGenerated,
        ];
        let factor = match multiplicity {
            StellarMultiplicityEnvironment::Single | StellarMultiplicityEnvironment::KnownWide => {
                Ok(1.0)
            }
            StellarMultiplicityEnvironment::KnownCompanionSeparation { semimajor_axis_au }
                if semimajor_axis_au.is_finite()
                    && semimajor_axis_au >= 0.0
                    && semimajor_axis_au
                        < self
                            .model
                            .close_binary_suppression
                            .maximum_semimajor_axis_au =>
            {
                quality_flags
                    .push(PlanetOccurrenceQualityFlag::MultiplicitySuppressionExtrapolated);
                Ok(self.model.close_binary_suppression.occurrence_factor)
            }
            StellarMultiplicityEnvironment::KnownCompanionSeparation { semimajor_axis_au }
                if semimajor_axis_au.is_finite() && semimajor_axis_au >= 0.0 =>
            {
                Ok(1.0)
            }
            StellarMultiplicityEnvironment::KnownCompanionSeparation { .. }
            | StellarMultiplicityEnvironment::SeparationUnknown => {
                Err(PlanetOccurrenceError::MultiplicitySeparationRequired)
            }
        };
        let close_binary_occurrence_factor = quality_flags
            .contains(&PlanetOccurrenceQualityFlag::MultiplicitySuppressionExtrapolated)
            .then(|| factor.as_ref().ok().copied())
            .flatten();
        let (small_planets, giant_planets) = match factor {
            Err(error) => (Err(error.clone()), Err(error)),
            Ok(factor) => {
                let Ok(snapshot) = evolution else {
                    return PlanetPopulationSummary {
                        model_version: self.model.model_version,
                        close_binary_occurrence_factor,
                        small_planets: Err(PlanetOccurrenceError::MissingStellarEvolution),
                        giant_planets: Err(PlanetOccurrenceError::MissingStellarEvolution),
                        quality_flags,
                    };
                };
                if snapshot.state != EvolutionaryState::MainSequence {
                    let error = PlanetOccurrenceError::UnsupportedEvolutionaryState {
                        state: snapshot.state,
                    };
                    (Err(error.clone()), Err(error))
                } else {
                    (
                        self.sample_small_planets(
                            seed, system_id, member_id, history, snapshot, factor,
                        ),
                        self.sample_giant_planets(
                            seed, system_id, member_id, history, snapshot, factor,
                        ),
                    )
                }
            }
        };
        if small_planets.is_ok() {
            quality_flags.push(PlanetOccurrenceQualityFlag::PoissonIndependenceApproximation);
        }
        PlanetPopulationSummary {
            model_version: self.model.model_version,
            close_binary_occurrence_factor,
            small_planets,
            giant_planets,
            quality_flags,
        }
    }

    fn sample_small_planets(
        &self,
        seed: u64,
        system_id: u64,
        member_id: u64,
        history: StellarPopulationHistory,
        snapshot: &StellarEvolutionSnapshot,
        factor: f64,
    ) -> Result<SmallPlanetOccurrence, PlanetOccurrenceError> {
        let temperature = snapshot.effective_temperature_k.ok_or(
            PlanetOccurrenceError::MissingStellarObservable {
                field: "effective_temperature_k",
            },
        )?;
        let gravity = snapshot.surface_gravity_log10_cgs.ok_or(
            PlanetOccurrenceError::MissingStellarObservable {
                field: "surface_gravity_log10_cgs",
            },
        )?;
        let m = self.model.m_dwarf_small_planets;
        if (m.minimum_effective_temperature_k..=m.maximum_effective_temperature_k)
            .contains(&temperature)
            && gravity > m.minimum_surface_gravity_log10_cgs
        {
            return Ok(SmallPlanetOccurrence::MDwarfAggregate {
                small_planet_count: sample_poisson_count(
                    m.small_planet_mean * factor,
                    &mut domain_rng(
                        seed,
                        b"planet_occurrence/m_dwarf_small/v1",
                        Some(stable_planet_host_id(system_id, member_id)),
                    ),
                ),
                sub_earth_count: sample_poisson_count(
                    m.sub_earth_mean * factor,
                    &mut domain_rng(
                        seed,
                        b"planet_occurrence/m_dwarf_sub_earth/v1",
                        Some(stable_planet_host_id(system_id, member_id)),
                    ),
                ),
            });
        }
        let fgk = self.model.fgk_small_planets;
        if !(fgk.minimum_effective_temperature_k..=fgk.maximum_effective_temperature_k)
            .contains(&temperature)
            || !(fgk.minimum_surface_gravity_log10_cgs..=fgk.maximum_surface_gravity_log10_cgs)
                .contains(&gravity)
        {
            return Err(PlanetOccurrenceError::OutsideHostCalibration);
        }
        let iron = history.chemistry.iron_abundance_feh;
        if !(fgk.minimum_iron_abundance_feh..=fgk.maximum_iron_abundance_feh).contains(&iron) {
            return Err(PlanetOccurrenceError::OutsideMetallicityCalibration);
        }
        let super_earth_mean = fgk.warm_super_earth_mean * factor;
        let sub_neptune_mean = fgk.warm_sub_neptune_solar_mean
            * 10_f64.powf(fgk.warm_sub_neptune_metallicity_exponent * iron)
            * factor;
        Ok(SmallPlanetOccurrence::FgkWarm {
            warm_super_earth_count: sample_poisson_count(
                super_earth_mean,
                &mut domain_rng(
                    seed,
                    b"planet_occurrence/fgk_super_earth/v1",
                    Some(stable_planet_host_id(system_id, member_id)),
                ),
            ),
            warm_sub_neptune_count: sample_poisson_count(
                sub_neptune_mean,
                &mut domain_rng(
                    seed,
                    b"planet_occurrence/fgk_sub_neptune/v1",
                    Some(stable_planet_host_id(system_id, member_id)),
                ),
            ),
        })
    }

    fn sample_giant_planets(
        &self,
        seed: u64,
        system_id: u64,
        member_id: u64,
        history: StellarPopulationHistory,
        snapshot: &StellarEvolutionSnapshot,
        factor: f64,
    ) -> Result<GiantPlanetOccurrence, PlanetOccurrenceError> {
        let giant = self.model.giant_planets;
        if !(giant.minimum_host_mass_msun..=giant.maximum_host_mass_msun)
            .contains(&snapshot.current_mass_msun)
        {
            return Err(PlanetOccurrenceError::OutsideHostCalibration);
        }
        let iron = history.chemistry.iron_abundance_feh;
        if !(giant.minimum_iron_abundance_feh..=giant.maximum_iron_abundance_feh).contains(&iron) {
            return Err(PlanetOccurrenceError::OutsideMetallicityCalibration);
        }
        let probability = giant.normalization
            * snapshot.current_mass_msun.powf(giant.host_mass_exponent)
            * 10_f64.powf(giant.iron_abundance_exponent * iron)
            * factor;
        if !(0.0..=1.0).contains(&probability) {
            return Err(PlanetOccurrenceError::OutsideHostCalibration);
        }
        let mut rng = domain_rng(
            seed,
            b"planet_occurrence/cps_giant/v1",
            Some(stable_planet_host_id(system_id, member_id)),
        );
        Ok(GiantPlanetOccurrence {
            has_at_least_one_cps_giant: rng.gen_bool(probability),
        })
    }
}

fn sample_poisson_count(mean: f64, rng: &mut ChaCha8Rng) -> u32 {
    Poisson::new(mean)
        .expect("validated positive occurrence mean")
        .sample(rng) as u32
}

pub(crate) fn stable_planet_host_id(system_id: u64, member_id: u64) -> u64 {
    let mut input = Vec::with_capacity(56);
    input.extend_from_slice(b"star_sim/planet_occurrence_host/v1");
    input.extend_from_slice(&system_id.to_le_bytes());
    input.extend_from_slice(&member_id.to_le_bytes());
    let hash = blake3::hash(&input);
    u64::from_le_bytes(
        hash.as_bytes()[..8]
            .try_into()
            .expect("eight-byte hash prefix"),
    )
}

pub(crate) fn valid_planet_occurrence_model(model: PlanetOccurrenceModel) -> bool {
    let fgk = model.fgk_small_planets;
    let m = model.m_dwarf_small_planets;
    let giant = model.giant_planets;
    let close = model.close_binary_suppression;
    [
        fgk.minimum_effective_temperature_k,
        fgk.maximum_effective_temperature_k,
        fgk.minimum_surface_gravity_log10_cgs,
        fgk.maximum_surface_gravity_log10_cgs,
        fgk.minimum_iron_abundance_feh,
        fgk.maximum_iron_abundance_feh,
        fgk.warm_super_earth_mean,
        fgk.warm_sub_neptune_solar_mean,
        fgk.warm_sub_neptune_metallicity_exponent,
        m.minimum_effective_temperature_k,
        m.maximum_effective_temperature_k,
        m.minimum_surface_gravity_log10_cgs,
        m.small_planet_mean,
        m.sub_earth_mean,
        giant.minimum_host_mass_msun,
        giant.maximum_host_mass_msun,
        giant.minimum_iron_abundance_feh,
        giant.maximum_iron_abundance_feh,
        giant.normalization,
        giant.host_mass_exponent,
        giant.iron_abundance_exponent,
        close.maximum_semimajor_axis_au,
        close.occurrence_factor,
    ]
    .into_iter()
    .all(f64::is_finite)
        && fgk.minimum_effective_temperature_k < fgk.maximum_effective_temperature_k
        && fgk.minimum_surface_gravity_log10_cgs < fgk.maximum_surface_gravity_log10_cgs
        && fgk.minimum_iron_abundance_feh < fgk.maximum_iron_abundance_feh
        && fgk.warm_super_earth_mean > 0.0
        && fgk.warm_sub_neptune_solar_mean > 0.0
        && m.minimum_effective_temperature_k < m.maximum_effective_temperature_k
        && m.small_planet_mean > 0.0
        && m.sub_earth_mean > 0.0
        && giant.minimum_host_mass_msun < giant.maximum_host_mass_msun
        && giant.minimum_iron_abundance_feh < giant.maximum_iron_abundance_feh
        && giant.normalization > 0.0
        && close.maximum_semimajor_axis_au > 0.0
        && (0.0..=1.0).contains(&close.occurrence_factor)
}
