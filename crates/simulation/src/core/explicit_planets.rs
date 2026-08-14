//! Explicit planet realization and candidate provenance.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplicitPlanetModelVersion {
    ObservationalDomainsV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExplicitSmallPlanetBin {
    pub minimum_radius_rearth: f64,
    pub maximum_radius_rearth: f64,
    pub minimum_period_days: f64,
    pub maximum_period_days: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExplicitDopplerGiantModel {
    pub minimum_host_effective_temperature_k: f64,
    pub maximum_host_effective_temperature_k: f64,
    pub minimum_mass_mjup: f64,
    pub maximum_mass_mjup: f64,
    pub mass_log_density_exponent: f64,
    pub minimum_period_days: f64,
    pub maximum_period_days: f64,
    pub period_log_density_exponent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExplicitPlanetOccurrenceCell {
    pub minimum_radius_rearth: f64,
    pub maximum_radius_rearth: f64,
    pub minimum_period_days: f64,
    pub maximum_period_days: f64,
    pub occurrence_weight_percent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplicitPlanetModel {
    pub model_version: ExplicitPlanetModelVersion,
    pub warm_super_earth: ExplicitSmallPlanetBin,
    pub warm_sub_neptune: ExplicitSmallPlanetBin,
    pub m_dwarf_occurrence_cells: Vec<ExplicitPlanetOccurrenceCell>,
    pub m_dwarf_sub_earth_occurrence_cells: Vec<ExplicitPlanetOccurrenceCell>,
    pub doppler_giant: ExplicitDopplerGiantModel,
}

impl Default for ExplicitPlanetModel {
    fn default() -> Self {
        Self {
            model_version: ExplicitPlanetModelVersion::ObservationalDomainsV1,
            warm_super_earth: ExplicitSmallPlanetBin {
                minimum_radius_rearth: 1.0,
                maximum_radius_rearth: 1.7,
                minimum_period_days: 10.0,
                maximum_period_days: 100.0,
            },
            warm_sub_neptune: ExplicitSmallPlanetBin {
                minimum_radius_rearth: 1.7,
                maximum_radius_rearth: 4.0,
                minimum_period_days: 10.0,
                maximum_period_days: 100.0,
            },
            m_dwarf_occurrence_cells: default_m_dwarf_occurrence_cells(),
            m_dwarf_sub_earth_occurrence_cells: default_m_dwarf_sub_earth_occurrence_cells(),
            doppler_giant: ExplicitDopplerGiantModel {
                minimum_host_effective_temperature_k: 4_700.0,
                maximum_host_effective_temperature_k: 6_500.0,
                minimum_mass_mjup: 0.3,
                maximum_mass_mjup: 10.0,
                mass_log_density_exponent: -0.31,
                minimum_period_days: 2.0,
                maximum_period_days: 2_000.0,
                period_log_density_exponent: 0.26,
            },
        }
    }
}

fn default_m_dwarf_sub_earth_occurrence_cells() -> Vec<ExplicitPlanetOccurrenceCell> {
    [(0.5, 1.7, 1.38), (1.7, 5.5, 8.42), (5.5, 18.2, 20.59)]
        .into_iter()
        .map(
            |(minimum_period_days, maximum_period_days, occurrence_weight_percent)| {
                ExplicitPlanetOccurrenceCell {
                    minimum_radius_rearth: 0.5,
                    maximum_radius_rearth: 1.0,
                    minimum_period_days,
                    maximum_period_days,
                    occurrence_weight_percent,
                }
            },
        )
        .collect()
}

fn default_m_dwarf_occurrence_cells() -> Vec<ExplicitPlanetOccurrenceCell> {
    let radius_bins = [
        (1.0, 1.5, [1.95, 9.94, 0.0, 26.85, 28.85]),
        (1.5, 2.0, [0.41, 4.15, 0.0, 24.59, 19.98]),
        (2.0, 2.5, [0.0, 2.72, 18.73, 27.58, 18.08]),
        (2.5, 3.0, [0.0, 1.59, 8.29, 14.51, 8.61]),
        (3.0, 3.5, [0.0, 0.65, 3.25, 3.37, 1.97]),
        (3.5, 4.0, [0.0, 0.38, 1.05, 0.56, 0.0]),
    ];
    let period_bins = [
        (0.5, 1.7),
        (1.7, 5.5),
        (5.5, 18.2),
        (18.2, 60.3),
        (60.3, 200.0),
    ];
    radius_bins
        .into_iter()
        .flat_map(|(minimum_radius_rearth, maximum_radius_rearth, weights)| {
            period_bins.into_iter().zip(weights).filter_map(
                move |((minimum_period_days, maximum_period_days), occurrence_weight_percent)| {
                    (occurrence_weight_percent > 0.0).then_some(ExplicitPlanetOccurrenceCell {
                        minimum_radius_rearth,
                        maximum_radius_rearth,
                        minimum_period_days,
                        maximum_period_days,
                        occurrence_weight_percent,
                    })
                },
            )
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplicitPlanetSourceChannel {
    FgkWarmSuperEarth,
    FgkWarmSubNeptune,
    MDwarfSmallPlanet,
    FgkDopplerGiant,
    MDwarfSubEarth,
}

impl ExplicitPlanetSourceChannel {
    fn stable_tag(self) -> u8 {
        match self {
            Self::FgkWarmSuperEarth => 0,
            Self::FgkWarmSubNeptune => 1,
            Self::MDwarfSmallPlanet => 2,
            Self::FgkDopplerGiant => 3,
            Self::MDwarfSubEarth => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExplicitPlanetProperties {
    TransitRadius { radius_rearth: f64 },
    DopplerMinimumMass { minimum_mass_mjup: f64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplicitPlanetQualityFlag {
    WithinBinLogUniformApproximation,
    FgkHostTemperatureProxy,
    DopplerMinimumMassIsMassTimesSinInclination,
    OneGiantFromAtLeastOneOccurrenceGate,
    PlanetInteractionsNotModeled,
    MDwarfOccurrenceGridRenormalizedToAggregateCount,
    MDwarfSubEarthOccurrenceLimitedToMeasuredCells,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExplicitPlanetCandidate {
    pub id: u64,
    pub host_member_id: u64,
    pub source_channel: ExplicitPlanetSourceChannel,
    pub properties: ExplicitPlanetProperties,
    pub period_days: f64,
    pub semimajor_axis_au: f64,
    pub quality_flags: Vec<ExplicitPlanetQualityFlag>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RejectedPlanetCandidateReason {
    OutsideCircumstellarStabilityZone {
        semimajor_axis_au: f64,
        conservative_outer_limit_au: f64,
    },
    StabilityZoneUnavailable(PlanetaryStabilityError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RejectedPlanetCandidate {
    pub candidate: ExplicitPlanetCandidate,
    pub reason: RejectedPlanetCandidateReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnresolvedPlanetPopulation {
    MDwarfSmallPlanets { count: u32 },
    GiantPlanetPropertiesUnavailable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanetarySystemRealization {
    pub model_version: ExplicitPlanetModelVersion,
    pub host_member_id: u64,
    pub accepted_planets: Vec<ExplicitPlanetCandidate>,
    pub rejected_candidates: Vec<RejectedPlanetCandidate>,
    pub unresolved_populations: Vec<UnresolvedPlanetPopulation>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ExplicitPlanetModelError {
    #[error("explicit-planet model is invalid")]
    InvalidModel,
}

#[derive(Debug, Clone)]
pub(crate) struct ExplicitPlanetGenerator {
    model: ExplicitPlanetModel,
}

impl ExplicitPlanetGenerator {
    pub(crate) fn new(model: ExplicitPlanetModel) -> Result<Self, ExplicitPlanetModelError> {
        if !valid_explicit_planet_model(&model) {
            return Err(ExplicitPlanetModelError::InvalidModel);
        }
        Ok(Self { model })
    }

    pub(crate) fn generate(
        &self,
        seed: u64,
        system_id: u64,
        host_member_id: u64,
        evolution: &Result<StellarEvolutionSnapshot, StellarEvolutionError>,
        occurrence: &PlanetPopulationSummary,
        stability_zone: &Result<CircumstellarSTypeStabilityZone, PlanetaryStabilityError>,
    ) -> PlanetarySystemRealization {
        let mut candidates = Vec::new();
        let mut unresolved_populations = Vec::new();
        if let Ok(small_planets) = &occurrence.small_planets {
            match small_planets {
                SmallPlanetOccurrence::FgkWarm {
                    warm_super_earth_count,
                    warm_sub_neptune_count,
                } => {
                    self.generate_small_candidates(
                        seed,
                        system_id,
                        host_member_id,
                        *warm_super_earth_count,
                        ExplicitPlanetSourceChannel::FgkWarmSuperEarth,
                        self.model.warm_super_earth,
                        evolution,
                        &mut candidates,
                    );
                    self.generate_small_candidates(
                        seed,
                        system_id,
                        host_member_id,
                        *warm_sub_neptune_count,
                        ExplicitPlanetSourceChannel::FgkWarmSubNeptune,
                        self.model.warm_sub_neptune,
                        evolution,
                        &mut candidates,
                    );
                }
                SmallPlanetOccurrence::MDwarfAggregate {
                    small_planet_count,
                    sub_earth_count,
                } => {
                    self.generate_m_dwarf_candidates(
                        seed,
                        system_id,
                        host_member_id,
                        *small_planet_count,
                        evolution,
                        &mut candidates,
                    );
                    self.generate_m_dwarf_sub_earth_candidates(
                        seed,
                        system_id,
                        host_member_id,
                        *sub_earth_count,
                        evolution,
                        &mut candidates,
                    );
                }
            }
        }
        if occurrence
            .giant_planets
            .as_ref()
            .is_ok_and(|giant| giant.has_at_least_one_cps_giant)
        {
            if let Ok(snapshot) = evolution
                && snapshot.effective_temperature_k.is_some_and(|temperature| {
                    (self
                        .model
                        .doppler_giant
                        .minimum_host_effective_temperature_k
                        ..=self
                            .model
                            .doppler_giant
                            .maximum_host_effective_temperature_k)
                        .contains(&temperature)
                })
            {
                candidates.push(self.generate_giant_candidate(
                    seed,
                    system_id,
                    host_member_id,
                    snapshot.current_mass_msun,
                ));
            } else {
                unresolved_populations
                    .push(UnresolvedPlanetPopulation::GiantPlanetPropertiesUnavailable);
            }
        }
        let mut accepted_planets = Vec::new();
        let mut rejected_candidates = Vec::new();
        for candidate in candidates {
            match candidate_acceptance(&candidate, stability_zone) {
                Ok(()) => accepted_planets.push(candidate),
                Err(reason) => {
                    rejected_candidates.push(RejectedPlanetCandidate { candidate, reason })
                }
            }
        }
        accepted_planets.sort_by(|left, right| left.period_days.total_cmp(&right.period_days));
        PlanetarySystemRealization {
            model_version: self.model.model_version,
            host_member_id,
            accepted_planets,
            rejected_candidates,
            unresolved_populations,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_small_candidates(
        &self,
        seed: u64,
        system_id: u64,
        host_member_id: u64,
        count: u32,
        channel: ExplicitPlanetSourceChannel,
        bin: ExplicitSmallPlanetBin,
        evolution: &Result<StellarEvolutionSnapshot, StellarEvolutionError>,
        output: &mut Vec<ExplicitPlanetCandidate>,
    ) {
        let Ok(snapshot) = evolution else {
            return;
        };
        for index in 0..count {
            let id = stable_explicit_planet_id(system_id, host_member_id, channel, index);
            let radius_rearth = sample_log_uniform_for_entity(
                seed,
                b"explicit_planet/small_radius/v1",
                id,
                bin.minimum_radius_rearth,
                bin.maximum_radius_rearth,
            );
            let period_days = sample_log_uniform_for_entity(
                seed,
                b"explicit_planet/small_period/v1",
                id,
                bin.minimum_period_days,
                bin.maximum_period_days,
            );
            output.push(ExplicitPlanetCandidate {
                id,
                host_member_id,
                source_channel: channel,
                properties: ExplicitPlanetProperties::TransitRadius { radius_rearth },
                period_days,
                semimajor_axis_au: semimajor_axis_from_period_days(
                    period_days,
                    snapshot.current_mass_msun,
                ),
                quality_flags: vec![
                    ExplicitPlanetQualityFlag::WithinBinLogUniformApproximation,
                    ExplicitPlanetQualityFlag::PlanetInteractionsNotModeled,
                ],
            });
        }
    }

    fn generate_giant_candidate(
        &self,
        seed: u64,
        system_id: u64,
        host_member_id: u64,
        host_mass_msun: f64,
    ) -> ExplicitPlanetCandidate {
        let channel = ExplicitPlanetSourceChannel::FgkDopplerGiant;
        let id = stable_explicit_planet_id(system_id, host_member_id, channel, 0);
        let model = self.model.doppler_giant;
        let minimum_mass_mjup = sample_log_power_law_for_entity(
            seed,
            b"explicit_planet/giant_minimum_mass/v1",
            id,
            model.minimum_mass_mjup,
            model.maximum_mass_mjup,
            model.mass_log_density_exponent,
        );
        let period_days = sample_log_power_law_for_entity(
            seed,
            b"explicit_planet/giant_period/v1",
            id,
            model.minimum_period_days,
            model.maximum_period_days,
            model.period_log_density_exponent,
        );
        ExplicitPlanetCandidate {
            id,
            host_member_id,
            source_channel: channel,
            properties: ExplicitPlanetProperties::DopplerMinimumMass { minimum_mass_mjup },
            period_days,
            semimajor_axis_au: semimajor_axis_from_period_days(period_days, host_mass_msun),
            quality_flags: vec![
                ExplicitPlanetQualityFlag::FgkHostTemperatureProxy,
                ExplicitPlanetQualityFlag::DopplerMinimumMassIsMassTimesSinInclination,
                ExplicitPlanetQualityFlag::OneGiantFromAtLeastOneOccurrenceGate,
                ExplicitPlanetQualityFlag::PlanetInteractionsNotModeled,
            ],
        }
    }

    fn generate_m_dwarf_candidates(
        &self,
        seed: u64,
        system_id: u64,
        host_member_id: u64,
        count: u32,
        evolution: &Result<StellarEvolutionSnapshot, StellarEvolutionError>,
        output: &mut Vec<ExplicitPlanetCandidate>,
    ) {
        let Ok(snapshot) = evolution else {
            return;
        };
        let total_weight: f64 = self
            .model
            .m_dwarf_occurrence_cells
            .iter()
            .map(|cell| cell.occurrence_weight_percent)
            .sum();
        let channel = ExplicitPlanetSourceChannel::MDwarfSmallPlanet;
        for index in 0..count {
            let id = stable_explicit_planet_id(system_id, host_member_id, channel, index);
            let draw = domain_rng(seed, b"explicit_planet/m_dwarf_cell/v1", Some(id))
                .gen_range(0.0..total_weight);
            let mut cumulative = 0.0;
            let cell = self
                .model
                .m_dwarf_occurrence_cells
                .iter()
                .find(|cell| {
                    cumulative += cell.occurrence_weight_percent;
                    draw < cumulative
                })
                .copied()
                .unwrap_or_else(|| {
                    *self
                        .model
                        .m_dwarf_occurrence_cells
                        .last()
                        .expect("validated non-empty occurrence grid")
                });
            let radius_rearth = sample_log_uniform_for_entity(
                seed,
                b"explicit_planet/m_dwarf_radius/v1",
                id,
                cell.minimum_radius_rearth,
                cell.maximum_radius_rearth,
            );
            let period_days = sample_log_uniform_for_entity(
                seed,
                b"explicit_planet/m_dwarf_period/v1",
                id,
                cell.minimum_period_days,
                cell.maximum_period_days,
            );
            output.push(ExplicitPlanetCandidate {
                id,
                host_member_id,
                source_channel: channel,
                properties: ExplicitPlanetProperties::TransitRadius { radius_rearth },
                period_days,
                semimajor_axis_au: semimajor_axis_from_period_days(
                    period_days,
                    snapshot.current_mass_msun,
                ),
                quality_flags: vec![
                    ExplicitPlanetQualityFlag::WithinBinLogUniformApproximation,
                    ExplicitPlanetQualityFlag::MDwarfOccurrenceGridRenormalizedToAggregateCount,
                    ExplicitPlanetQualityFlag::PlanetInteractionsNotModeled,
                ],
            });
        }
    }

    fn generate_m_dwarf_sub_earth_candidates(
        &self,
        seed: u64,
        system_id: u64,
        host_member_id: u64,
        count: u32,
        evolution: &Result<StellarEvolutionSnapshot, StellarEvolutionError>,
        output: &mut Vec<ExplicitPlanetCandidate>,
    ) {
        let Ok(snapshot) = evolution else {
            return;
        };
        let cells = &self.model.m_dwarf_sub_earth_occurrence_cells;
        let total_weight: f64 = cells
            .iter()
            .map(|cell| cell.occurrence_weight_percent)
            .sum();
        let channel = ExplicitPlanetSourceChannel::MDwarfSubEarth;
        for index in 0..count {
            let id = stable_explicit_planet_id(system_id, host_member_id, channel, index);
            let draw = domain_rng(seed, b"explicit_planet/m_dwarf_sub_earth_cell/v1", Some(id))
                .gen_range(0.0..total_weight);
            let mut cumulative = 0.0;
            let cell = cells
                .iter()
                .find(|cell| {
                    cumulative += cell.occurrence_weight_percent;
                    draw < cumulative
                })
                .copied()
                .unwrap_or_else(|| *cells.last().expect("validated non-empty occurrence grid"));
            let radius_rearth = sample_log_uniform_for_entity(
                seed,
                b"explicit_planet/m_dwarf_sub_earth_radius/v1",
                id,
                cell.minimum_radius_rearth,
                cell.maximum_radius_rearth,
            );
            let period_days = sample_log_uniform_for_entity(
                seed,
                b"explicit_planet/m_dwarf_sub_earth_period/v1",
                id,
                cell.minimum_period_days,
                cell.maximum_period_days,
            );
            output.push(ExplicitPlanetCandidate {
                id,
                host_member_id,
                source_channel: channel,
                properties: ExplicitPlanetProperties::TransitRadius { radius_rearth },
                period_days,
                semimajor_axis_au: semimajor_axis_from_period_days(
                    period_days,
                    snapshot.current_mass_msun,
                ),
                quality_flags: vec![
                    ExplicitPlanetQualityFlag::WithinBinLogUniformApproximation,
                    ExplicitPlanetQualityFlag::MDwarfSubEarthOccurrenceLimitedToMeasuredCells,
                    ExplicitPlanetQualityFlag::PlanetInteractionsNotModeled,
                ],
            });
        }
    }
}

fn candidate_acceptance(
    candidate: &ExplicitPlanetCandidate,
    stability_zone: &Result<CircumstellarSTypeStabilityZone, PlanetaryStabilityError>,
) -> Result<(), RejectedPlanetCandidateReason> {
    match stability_zone {
        Ok(CircumstellarSTypeStabilityZone::UnboundedByStellarCompanion { .. }) => Ok(()),
        Ok(CircumstellarSTypeStabilityZone::CompanionLimited {
            fit_residual_lower_semimajor_axis_au,
            ..
        }) if candidate.semimajor_axis_au <= *fit_residual_lower_semimajor_axis_au => Ok(()),
        Ok(CircumstellarSTypeStabilityZone::CompanionLimited {
            fit_residual_lower_semimajor_axis_au,
            ..
        }) => Err(
            RejectedPlanetCandidateReason::OutsideCircumstellarStabilityZone {
                semimajor_axis_au: candidate.semimajor_axis_au,
                conservative_outer_limit_au: *fit_residual_lower_semimajor_axis_au,
            },
        ),
        Err(error) => Err(RejectedPlanetCandidateReason::StabilityZoneUnavailable(
            error.clone(),
        )),
    }
}

fn stable_explicit_planet_id(
    system_id: u64,
    host_member_id: u64,
    channel: ExplicitPlanetSourceChannel,
    index: u32,
) -> u64 {
    let mut input = Vec::with_capacity(64);
    input.extend_from_slice(b"star_sim/explicit_planet_id/v1");
    input.extend_from_slice(&system_id.to_le_bytes());
    input.extend_from_slice(&host_member_id.to_le_bytes());
    input.push(channel.stable_tag());
    input.extend_from_slice(&index.to_le_bytes());
    let hash = blake3::hash(&input);
    u64::from_le_bytes(
        hash.as_bytes()[..8]
            .try_into()
            .expect("eight-byte hash prefix"),
    )
}

fn sample_log_uniform_for_entity(
    seed: u64,
    domain: &[u8],
    entity_id: u64,
    minimum: f64,
    maximum: f64,
) -> f64 {
    let draw = domain_rng(seed, domain, Some(entity_id)).gen_range(0.0..1.0);
    minimum * (maximum / minimum).powf(draw)
}

fn sample_log_power_law_for_entity(
    seed: u64,
    domain: &[u8],
    entity_id: u64,
    minimum: f64,
    maximum: f64,
    exponent: f64,
) -> f64 {
    let draw = domain_rng(seed, domain, Some(entity_id)).gen_range(0.0..1.0);
    if exponent.abs() < 1e-12 {
        minimum * (maximum / minimum).powf(draw)
    } else {
        (minimum.powf(exponent) + draw * (maximum.powf(exponent) - minimum.powf(exponent)))
            .powf(1.0 / exponent)
    }
}

fn valid_explicit_planet_model(model: &ExplicitPlanetModel) -> bool {
    let bins = [model.warm_super_earth, model.warm_sub_neptune];
    bins.into_iter().all(|bin| {
        [
            bin.minimum_radius_rearth,
            bin.maximum_radius_rearth,
            bin.minimum_period_days,
            bin.maximum_period_days,
        ]
        .into_iter()
        .all(f64::is_finite)
            && bin.minimum_radius_rearth > 0.0
            && bin.minimum_radius_rearth < bin.maximum_radius_rearth
            && bin.minimum_period_days > 0.0
            && bin.minimum_period_days < bin.maximum_period_days
    }) && valid_occurrence_cells(&model.m_dwarf_occurrence_cells, 1.0, 4.0, 200.0)
        && valid_occurrence_cells(&model.m_dwarf_sub_earth_occurrence_cells, 0.5, 1.0, 18.2)
        && {
            let giant = model.doppler_giant;
            [
                giant.minimum_host_effective_temperature_k,
                giant.maximum_host_effective_temperature_k,
                giant.minimum_mass_mjup,
                giant.maximum_mass_mjup,
                giant.mass_log_density_exponent,
                giant.minimum_period_days,
                giant.maximum_period_days,
                giant.period_log_density_exponent,
            ]
            .into_iter()
            .all(f64::is_finite)
                && giant.minimum_host_effective_temperature_k
                    < giant.maximum_host_effective_temperature_k
                && giant.minimum_mass_mjup > 0.0
                && giant.minimum_mass_mjup < giant.maximum_mass_mjup
                && giant.minimum_period_days > 0.0
                && giant.minimum_period_days < giant.maximum_period_days
        }
}

fn valid_occurrence_cells(
    cells: &[ExplicitPlanetOccurrenceCell],
    minimum_radius_rearth: f64,
    maximum_radius_rearth: f64,
    maximum_period_days: f64,
) -> bool {
    !cells.is_empty()
        && cells.iter().all(|cell| {
            [
                cell.minimum_radius_rearth,
                cell.maximum_radius_rearth,
                cell.minimum_period_days,
                cell.maximum_period_days,
                cell.occurrence_weight_percent,
            ]
            .into_iter()
            .all(f64::is_finite)
                && cell.minimum_radius_rearth >= minimum_radius_rearth
                && cell.minimum_radius_rearth < cell.maximum_radius_rearth
                && cell.maximum_radius_rearth <= maximum_radius_rearth
                && cell.minimum_period_days > 0.0
                && cell.minimum_period_days < cell.maximum_period_days
                && cell.maximum_period_days <= maximum_period_days
                && cell.occurrence_weight_percent > 0.0
        })
}
