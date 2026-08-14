//! Stellar evolution tracks, white-dwarf cooling, and interpolation.

use super::*;

/// Versioned stellar-track subset used by the deterministic single-star evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StellarEvolutionModelVersion {
    MistV12NonRotatingSolarScaledThroughWhiteDwarfHandoffV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StellarEvolutionTrackBranch {
    WhiteDwarfProgenitor,
    MassiveBurning,
}

/// One reduced MIST equivalent evolutionary point (EEP).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StellarEvolutionTrackPoint {
    pub eep: u16,
    pub age_gyr: f64,
    pub current_mass_msun: f64,
    pub carbon_oxygen_core_mass_msun: f64,
    pub log10_luminosity_lsun: f64,
    pub log10_effective_temperature_k: f64,
    pub log10_radius_rsun: f64,
    pub surface_gravity_log10_cgs: f64,
    pub phase: i8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarEvolutionTrack {
    pub initial_mass_msun: f64,
    /// Solar-scaled MIST composition coordinate; alpha-enhanced chemistry is projected onto it.
    pub global_metallicity_mh: f64,
    pub branch: StellarEvolutionTrackBranch,
    pub primary_eeps: Vec<u16>,
    pub points: Vec<StellarEvolutionTrackPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarEvolutionModel {
    pub model_version: StellarEvolutionModelVersion,
    pub tracks: Vec<StellarEvolutionTrack>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhiteDwarfCoolingModelVersion {
    MontrealBedard2020ThickHydrogenV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WhiteDwarfCoolingPoint {
    pub cooling_age_gyr: f64,
    pub luminosity_lsun: f64,
    pub radius_rsun: f64,
    pub effective_temperature_k: f64,
    pub surface_gravity_log10_cgs: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteDwarfCoolingSequence {
    pub mass_msun: f64,
    pub points: Vec<WhiteDwarfCoolingPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteDwarfCoolingModel {
    pub model_version: WhiteDwarfCoolingModelVersion,
    pub sequences: Vec<WhiteDwarfCoolingSequence>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum WhiteDwarfCoolingError {
    #[error("white-dwarf cooling model contains invalid sequences")]
    InvalidModel,
    #[error("white-dwarf cooling input `{field}` is invalid")]
    InvalidInput { field: &'static str },
    #[error("white-dwarf mass {mass_msun:.4} Msun requires a non-C/O core model")]
    UnsupportedCoreComposition { mass_msun: f64 },
    #[error(
        "white-dwarf mass {mass_msun:.4} Msun is outside the loaded cooling grid {minimum_mass_msun:.4}..={maximum_mass_msun:.4} Msun"
    )]
    OutsideMassGrid {
        mass_msun: f64,
        minimum_mass_msun: f64,
        maximum_mass_msun: f64,
    },
    #[error("cooling age {cooling_age_gyr:.6} Gyr is not covered at mass {mass_msun:.4} Msun")]
    OutsideAgeGrid {
        mass_msun: f64,
        cooling_age_gyr: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WhiteDwarfCoolingSnapshot {
    pub model_version: WhiteDwarfCoolingModelVersion,
    pub cooling_age_gyr: f64,
    pub luminosity_lsun: f64,
    pub radius_rsun: f64,
    pub effective_temperature_k: f64,
    pub surface_gravity_log10_cgs: f64,
    pub young_cooling_zero_point_uncertain: bool,
}

#[derive(Debug, Clone)]
pub struct WhiteDwarfCoolingEvaluator {
    model: WhiteDwarfCoolingModel,
    masses: Vec<f64>,
}

impl WhiteDwarfCoolingEvaluator {
    pub fn new(model: WhiteDwarfCoolingModel) -> Result<Self, WhiteDwarfCoolingError> {
        let masses: Vec<_> = model
            .sequences
            .iter()
            .map(|sequence| sequence.mass_msun)
            .collect();
        let valid = masses.len() >= 2
            && masses
                .windows(2)
                .all(|pair| pair[0].is_finite() && pair[0] < pair[1])
            && masses.last().is_some_and(|mass| mass.is_finite())
            && model.sequences.iter().all(|sequence| {
                sequence.points.len() >= 2
                    && sequence.points[0].cooling_age_gyr == 0.0
                    && sequence.points.windows(2).all(|pair| {
                        pair[0].cooling_age_gyr >= 0.0
                            && pair[0].cooling_age_gyr < pair[1].cooling_age_gyr
                    })
                    && sequence.points.iter().all(valid_white_dwarf_cooling_point)
            });
        if !valid {
            return Err(WhiteDwarfCoolingError::InvalidModel);
        }
        Ok(Self { model, masses })
    }

    pub fn evaluate(
        &self,
        mass_msun: f64,
        cooling_age_gyr: f64,
    ) -> Result<WhiteDwarfCoolingSnapshot, WhiteDwarfCoolingError> {
        if !mass_msun.is_finite() || mass_msun <= 0.0 {
            return Err(WhiteDwarfCoolingError::InvalidInput { field: "mass_msun" });
        }
        if !cooling_age_gyr.is_finite() || cooling_age_gyr < 0.0 {
            return Err(WhiteDwarfCoolingError::InvalidInput {
                field: "cooling_age_gyr",
            });
        }
        if !(0.45..=1.10).contains(&mass_msun) {
            return Err(WhiteDwarfCoolingError::UnsupportedCoreComposition { mass_msun });
        }
        let (mass_low, mass_high, mass_fraction) = bracket_grid(&self.masses, mass_msun)
            .ok_or_else(|| WhiteDwarfCoolingError::OutsideMassGrid {
                mass_msun,
                minimum_mass_msun: self.masses[0],
                maximum_mass_msun: *self.masses.last().expect("validated cooling grid"),
            })?;
        let low = interpolate_white_dwarf_sequence(
            self.model
                .sequences
                .iter()
                .find(|sequence| sequence.mass_msun == mass_low)
                .expect("validated cooling mass"),
            cooling_age_gyr,
        )?;
        let high = interpolate_white_dwarf_sequence(
            self.model
                .sequences
                .iter()
                .find(|sequence| sequence.mass_msun == mass_high)
                .expect("validated cooling mass"),
            cooling_age_gyr,
        )?;
        Ok(WhiteDwarfCoolingSnapshot {
            model_version: self.model.model_version,
            cooling_age_gyr,
            luminosity_lsun: log_lerp(low.luminosity_lsun, high.luminosity_lsun, mass_fraction),
            radius_rsun: log_lerp(low.radius_rsun, high.radius_rsun, mass_fraction),
            effective_temperature_k: log_lerp(
                low.effective_temperature_k,
                high.effective_temperature_k,
                mass_fraction,
            ),
            surface_gravity_log10_cgs: lerp(
                low.surface_gravity_log10_cgs,
                high.surface_gravity_log10_cgs,
                mass_fraction,
            ),
            young_cooling_zero_point_uncertain: cooling_age_gyr <= 1.0e-4,
        })
    }
}

fn valid_white_dwarf_cooling_point(point: &WhiteDwarfCoolingPoint) -> bool {
    point.cooling_age_gyr.is_finite()
        && point.cooling_age_gyr >= 0.0
        && point.luminosity_lsun.is_finite()
        && point.luminosity_lsun > 0.0
        && point.radius_rsun.is_finite()
        && point.radius_rsun > 0.0
        && point.effective_temperature_k.is_finite()
        && point.effective_temperature_k > 0.0
        && point.surface_gravity_log10_cgs.is_finite()
}

fn interpolate_white_dwarf_sequence(
    sequence: &WhiteDwarfCoolingSequence,
    cooling_age_gyr: f64,
) -> Result<WhiteDwarfCoolingPoint, WhiteDwarfCoolingError> {
    let last_age = sequence
        .points
        .last()
        .expect("validated sequence")
        .cooling_age_gyr;
    if cooling_age_gyr > last_age {
        return Err(WhiteDwarfCoolingError::OutsideAgeGrid {
            mass_msun: sequence.mass_msun,
            cooling_age_gyr,
        });
    }
    let upper_index = sequence
        .points
        .partition_point(|point| point.cooling_age_gyr < cooling_age_gyr);
    if upper_index == 0 {
        return Ok(sequence.points[0]);
    }
    if upper_index == sequence.points.len() {
        return Ok(*sequence.points.last().expect("validated sequence"));
    }
    let lower = sequence.points[upper_index - 1];
    let upper = sequence.points[upper_index];
    let fraction = if lower.cooling_age_gyr == 0.0 {
        cooling_age_gyr / upper.cooling_age_gyr
    } else {
        (cooling_age_gyr.log10() - lower.cooling_age_gyr.log10())
            / (upper.cooling_age_gyr.log10() - lower.cooling_age_gyr.log10())
    };
    Ok(WhiteDwarfCoolingPoint {
        cooling_age_gyr,
        luminosity_lsun: log_lerp(lower.luminosity_lsun, upper.luminosity_lsun, fraction),
        radius_rsun: log_lerp(lower.radius_rsun, upper.radius_rsun, fraction),
        effective_temperature_k: log_lerp(
            lower.effective_temperature_k,
            upper.effective_temperature_k,
            fraction,
        ),
        surface_gravity_log10_cgs: lerp(
            lower.surface_gravity_log10_cgs,
            upper.surface_gravity_log10_cgs,
            fraction,
        ),
    })
}

fn log_lerp(lower: f64, upper: f64, fraction: f64) -> f64 {
    10_f64.powf(lerp(lower.log10(), upper.log10(), fraction))
}

impl Default for StellarEvolutionModel {
    fn default() -> Self {
        // Compact exact-node fixture. The application loads the larger reduced grid from RON.
        let point = |eep,
                     age_gyr,
                     current_mass_msun,
                     log10_luminosity_lsun,
                     log10_effective_temperature_k,
                     log10_radius_rsun,
                     surface_gravity_log10_cgs,
                     phase| StellarEvolutionTrackPoint {
            eep,
            age_gyr,
            current_mass_msun,
            carbon_oxygen_core_mass_msun: 0.0,
            log10_luminosity_lsun,
            log10_effective_temperature_k,
            log10_radius_rsun,
            surface_gravity_log10_cgs,
            phase,
        };
        Self {
            model_version:
                StellarEvolutionModelVersion::MistV12NonRotatingSolarScaledThroughWhiteDwarfHandoffV2,
            tracks: vec![StellarEvolutionTrack {
                initial_mass_msun: 1.0,
                global_metallicity_mh: 0.0,
                branch: StellarEvolutionTrackBranch::WhiteDwarfProgenitor,
                primary_eeps: vec![1, 202, 353, 454],
                points: vec![
                    point(
                        1,
                        1.76636786067929e-6,
                        0.999999932043126,
                        1.74769124750463,
                        3.61121667787835,
                        1.17408942017772,
                        2.08996734769062,
                        -1,
                    ),
                    point(
                        201,
                        0.0397540235802898,
                        0.999997430495567,
                        -0.124441823862272,
                        3.75691022019445,
                        -0.0533642001379212,
                        4.54487350191217,
                        -1,
                    ),
                    point(
                        202,
                        0.0418734723298599,
                        0.999997374271683,
                        -0.127208577190252,
                        3.75641221426305,
                        -0.0537515649391111,
                        4.54564820709677,
                        0,
                    ),
                    point(
                        354,
                        4.54158574272208,
                        0.999840643288479,
                        0.0427563645541815,
                        3.76698391100491,
                        0.0100875124493893,
                        4.41790197940258,
                        0,
                    ),
                    point(
                        355,
                        4.58181508971474,
                        0.999838835221692,
                        0.0443755236443844,
                        3.7670478749557,
                        0.0107691640929126,
                        4.41653789075507,
                        0,
                    ),
                    point(
                        453,
                        9.87950379657566,
                        0.999443916523462,
                        0.354704972893881,
                        3.75559623221534,
                        0.188837174198377,
                        4.060230298004,
                        0,
                    ),
                    point(
                        454,
                        9.91942394274494,
                        0.99943830963835,
                        0.358461324716226,
                        3.75465257728806,
                        0.192602659964104,
                        4.05269689007178,
                        2,
                    ),
                ],
            }],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionaryState {
    PreMainSequence,
    MainSequence,
    SubgiantAndRedGiantBranch,
    HeliumIgnitionTransition,
    CoreHeliumBurning,
    EarlyAsymptoticGiantBranch,
    ThermallyPulsingAsymptoticGiantBranch,
    AdvancedBurningTrackEnd,
    WolfRayet,
    PostAsymptoticGiantBranch,
    WhiteDwarf,
}

impl EvolutionaryState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::PreMainSequence => "pre-main sequence",
            Self::MainSequence => "main sequence",
            Self::SubgiantAndRedGiantBranch => "subgiant and red-giant branch",
            Self::HeliumIgnitionTransition => "helium-ignition transition",
            Self::CoreHeliumBurning => "core-helium burning",
            Self::EarlyAsymptoticGiantBranch => "early asymptotic giant branch",
            Self::ThermallyPulsingAsymptoticGiantBranch => {
                "thermally pulsing asymptotic giant branch"
            }
            Self::AdvancedBurningTrackEnd => "advanced-burning track end",
            Self::WolfRayet => "Wolf-Rayet",
            Self::PostAsymptoticGiantBranch => "post-asymptotic giant branch",
            Self::WhiteDwarf => "white dwarf",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StellarEvolutionQualityFlag {
    AlphaProjectedToSolarScaled,
    BinaryInteractionIgnored,
    WhiteDwarfCoolingNotBundled,
    WhiteDwarfCoolingOutsideModelCoverage,
    MontrealCoolingHybridModel,
    YoungWhiteDwarfCoolingZeroPointUncertain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarEvolutionSnapshot {
    pub model_version: StellarEvolutionModelVersion,
    pub initial_mass_msun: f64,
    pub age_gyr: f64,
    pub source_metallicity_coordinate_mh: f64,
    pub state: EvolutionaryState,
    pub raw_eep: f64,
    pub raw_phase: i8,
    pub zams_age_gyr: f64,
    pub tams_age_gyr: f64,
    pub main_sequence_lifetime_gyr: f64,
    pub fractional_main_sequence_age: Option<f64>,
    pub white_dwarf_handoff_age_gyr: Option<f64>,
    pub cooling_age_gyr: Option<f64>,
    pub remnant_mass_msun: Option<f64>,
    pub white_dwarf_cooling_model_version: Option<WhiteDwarfCoolingModelVersion>,
    pub current_mass_msun: f64,
    pub luminosity_lsun: Option<f64>,
    pub radius_rsun: Option<f64>,
    pub effective_temperature_k: Option<f64>,
    pub surface_gravity_log10_cgs: Option<f64>,
    pub quality_flags: Vec<StellarEvolutionQualityFlag>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum StellarEvolutionError {
    #[error("stellar-evolution model contains invalid or incompatible EEP tracks")]
    InvalidModel,
    #[error("stellar-evolution input `{field}` is invalid")]
    InvalidInput { field: &'static str },
    #[error(
        "initial mass {initial_mass_msun:.4} Msun is outside the bundled MIST range {minimum_mass_msun:.4}..={maximum_mass_msun:.4} Msun"
    )]
    OutsideMassGrid {
        initial_mass_msun: f64,
        minimum_mass_msun: f64,
        maximum_mass_msun: f64,
    },
    #[error(
        "[M/H] {global_metallicity_mh:+.3} is outside the bundled MIST range {minimum_mh:+.3}..={maximum_mh:+.3}"
    )]
    OutsideMetallicityGrid {
        global_metallicity_mh: f64,
        minimum_mh: f64,
        maximum_mh: f64,
    },
    #[error(
        "age {age_gyr:.6} Gyr predates the first bundled track point at {first_age_gyr:.6} Gyr"
    )]
    AgeBeforeTrack { age_gyr: f64, first_age_gyr: f64 },
    #[error(
        "age {age_gyr:.6} Gyr is beyond the bundled PMS/main-sequence track ending at {track_end_age_gyr:.6} Gyr"
    )]
    PostMainSequenceNotBundled {
        age_gyr: f64,
        track_end_age_gyr: f64,
    },
    #[error(
        "massive MIST track ended at {track_end_age_gyr:.6} Gyr; core-collapse remnant classification is not bundled"
    )]
    UnsupportedCoreCollapse {
        last_current_mass_msun: f64,
        last_carbon_oxygen_core_mass_msun: f64,
        track_end_age_gyr: f64,
    },
    #[error(
        "white-dwarf progenitor track ends at post-AGB EEP {last_eep} without a temperature knee"
    )]
    PostAgbTrackIncomplete {
        last_eep: u16,
        last_current_mass_msun: f64,
        track_end_age_gyr: f64,
    },
    #[error(
        "MIST track ended early at EEP {last_eep} and does not reach a supported terminal handoff"
    )]
    TrackEndedBeforeExpectedEndpoint {
        last_eep: u16,
        last_current_mass_msun: f64,
        track_end_age_gyr: f64,
    },
}

#[derive(Debug, Clone)]
pub struct StellarEvolutionEvaluator {
    model: StellarEvolutionModel,
    masses: Vec<f64>,
    metallicities: Vec<f64>,
    white_dwarf_cooling: Option<WhiteDwarfCoolingEvaluator>,
}

impl StellarEvolutionEvaluator {
    pub fn new(model: StellarEvolutionModel) -> Result<Self, StellarEvolutionError> {
        if model.tracks.is_empty() {
            return Err(StellarEvolutionError::InvalidModel);
        }
        let mut masses: Vec<_> = model
            .tracks
            .iter()
            .map(|track| track.initial_mass_msun)
            .collect();
        let mut metallicities: Vec<_> = model
            .tracks
            .iter()
            .map(|track| track.global_metallicity_mh)
            .collect();
        sort_and_deduplicate_finite(&mut masses)?;
        sort_and_deduplicate_finite(&mut metallicities)?;

        for metallicity in &metallicities {
            for mass in &masses {
                let Some(track) = find_track(&model, *mass, *metallicity) else {
                    return Err(StellarEvolutionError::InvalidModel);
                };
                if !valid_evolution_track(track) {
                    return Err(StellarEvolutionError::InvalidModel);
                }
            }
        }
        Ok(Self {
            model,
            masses,
            metallicities,
            white_dwarf_cooling: None,
        })
    }

    pub fn with_white_dwarf_cooling(
        mut self,
        model: WhiteDwarfCoolingModel,
    ) -> Result<Self, WhiteDwarfCoolingError> {
        self.white_dwarf_cooling = Some(WhiteDwarfCoolingEvaluator::new(model)?);
        Ok(self)
    }

    pub fn evaluate(
        &self,
        initial_mass_msun: f64,
        age_gyr: f64,
        chemistry: StellarChemistry,
    ) -> Result<StellarEvolutionSnapshot, StellarEvolutionError> {
        validate_evolution_input(initial_mass_msun, age_gyr, chemistry)?;
        let (mass_low, mass_high, mass_fraction) = bracket_grid(&self.masses, initial_mass_msun)
            .ok_or_else(|| StellarEvolutionError::OutsideMassGrid {
                initial_mass_msun,
                minimum_mass_msun: self.masses[0],
                maximum_mass_msun: *self.masses.last().expect("non-empty grid"),
            })?;
        let (metallicity_low, metallicity_high, metallicity_fraction) =
            bracket_grid(&self.metallicities, chemistry.global_metallicity_mh).ok_or_else(
                || StellarEvolutionError::OutsideMetallicityGrid {
                    global_metallicity_mh: chemistry.global_metallicity_mh,
                    minimum_mh: self.metallicities[0],
                    maximum_mh: *self.metallicities.last().expect("non-empty grid"),
                },
            )?;

        let tracks = [
            find_track(&self.model, mass_low, metallicity_low).expect("validated grid"),
            find_track(&self.model, mass_high, metallicity_low).expect("validated grid"),
            find_track(&self.model, mass_low, metallicity_high).expect("validated grid"),
            find_track(&self.model, mass_high, metallicity_high).expect("validated grid"),
        ];
        let common_branch = tracks
            .iter()
            .all(|track| track.branch == tracks[0].branch)
            .then_some(tracks[0].branch);
        let common_eeps: Vec<_> = tracks[0]
            .points
            .iter()
            .map(|point| point.eep)
            .filter(|eep| {
                tracks[1..]
                    .iter()
                    .all(|track| track.points.iter().any(|point| point.eep == *eep))
            })
            .collect();
        let virtual_points: Vec<_> = common_eeps
            .into_iter()
            .map(|eep| {
                interpolate_grid_point(
                    tracks.map(|track| {
                        *track
                            .points
                            .iter()
                            .find(|point| point.eep == eep)
                            .expect("EEP belongs to common prefix")
                    }),
                    mass_fraction,
                    metallicity_fraction,
                )
            })
            .collect();
        let white_dwarf_handoff = virtual_points
            .iter()
            .enumerate()
            .filter(|(_, point)| point.eep >= 1409.0 && point.phase == 6)
            .max_by(|(_, left), (_, right)| {
                left.log10_effective_temperature_k
                    .total_cmp(&right.log10_effective_temperature_k)
            })
            .filter(|(index, _)| *index + 1 < virtual_points.len())
            .map(|(_, point)| *point);
        let zams_age_gyr = virtual_points
            .iter()
            .find(|point| point.eep == 202.0)
            .expect("validated EEP grid")
            .age_gyr;
        let tams_age_gyr = virtual_points
            .iter()
            .find(|point| point.eep == 454.0)
            .expect("validated EEP grid")
            .age_gyr;
        let main_sequence_lifetime_gyr = tams_age_gyr - zams_age_gyr;
        let first_age_gyr = virtual_points[0].age_gyr;
        let track_end_age_gyr = virtual_points.last().expect("validated track").age_gyr;
        let track_end_tolerance_gyr = (track_end_age_gyr.abs() * 1e-12).max(1e-15);
        if age_gyr < first_age_gyr {
            return Err(StellarEvolutionError::AgeBeforeTrack {
                age_gyr,
                first_age_gyr,
            });
        }
        if age_gyr > track_end_age_gyr + track_end_tolerance_gyr {
            let last = *virtual_points.last().expect("validated track");
            if let Some(handoff) = white_dwarf_handoff {
                let mut quality_flags =
                    vec![StellarEvolutionQualityFlag::WhiteDwarfCoolingNotBundled];
                if chemistry.alpha_enhancement_alpha_fe.abs() > 1e-12 {
                    quality_flags.push(StellarEvolutionQualityFlag::AlphaProjectedToSolarScaled);
                }
                let mut snapshot = StellarEvolutionSnapshot {
                    model_version: self.model.model_version,
                    initial_mass_msun,
                    age_gyr,
                    source_metallicity_coordinate_mh: chemistry.global_metallicity_mh,
                    state: EvolutionaryState::WhiteDwarf,
                    raw_eep: last.eep,
                    raw_phase: last.phase,
                    zams_age_gyr,
                    tams_age_gyr,
                    main_sequence_lifetime_gyr,
                    fractional_main_sequence_age: None,
                    white_dwarf_handoff_age_gyr: Some(handoff.age_gyr),
                    cooling_age_gyr: Some(age_gyr - handoff.age_gyr),
                    remnant_mass_msun: Some(handoff.current_mass_msun),
                    white_dwarf_cooling_model_version: None,
                    current_mass_msun: handoff.current_mass_msun,
                    luminosity_lsun: None,
                    radius_rsun: None,
                    effective_temperature_k: None,
                    surface_gravity_log10_cgs: None,
                    quality_flags,
                };
                self.populate_white_dwarf_cooling(&mut snapshot);
                return Ok(snapshot);
            }
            if tracks
                .iter()
                .all(|track| track.branch == StellarEvolutionTrackBranch::MassiveBurning)
                && last.eep >= 808.0
            {
                return Err(StellarEvolutionError::UnsupportedCoreCollapse {
                    last_current_mass_msun: last.current_mass_msun,
                    last_carbon_oxygen_core_mass_msun: last.carbon_oxygen_core_mass_msun,
                    track_end_age_gyr,
                });
            }
            if tracks
                .iter()
                .all(|track| track.branch == StellarEvolutionTrackBranch::MassiveBurning)
            {
                return Err(StellarEvolutionError::TrackEndedBeforeExpectedEndpoint {
                    last_eep: last.eep.round() as u16,
                    last_current_mass_msun: last.current_mass_msun,
                    track_end_age_gyr,
                });
            }
            if tracks
                .iter()
                .all(|track| track.branch == StellarEvolutionTrackBranch::WhiteDwarfProgenitor)
                && (last.eep - 1409.0).abs() < 1e-9
                && white_dwarf_handoff.is_none()
            {
                return Err(StellarEvolutionError::PostAgbTrackIncomplete {
                    last_eep: 1409,
                    last_current_mass_msun: last.current_mass_msun,
                    track_end_age_gyr,
                });
            }
            return Err(StellarEvolutionError::PostMainSequenceNotBundled {
                age_gyr,
                track_end_age_gyr,
            });
        }
        let evaluation_age_gyr = age_gyr.min(track_end_age_gyr);

        let upper_index =
            virtual_points.partition_point(|point| point.age_gyr < evaluation_age_gyr);
        let (lower, upper, age_fraction) = if upper_index == 0 {
            (virtual_points[0], virtual_points[0], 0.0)
        } else if upper_index == virtual_points.len() {
            let last = *virtual_points.last().expect("validated track");
            (last, last, 0.0)
        } else {
            let lower = virtual_points[upper_index - 1];
            let upper = virtual_points[upper_index];
            (
                lower,
                upper,
                (evaluation_age_gyr - lower.age_gyr) / (upper.age_gyr - lower.age_gyr),
            )
        };
        let evaluated = interpolate_evolution_point(lower, upper, age_fraction);
        let state = match evaluated.phase {
            -1 => EvolutionaryState::PreMainSequence,
            0 => EvolutionaryState::MainSequence,
            3 if evaluated.eep < 631.0 => EvolutionaryState::HeliumIgnitionTransition,
            2 => EvolutionaryState::SubgiantAndRedGiantBranch,
            3 => EvolutionaryState::CoreHeliumBurning,
            4 => EvolutionaryState::EarlyAsymptoticGiantBranch,
            5 if common_branch == Some(StellarEvolutionTrackBranch::MassiveBurning)
                && evaluated.eep >= 808.0 =>
            {
                EvolutionaryState::AdvancedBurningTrackEnd
            }
            5 => EvolutionaryState::ThermallyPulsingAsymptoticGiantBranch,
            9 => EvolutionaryState::WolfRayet,
            6 if white_dwarf_handoff.is_some_and(|handoff| evaluated.eep >= handoff.eep) => {
                EvolutionaryState::WhiteDwarf
            }
            6 => EvolutionaryState::PostAsymptoticGiantBranch,
            _ => return Err(StellarEvolutionError::InvalidModel),
        };
        let fractional_main_sequence_age = (state == EvolutionaryState::MainSequence)
            .then(|| ((age_gyr - zams_age_gyr) / main_sequence_lifetime_gyr).clamp(0.0, 1.0));
        let mut quality_flags = Vec::new();
        if chemistry.alpha_enhancement_alpha_fe.abs() > 1e-12 {
            quality_flags.push(StellarEvolutionQualityFlag::AlphaProjectedToSolarScaled);
        }
        if state == EvolutionaryState::WhiteDwarf {
            quality_flags.push(StellarEvolutionQualityFlag::WhiteDwarfCoolingNotBundled);
        }
        let white_dwarf_handoff_age_gyr = white_dwarf_handoff.map(|point| point.age_gyr);
        let cooling_age_gyr = (state == EvolutionaryState::WhiteDwarf)
            .then(|| age_gyr - white_dwarf_handoff_age_gyr.expect("white dwarf has a handoff"));
        let remnant_mass_msun = (state == EvolutionaryState::WhiteDwarf).then(|| {
            white_dwarf_handoff
                .expect("white dwarf has a handoff")
                .current_mass_msun
        });
        let has_photospheric_observables = state != EvolutionaryState::WhiteDwarf;
        let current_mass_msun = remnant_mass_msun.unwrap_or(evaluated.current_mass_msun);

        let mut snapshot = StellarEvolutionSnapshot {
            model_version: self.model.model_version,
            initial_mass_msun,
            age_gyr,
            source_metallicity_coordinate_mh: chemistry.global_metallicity_mh,
            state,
            raw_eep: evaluated.eep,
            raw_phase: evaluated.phase,
            zams_age_gyr,
            tams_age_gyr,
            main_sequence_lifetime_gyr,
            fractional_main_sequence_age,
            white_dwarf_handoff_age_gyr,
            cooling_age_gyr,
            remnant_mass_msun,
            white_dwarf_cooling_model_version: None,
            current_mass_msun,
            luminosity_lsun: has_photospheric_observables
                .then(|| 10_f64.powf(evaluated.log10_luminosity_lsun)),
            radius_rsun: has_photospheric_observables
                .then(|| 10_f64.powf(evaluated.log10_radius_rsun)),
            effective_temperature_k: has_photospheric_observables
                .then(|| 10_f64.powf(evaluated.log10_effective_temperature_k)),
            surface_gravity_log10_cgs: has_photospheric_observables
                .then_some(evaluated.surface_gravity_log10_cgs),
            quality_flags,
        };
        self.populate_white_dwarf_cooling(&mut snapshot);
        Ok(snapshot)
    }

    fn populate_white_dwarf_cooling(&self, snapshot: &mut StellarEvolutionSnapshot) {
        if snapshot.state != EvolutionaryState::WhiteDwarf {
            return;
        }
        let Some(evaluator) = &self.white_dwarf_cooling else {
            return;
        };
        let result = evaluator.evaluate(
            snapshot.current_mass_msun,
            snapshot
                .cooling_age_gyr
                .expect("white dwarf has a cooling age"),
        );
        match result {
            Ok(cooling) => {
                snapshot.luminosity_lsun = Some(cooling.luminosity_lsun);
                snapshot.radius_rsun = Some(cooling.radius_rsun);
                snapshot.effective_temperature_k = Some(cooling.effective_temperature_k);
                snapshot.surface_gravity_log10_cgs = Some(cooling.surface_gravity_log10_cgs);
                snapshot.white_dwarf_cooling_model_version = Some(cooling.model_version);
                snapshot.quality_flags.retain(|flag| {
                    *flag != StellarEvolutionQualityFlag::WhiteDwarfCoolingNotBundled
                });
                snapshot
                    .quality_flags
                    .push(StellarEvolutionQualityFlag::MontrealCoolingHybridModel);
                if cooling.young_cooling_zero_point_uncertain {
                    snapshot.quality_flags.push(
                        StellarEvolutionQualityFlag::YoungWhiteDwarfCoolingZeroPointUncertain,
                    );
                }
            }
            Err(_) => snapshot
                .quality_flags
                .push(StellarEvolutionQualityFlag::WhiteDwarfCoolingOutsideModelCoverage),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct InterpolatedEvolutionPoint {
    eep: f64,
    age_gyr: f64,
    current_mass_msun: f64,
    carbon_oxygen_core_mass_msun: f64,
    log10_luminosity_lsun: f64,
    log10_effective_temperature_k: f64,
    log10_radius_rsun: f64,
    surface_gravity_log10_cgs: f64,
    phase: i8,
}

fn sort_and_deduplicate_finite(values: &mut Vec<f64>) -> Result<(), StellarEvolutionError> {
    if values.iter().any(|value| !value.is_finite()) {
        return Err(StellarEvolutionError::InvalidModel);
    }
    values.sort_by(|left, right| left.total_cmp(right));
    values.dedup_by(|left, right| (*left - *right).abs() < 1e-12);
    Ok(())
}

fn valid_evolution_track(track: &StellarEvolutionTrack) -> bool {
    track.initial_mass_msun.is_finite()
        && track.initial_mass_msun > 0.0
        && track.global_metallicity_mh.is_finite()
        && !track.primary_eeps.is_empty()
        && track.primary_eeps.windows(2).all(|pair| pair[0] < pair[1])
        && track.points.len() >= 2
        && track.points.iter().any(|point| point.eep == 202)
        && track.points.iter().any(|point| point.eep == 454)
        && track
            .points
            .windows(2)
            .all(|pair| pair[0].eep < pair[1].eep && pair[0].age_gyr < pair[1].age_gyr)
        && track.points.iter().all(|point| {
            point.age_gyr.is_finite()
                && point.age_gyr >= 0.0
                && point.current_mass_msun.is_finite()
                && point.current_mass_msun > 0.0
                && point.current_mass_msun <= track.initial_mass_msun * (1.0 + 1e-5)
                && point.carbon_oxygen_core_mass_msun.is_finite()
                && point.carbon_oxygen_core_mass_msun >= 0.0
                && point.carbon_oxygen_core_mass_msun <= point.current_mass_msun * (1.0 + 1e-5)
                && point.log10_luminosity_lsun.is_finite()
                && point.log10_effective_temperature_k.is_finite()
                && point.log10_radius_rsun.is_finite()
                && point.surface_gravity_log10_cgs.is_finite()
        })
}

fn validate_evolution_input(
    initial_mass_msun: f64,
    age_gyr: f64,
    chemistry: StellarChemistry,
) -> Result<(), StellarEvolutionError> {
    if !initial_mass_msun.is_finite() || initial_mass_msun <= 0.0 {
        return Err(StellarEvolutionError::InvalidInput {
            field: "initial_mass_msun",
        });
    }
    if !age_gyr.is_finite() || age_gyr < 0.0 {
        return Err(StellarEvolutionError::InvalidInput { field: "age_gyr" });
    }
    for (field, value) in [
        ("iron_abundance_feh", chemistry.iron_abundance_feh),
        (
            "alpha_enhancement_alpha_fe",
            chemistry.alpha_enhancement_alpha_fe,
        ),
        ("global_metallicity_mh", chemistry.global_metallicity_mh),
        (
            "hydrogen_mass_fraction_x",
            chemistry.hydrogen_mass_fraction_x,
        ),
        ("helium_mass_fraction_y", chemistry.helium_mass_fraction_y),
        ("metal_mass_fraction_z", chemistry.metal_mass_fraction_z),
    ] {
        if !value.is_finite() {
            return Err(StellarEvolutionError::InvalidInput { field });
        }
    }
    if chemistry.hydrogen_mass_fraction_x <= 0.0
        || chemistry.helium_mass_fraction_y <= 0.0
        || chemistry.metal_mass_fraction_z <= 0.0
        || (chemistry.hydrogen_mass_fraction_x
            + chemistry.helium_mass_fraction_y
            + chemistry.metal_mass_fraction_z
            - 1.0)
            .abs()
            > 2e-3
    {
        return Err(StellarEvolutionError::InvalidInput {
            field: "chemical_mass_fractions",
        });
    }
    Ok(())
}

fn find_track(
    model: &StellarEvolutionModel,
    initial_mass_msun: f64,
    global_metallicity_mh: f64,
) -> Option<&StellarEvolutionTrack> {
    model.tracks.iter().find(|track| {
        (track.initial_mass_msun - initial_mass_msun).abs() < 1e-12
            && (track.global_metallicity_mh - global_metallicity_mh).abs() < 1e-12
    })
}

fn bracket_grid(values: &[f64], requested: f64) -> Option<(f64, f64, f64)> {
    if requested < values[0] || requested > *values.last()? {
        return None;
    }
    let upper_index = values.partition_point(|value| *value < requested);
    if upper_index == values.len() {
        let value = values[values.len() - 1];
        return Some((value, value, 0.0));
    }
    if (values[upper_index] - requested).abs() < 1e-12 || upper_index == 0 {
        let value = values[upper_index];
        return Some((value, value, 0.0));
    }
    let lower = values[upper_index - 1];
    let upper = values[upper_index];
    Some((lower, upper, (requested - lower) / (upper - lower)))
}

fn interpolate_grid_point(
    points: [StellarEvolutionTrackPoint; 4],
    mass_fraction: f64,
    metallicity_fraction: f64,
) -> InterpolatedEvolutionPoint {
    let bilinear = |values: [f64; 4]| {
        let low_metallicity = lerp(values[0], values[1], mass_fraction);
        let high_metallicity = lerp(values[2], values[3], mass_fraction);
        lerp(low_metallicity, high_metallicity, metallicity_fraction)
    };
    InterpolatedEvolutionPoint {
        eep: points[0].eep as f64,
        age_gyr: 10_f64.powf(bilinear(points.map(|point| point.age_gyr.log10()))),
        current_mass_msun: bilinear(points.map(|point| point.current_mass_msun)),
        carbon_oxygen_core_mass_msun: bilinear(
            points.map(|point| point.carbon_oxygen_core_mass_msun),
        ),
        log10_luminosity_lsun: bilinear(points.map(|point| point.log10_luminosity_lsun)),
        log10_effective_temperature_k: bilinear(
            points.map(|point| point.log10_effective_temperature_k),
        ),
        log10_radius_rsun: bilinear(points.map(|point| point.log10_radius_rsun)),
        surface_gravity_log10_cgs: bilinear(points.map(|point| point.surface_gravity_log10_cgs)),
        phase: points[0].phase,
    }
}

fn interpolate_evolution_point(
    lower: InterpolatedEvolutionPoint,
    upper: InterpolatedEvolutionPoint,
    fraction: f64,
) -> InterpolatedEvolutionPoint {
    InterpolatedEvolutionPoint {
        eep: lerp(lower.eep, upper.eep, fraction),
        age_gyr: lerp(lower.age_gyr, upper.age_gyr, fraction),
        current_mass_msun: lerp(lower.current_mass_msun, upper.current_mass_msun, fraction),
        carbon_oxygen_core_mass_msun: lerp(
            lower.carbon_oxygen_core_mass_msun,
            upper.carbon_oxygen_core_mass_msun,
            fraction,
        ),
        log10_luminosity_lsun: lerp(
            lower.log10_luminosity_lsun,
            upper.log10_luminosity_lsun,
            fraction,
        ),
        log10_effective_temperature_k: lerp(
            lower.log10_effective_temperature_k,
            upper.log10_effective_temperature_k,
            fraction,
        ),
        log10_radius_rsun: lerp(lower.log10_radius_rsun, upper.log10_radius_rsun, fraction),
        surface_gravity_log10_cgs: lerp(
            lower.surface_gravity_log10_cgs,
            upper.surface_gravity_log10_cgs,
            fraction,
        ),
        phase: if fraction < 1.0 {
            lower.phase
        } else {
            upper.phase
        },
    }
}

fn lerp(lower: f64, upper: f64, fraction: f64) -> f64 {
    lower + (upper - lower) * fraction
}
