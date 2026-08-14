//! Stellar orbital hierarchy generation and orbital-scale models.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StellarOrbitalHierarchyModelVersion {
    StaticFieldHierarchyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MDwarfOrbitalScaleModel {
    pub minimum_primary_mass_msun: f64,
    pub source_minimum_primary_mass_msun: f64,
    pub source_maximum_primary_mass_msun: f64,
    pub maximum_primary_mass_msun: f64,
    pub log10_semimajor_axis_au_mean: f64,
    pub log10_semimajor_axis_au_standard_deviation: f64,
    pub maximum_semimajor_axis_au: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SolarTypeOrbitalScaleModel {
    pub minimum_primary_mass_msun: f64,
    pub maximum_primary_mass_msun: f64,
    pub source_anchor_minimum_primary_mass_msun: f64,
    pub source_anchor_maximum_primary_mass_msun: f64,
    pub log10_period_days_mean: f64,
    pub log10_period_days_standard_deviation: f64,
    pub minimum_log10_period_days: f64,
    pub maximum_log10_period_days: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StellarEccentricityModel {
    pub circularization_period_days: f64,
    pub absolute_maximum: f64,
    pub m_dwarf_uses_solar_proxy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QuadrupleTopologyModel {
    pub probability_two_plus_two: f64,
    pub probability_three_plus_one: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HierarchicalStabilityModel {
    pub mardling_aarseth_coefficient: f64,
    pub mutual_inclination_rad: f64,
    pub maximum_sampling_attempts: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LowMassContactRadiusProxyModel {
    pub minimum_mass_msun: f64,
    pub maximum_mass_msun_exclusive: f64,
    pub minimum_age_gyr: f64,
    pub old_field_age_gyr: f64,
    pub young_field_radius_rsun: f64,
    pub old_field_radius_rsun: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StellarOrbitalHierarchyModel {
    pub model_version: StellarOrbitalHierarchyModelVersion,
    pub m_dwarf_scale: MDwarfOrbitalScaleModel,
    pub solar_type_scale: SolarTypeOrbitalScaleModel,
    pub eccentricity: StellarEccentricityModel,
    pub quadruple_topology: QuadrupleTopologyModel,
    pub stability: HierarchicalStabilityModel,
    pub low_mass_contact_radius_proxy: LowMassContactRadiusProxyModel,
}

impl Default for StellarOrbitalHierarchyModel {
    fn default() -> Self {
        Self {
            model_version: StellarOrbitalHierarchyModelVersion::StaticFieldHierarchyV1,
            m_dwarf_scale: MDwarfOrbitalScaleModel {
                minimum_primary_mass_msun: 0.08,
                source_minimum_primary_mass_msun: 0.20,
                source_maximum_primary_mass_msun: 0.67,
                maximum_primary_mass_msun: 0.70,
                log10_semimajor_axis_au_mean: 1.68,
                log10_semimajor_axis_au_standard_deviation: 0.97,
                maximum_semimajor_axis_au: 10_000.0,
            },
            solar_type_scale: SolarTypeOrbitalScaleModel {
                minimum_primary_mass_msun: 0.70,
                maximum_primary_mass_msun: 1.30,
                source_anchor_minimum_primary_mass_msun: 0.80,
                source_anchor_maximum_primary_mass_msun: 1.20,
                log10_period_days_mean: 5.03,
                log10_period_days_standard_deviation: 2.28,
                minimum_log10_period_days: -0.3,
                maximum_log10_period_days: 10.0,
            },
            eccentricity: StellarEccentricityModel {
                circularization_period_days: 12.0,
                absolute_maximum: 0.99,
                m_dwarf_uses_solar_proxy: true,
            },
            quadruple_topology: QuadrupleTopologyModel {
                probability_two_plus_two: 0.74,
                probability_three_plus_one: 0.26,
            },
            stability: HierarchicalStabilityModel {
                mardling_aarseth_coefficient: 2.8,
                mutual_inclination_rad: 0.0,
                maximum_sampling_attempts: 128,
            },
            low_mass_contact_radius_proxy: LowMassContactRadiusProxyModel {
                minimum_mass_msun: 0.08,
                maximum_mass_msun_exclusive: 0.10,
                minimum_age_gyr: 0.10,
                old_field_age_gyr: 1.0,
                young_field_radius_rsun: 0.20,
                old_field_radius_rsun: 0.15,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadrupleTopology {
    TwoPlusTwo,
    ThreePlusOne,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelativeStellarOrbit {
    pub semimajor_axis_au: f64,
    pub period_days: f64,
    pub eccentricity: f64,
    pub combined_mass_msun: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StellarOrbitNode {
    Member {
        member_id: u64,
        mass_msun: f64,
    },
    RelativeOrbit {
        orbit: RelativeStellarOrbit,
        left: Box<StellarOrbitNode>,
        right: Box<StellarOrbitNode>,
    },
}

impl StellarOrbitNode {
    fn mass_msun(&self) -> f64 {
        match self {
            Self::Member { mass_msun, .. } => *mass_msun,
            Self::RelativeOrbit { orbit, .. } => orbit.combined_mass_msun,
        }
    }

    fn contains_member(&self, candidate_id: u64) -> bool {
        match self {
            Self::Member { member_id, .. } => *member_id == candidate_id,
            Self::RelativeOrbit { left, right, .. } => {
                left.contains_member(candidate_id) || right.contains_member(candidate_id)
            }
        }
    }

    pub(crate) fn member_mass_msun(&self, candidate_id: u64) -> Option<f64> {
        match self {
            Self::Member {
                member_id,
                mass_msun,
            } => (*member_id == candidate_id).then_some(*mass_msun),
            Self::RelativeOrbit { left, right, .. } => left
                .member_mass_msun(candidate_id)
                .or_else(|| right.member_mass_msun(candidate_id)),
        }
    }

    fn collect_member_ids(&self, output: &mut Vec<u64>) {
        match self {
            Self::Member { member_id, .. } => output.push(*member_id),
            Self::RelativeOrbit { left, right, .. } => {
                left.collect_member_ids(output);
                right.collect_member_ids(output);
            }
        }
    }

    fn collect_relative_orbits(&self, output: &mut Vec<RelativeStellarOrbit>) {
        if let Self::RelativeOrbit { orbit, left, right } = self {
            output.push(*orbit);
            left.collect_relative_orbits(output);
            right.collect_relative_orbits(output);
        }
    }

    fn nearest_companion_scale(&self, member_id: u64) -> Option<f64> {
        match self {
            Self::Member {
                member_id: candidate,
                ..
            } => (*candidate == member_id).then_some(f64::INFINITY),
            Self::RelativeOrbit { orbit, left, right } => {
                let child = if left.contains_member(member_id) {
                    left
                } else if right.contains_member(member_id) {
                    right
                } else {
                    return None;
                };
                Some(
                    child
                        .nearest_companion_scale(member_id)
                        .unwrap_or(f64::INFINITY)
                        .min(orbit.semimajor_axis_au),
                )
            }
        }
    }

    pub(crate) fn direct_parent_companion(&self, member_id: u64) -> Option<DirectParentCompanion> {
        let Self::RelativeOrbit { orbit, left, right } = self else {
            return None;
        };
        if matches!(left.as_ref(), Self::Member { member_id: id, .. } if *id == member_id) {
            return Some(DirectParentCompanion {
                orbit: *orbit,
                companion_mass_msun: right.mass_msun(),
                companion_is_subtree: !matches!(right.as_ref(), Self::Member { .. }),
            });
        }
        if matches!(right.as_ref(), Self::Member { member_id: id, .. } if *id == member_id) {
            return Some(DirectParentCompanion {
                orbit: *orbit,
                companion_mass_msun: left.mass_msun(),
                companion_is_subtree: !matches!(left.as_ref(), Self::Member { .. }),
            });
        }
        if left.contains_member(member_id) {
            left.direct_parent_companion(member_id)
        } else if right.contains_member(member_id) {
            right.direct_parent_companion(member_id)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DirectParentCompanion {
    pub(crate) orbit: RelativeStellarOrbit,
    pub(crate) companion_mass_msun: f64,
    pub(crate) companion_is_subtree: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StellarOrbitalHierarchyQualityFlag {
    MSeparationShapeDecoupledFromMassRatio,
    LowMassExtrapolation,
    HighMassExtrapolation,
    SolarPeriodShapeProxy,
    SolarEccentricityProxyForMDwarf,
    TopologyMassExtrapolation,
    HierarchyPairingEngineered,
    IndependentOrbitScaleDraws,
    CoplanarProgradeStabilityScreen,
    LowMassContactRadiusProxy,
    SolarCompositionRadiusProxy,
    HydrogenBurningBoundaryAmbiguous,
    BirthMassUsedAsDynamicalMass,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StellarOrbitalHierarchy {
    pub model_version: StellarOrbitalHierarchyModelVersion,
    pub root: StellarOrbitNode,
    pub quadruple_topology: Option<QuadrupleTopology>,
    pub quality_flags: Vec<StellarOrbitalHierarchyQualityFlag>,
}

impl StellarOrbitalHierarchy {
    pub fn member_ids(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        self.root.collect_member_ids(&mut ids);
        ids
    }

    pub fn nearest_companion_semimajor_axis_au(&self, member_id: u64) -> Option<f64> {
        self.root
            .nearest_companion_scale(member_id)
            .filter(|value| value.is_finite())
    }

    pub fn relative_orbits(&self) -> Vec<RelativeStellarOrbit> {
        let mut orbits = Vec::new();
        self.root.collect_relative_orbits(&mut orbits);
        orbits
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum StellarOrbitalHierarchyError {
    #[error("stellar-orbital hierarchy model is invalid")]
    InvalidModel,
    #[error("stellar system contains an unsupported number of members")]
    UnsupportedMemberCount,
    #[error("stellar evolution is unavailable for an orbital member")]
    MissingStellarEvolution,
    #[error("orbital evolution is not modeled for state {state:?}")]
    OrbitalEvolutionNotModeled { state: EvolutionaryState },
    #[error("stellar radius is unavailable for contact rejection")]
    MissingStellarRadius,
    #[error("primary mass {primary_mass_msun:.4} Msun is outside orbital-scale calibration")]
    OutsideOrbitalScaleCalibration { primary_mass_msun: f64 },
    #[error("no calibrated eccentricity model is available")]
    OutsideEccentricityCalibration,
    #[error("stable hierarchy sampling exhausted its deterministic attempt limit")]
    StableHierarchySamplingExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StellarOrbitMemberProvenance {
    EvolutionSnapshot,
    LowMassContactRadiusProxy {
        solar_composition_proxy: bool,
        hydrogen_burning_boundary_ambiguous: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StellarOrbitMemberInput {
    pub(crate) id: u64,
    pub(crate) role: StellarMemberRole,
    pub(crate) mass_msun: f64,
    pub(crate) radius_rsun: f64,
    pub(crate) provenance: StellarOrbitMemberProvenance,
}

#[derive(Debug, Clone, Copy)]
struct OrbitSamplingContext {
    seed: u64,
    system_id: u64,
    attempt: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OrbitalScaleRegime {
    MDwarf,
    SolarType,
}

#[derive(Debug, Clone, Copy)]
pub struct StellarOrbitalHierarchySampler {
    pub(crate) model: StellarOrbitalHierarchyModel,
}

impl StellarOrbitalHierarchySampler {
    pub fn new(model: StellarOrbitalHierarchyModel) -> Result<Self, StellarOrbitalHierarchyError> {
        if !valid_stellar_orbital_hierarchy_model(model) {
            return Err(StellarOrbitalHierarchyError::InvalidModel);
        }
        Ok(Self { model })
    }

    pub(crate) fn generate(
        &self,
        seed: u64,
        system_id: u64,
        members: &[StellarOrbitMemberInput],
    ) -> Result<StellarOrbitalHierarchy, StellarOrbitalHierarchyError> {
        if members.is_empty() || members.len() > 4 {
            return Err(StellarOrbitalHierarchyError::UnsupportedMemberCount);
        }
        if members.len() == 1 {
            return Ok(StellarOrbitalHierarchy {
                model_version: self.model.model_version,
                root: member_orbit_node(members[0]),
                quadruple_topology: None,
                quality_flags: Vec::new(),
            });
        }
        let primary = members
            .iter()
            .find(|member| member.role == StellarMemberRole::Primary)
            .ok_or(StellarOrbitalHierarchyError::InvalidModel)?;
        let (regime, mut quality_flags) = self.scale_regime(primary.mass_msun)?;
        for member in members {
            if let StellarOrbitMemberProvenance::LowMassContactRadiusProxy {
                solar_composition_proxy,
                hydrogen_burning_boundary_ambiguous,
            } = member.provenance
            {
                push_unique_quality_flag(
                    &mut quality_flags,
                    StellarOrbitalHierarchyQualityFlag::LowMassContactRadiusProxy,
                );
                push_unique_quality_flag(
                    &mut quality_flags,
                    StellarOrbitalHierarchyQualityFlag::BirthMassUsedAsDynamicalMass,
                );
                if solar_composition_proxy {
                    push_unique_quality_flag(
                        &mut quality_flags,
                        StellarOrbitalHierarchyQualityFlag::SolarCompositionRadiusProxy,
                    );
                }
                if hydrogen_burning_boundary_ambiguous {
                    push_unique_quality_flag(
                        &mut quality_flags,
                        StellarOrbitalHierarchyQualityFlag::HydrogenBurningBoundaryAmbiguous,
                    );
                }
            }
        }
        if regime == OrbitalScaleRegime::MDwarf {
            if !self.model.eccentricity.m_dwarf_uses_solar_proxy {
                return Err(StellarOrbitalHierarchyError::OutsideEccentricityCalibration);
            }
            quality_flags.push(StellarOrbitalHierarchyQualityFlag::SolarEccentricityProxyForMDwarf);
        }
        if members.len() >= 3 {
            quality_flags.extend([
                StellarOrbitalHierarchyQualityFlag::HierarchyPairingEngineered,
                StellarOrbitalHierarchyQualityFlag::IndependentOrbitScaleDraws,
                StellarOrbitalHierarchyQualityFlag::CoplanarProgradeStabilityScreen,
            ]);
        }

        let topology = (members.len() == 4).then(|| {
            let draw = domain_rng(
                seed,
                b"stellar_orbit/quadruple_topology/v1",
                Some(system_id),
            )
            .gen_range(0.0..1.0);
            if draw < self.model.quadruple_topology.probability_two_plus_two {
                QuadrupleTopology::TwoPlusTwo
            } else {
                QuadrupleTopology::ThreePlusOne
            }
        });
        if members.len() == 4 && regime != OrbitalScaleRegime::SolarType {
            quality_flags.push(StellarOrbitalHierarchyQualityFlag::TopologyMassExtrapolation);
        }

        for attempt in 0..self.model.stability.maximum_sampling_attempts {
            if let Some(root) =
                self.sample_candidate(seed, system_id, attempt, members, regime, topology)
            {
                return Ok(StellarOrbitalHierarchy {
                    model_version: self.model.model_version,
                    root,
                    quadruple_topology: topology,
                    quality_flags,
                });
            }
        }
        Err(StellarOrbitalHierarchyError::StableHierarchySamplingExhausted)
    }

    fn scale_regime(
        &self,
        primary_mass_msun: f64,
    ) -> Result<
        (OrbitalScaleRegime, Vec<StellarOrbitalHierarchyQualityFlag>),
        StellarOrbitalHierarchyError,
    > {
        let m = self.model.m_dwarf_scale;
        if (m.minimum_primary_mass_msun..m.maximum_primary_mass_msun).contains(&primary_mass_msun) {
            let mut flags =
                vec![StellarOrbitalHierarchyQualityFlag::MSeparationShapeDecoupledFromMassRatio];
            if primary_mass_msun < m.source_minimum_primary_mass_msun {
                flags.push(StellarOrbitalHierarchyQualityFlag::LowMassExtrapolation);
            }
            if primary_mass_msun > m.source_maximum_primary_mass_msun {
                flags.push(StellarOrbitalHierarchyQualityFlag::HighMassExtrapolation);
            }
            return Ok((OrbitalScaleRegime::MDwarf, flags));
        }
        let solar = self.model.solar_type_scale;
        if (solar.minimum_primary_mass_msun..=solar.maximum_primary_mass_msun)
            .contains(&primary_mass_msun)
        {
            let mut flags = Vec::new();
            if !(solar.source_anchor_minimum_primary_mass_msun
                ..=solar.source_anchor_maximum_primary_mass_msun)
                .contains(&primary_mass_msun)
            {
                flags.push(StellarOrbitalHierarchyQualityFlag::SolarPeriodShapeProxy);
            }
            return Ok((OrbitalScaleRegime::SolarType, flags));
        }
        Err(StellarOrbitalHierarchyError::OutsideOrbitalScaleCalibration { primary_mass_msun })
    }

    fn sample_candidate(
        &self,
        seed: u64,
        system_id: u64,
        attempt: u16,
        members: &[StellarOrbitMemberInput],
        regime: OrbitalScaleRegime,
        topology: Option<QuadrupleTopology>,
    ) -> Option<StellarOrbitNode> {
        let context = OrbitSamplingContext {
            seed,
            system_id,
            attempt,
        };
        let required_orbits = members.len() - 1;
        let mut scales: Vec<_> = (0..required_orbits)
            .map(|slot| self.sample_orbital_scale(seed, system_id, attempt, slot as u8, regime))
            .collect::<Option<_>>()?;
        scales.sort_by(f64::total_cmp);

        match (members.len(), topology) {
            (2, _) => {
                let pair_mass = members[0].mass_msun + members[1].mass_msun;
                self.make_leaf_orbit(
                    context,
                    0,
                    members[0],
                    members[1],
                    resolve_semimajor_axis_au(scales[0], pair_mass, regime),
                )
            }
            (3, _) => {
                let inner_mass = members[0].mass_msun + members[1].mass_msun;
                let total_mass = inner_mass + members[2].mass_msun;
                let inner = self.make_leaf_orbit(
                    context,
                    0,
                    members[0],
                    members[1],
                    resolve_semimajor_axis_au(scales[0], inner_mass, regime),
                )?;
                let outer = self.make_relative_orbit(
                    context,
                    1,
                    inner,
                    member_orbit_node(members[2]),
                    resolve_semimajor_axis_au(scales[1], total_mass, regime),
                );
                self.nested_pair_is_stable(&outer).then_some(outer)
            }
            (4, Some(QuadrupleTopology::ThreePlusOne)) => {
                let inner_mass = members[0].mass_msun + members[1].mass_msun;
                let middle_mass = inner_mass + members[2].mass_msun;
                let total_mass = middle_mass + members[3].mass_msun;
                let inner = self.make_leaf_orbit(
                    context,
                    0,
                    members[0],
                    members[1],
                    resolve_semimajor_axis_au(scales[0], inner_mass, regime),
                )?;
                let middle = self.make_relative_orbit(
                    context,
                    1,
                    inner,
                    member_orbit_node(members[2]),
                    resolve_semimajor_axis_au(scales[1], middle_mass, regime),
                );
                if !self.nested_pair_is_stable(&middle) {
                    return None;
                }
                let outer = self.make_relative_orbit(
                    context,
                    2,
                    middle,
                    member_orbit_node(members[3]),
                    resolve_semimajor_axis_au(scales[2], total_mass, regime),
                );
                self.nested_pair_is_stable(&outer).then_some(outer)
            }
            (4, Some(QuadrupleTopology::TwoPlusTwo)) => {
                let left_mass = members[0].mass_msun + members[1].mass_msun;
                let right_mass = members[2].mass_msun + members[3].mass_msun;
                let left = self.make_leaf_orbit(
                    context,
                    0,
                    members[0],
                    members[1],
                    resolve_semimajor_axis_au(scales[0], left_mass, regime),
                )?;
                let right = self.make_leaf_orbit(
                    context,
                    1,
                    members[2],
                    members[3],
                    resolve_semimajor_axis_au(scales[1], right_mass, regime),
                )?;
                let outer = self.make_relative_orbit(
                    context,
                    2,
                    left,
                    right,
                    resolve_semimajor_axis_au(scales[2], left_mass + right_mass, regime),
                );
                self.two_plus_two_is_stable(&outer).then_some(outer)
            }
            _ => None,
        }
    }

    fn sample_orbital_scale(
        &self,
        seed: u64,
        system_id: u64,
        attempt: u16,
        slot: u8,
        regime: OrbitalScaleRegime,
    ) -> Option<f64> {
        let entity = stable_orbit_draw_id(system_id, attempt, slot);
        let mut rng = domain_rng(seed, b"stellar_orbit/scale/v1", Some(entity));
        match regime {
            OrbitalScaleRegime::MDwarf => {
                let model = self.model.m_dwarf_scale;
                let normal = Normal::new(
                    model.log10_semimajor_axis_au_mean,
                    model.log10_semimajor_axis_au_standard_deviation,
                )
                .expect("validated orbital scale");
                let semimajor_axis_au = 10_f64.powf(normal.sample(&mut rng));
                (semimajor_axis_au > 0.0 && semimajor_axis_au <= model.maximum_semimajor_axis_au)
                    .then_some(semimajor_axis_au)
            }
            OrbitalScaleRegime::SolarType => {
                let model = self.model.solar_type_scale;
                let normal = Normal::new(
                    model.log10_period_days_mean,
                    model.log10_period_days_standard_deviation,
                )
                .expect("validated orbital scale");
                let log_period = normal.sample(&mut rng);
                (model.minimum_log10_period_days..model.maximum_log10_period_days)
                    .contains(&log_period)
                    .then(|| 10_f64.powf(log_period))
            }
        }
    }

    fn make_leaf_orbit(
        &self,
        context: OrbitSamplingContext,
        slot: u8,
        left: StellarOrbitMemberInput,
        right: StellarOrbitMemberInput,
        semimajor_axis_au: f64,
    ) -> Option<StellarOrbitNode> {
        let orbit = self.sample_orbit(
            context.seed,
            context.system_id,
            context.attempt,
            slot,
            semimajor_axis_au,
            left.mass_msun + right.mass_msun,
        );
        let minimum_separation_au = (left.radius_rsun + right.radius_rsun) * 0.004_650_467_3;
        (orbit.semimajor_axis_au * (1.0 - orbit.eccentricity) > minimum_separation_au).then(|| {
            StellarOrbitNode::RelativeOrbit {
                orbit,
                left: Box::new(member_orbit_node(left)),
                right: Box::new(member_orbit_node(right)),
            }
        })
    }

    fn make_relative_orbit(
        &self,
        context: OrbitSamplingContext,
        slot: u8,
        left: StellarOrbitNode,
        right: StellarOrbitNode,
        semimajor_axis_au: f64,
    ) -> StellarOrbitNode {
        let combined_mass_msun = left.mass_msun() + right.mass_msun();
        StellarOrbitNode::RelativeOrbit {
            orbit: self.sample_orbit(
                context.seed,
                context.system_id,
                context.attempt,
                slot,
                semimajor_axis_au,
                combined_mass_msun,
            ),
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn sample_orbit(
        &self,
        seed: u64,
        system_id: u64,
        attempt: u16,
        slot: u8,
        semimajor_axis_au: f64,
        combined_mass_msun: f64,
    ) -> RelativeStellarOrbit {
        let period_days = period_days_from_semimajor_axis(semimajor_axis_au, combined_mass_msun);
        let eccentricity = if period_days <= self.model.eccentricity.circularization_period_days {
            0.0
        } else {
            let period_envelope = 1.0 - (period_days / 2.0).powf(-2.0 / 3.0);
            let maximum = self
                .model
                .eccentricity
                .absolute_maximum
                .min(period_envelope);
            domain_rng(
                seed,
                b"stellar_orbit/eccentricity/v1",
                Some(stable_orbit_draw_id(system_id, attempt, slot)),
            )
            .gen_range(0.0..maximum)
        };
        RelativeStellarOrbit {
            semimajor_axis_au,
            period_days,
            eccentricity,
            combined_mass_msun,
        }
    }

    fn nested_pair_is_stable(&self, node: &StellarOrbitNode) -> bool {
        let StellarOrbitNode::RelativeOrbit {
            orbit: outer,
            left,
            right,
        } = node
        else {
            return false;
        };
        let (inner, outer_child_mass) = match (&**left, &**right) {
            (StellarOrbitNode::RelativeOrbit { orbit, .. }, other) => (orbit, other.mass_msun()),
            (other, StellarOrbitNode::RelativeOrbit { orbit, .. }) => (orbit, other.mass_msun()),
            _ => return true,
        };
        self.passes_stability(inner, outer, outer_child_mass / inner.combined_mass_msun)
    }

    fn two_plus_two_is_stable(&self, node: &StellarOrbitNode) -> bool {
        let StellarOrbitNode::RelativeOrbit {
            orbit: outer,
            left,
            right,
        } = node
        else {
            return false;
        };
        let (
            StellarOrbitNode::RelativeOrbit {
                orbit: left_inner, ..
            },
            StellarOrbitNode::RelativeOrbit {
                orbit: right_inner, ..
            },
        ) = (&**left, &**right)
        else {
            return false;
        };
        self.passes_stability(
            left_inner,
            outer,
            right_inner.combined_mass_msun / left_inner.combined_mass_msun,
        ) && self.passes_stability(
            right_inner,
            outer,
            left_inner.combined_mass_msun / right_inner.combined_mass_msun,
        )
    }

    fn passes_stability(
        &self,
        inner: &RelativeStellarOrbit,
        outer: &RelativeStellarOrbit,
        outer_mass_ratio: f64,
    ) -> bool {
        let model = self.model.stability;
        let critical_ratio = model.mardling_aarseth_coefficient
            * (1.0 + outer_mass_ratio).powf(2.0 / 5.0)
            * (1.0 + outer.eccentricity).powf(2.0 / 5.0)
            / (1.0 - outer.eccentricity).powf(6.0 / 5.0)
            * (1.0 - 0.3 * model.mutual_inclination_rad / std::f64::consts::PI);
        outer.semimajor_axis_au / inner.semimajor_axis_au > critical_ratio
    }
}

fn member_orbit_node(member: StellarOrbitMemberInput) -> StellarOrbitNode {
    StellarOrbitNode::Member {
        member_id: member.id,
        mass_msun: member.mass_msun,
    }
}

fn push_unique_quality_flag(
    flags: &mut Vec<StellarOrbitalHierarchyQualityFlag>,
    flag: StellarOrbitalHierarchyQualityFlag,
) {
    if !flags.contains(&flag) {
        flags.push(flag);
    }
}

pub(crate) fn semimajor_axis_from_period_days(period_days: f64, combined_mass_msun: f64) -> f64 {
    (combined_mass_msun * (period_days / 365.25).powi(2)).cbrt()
}

fn resolve_semimajor_axis_au(
    sampled_scale: f64,
    combined_mass_msun: f64,
    regime: OrbitalScaleRegime,
) -> f64 {
    match regime {
        OrbitalScaleRegime::MDwarf => sampled_scale,
        OrbitalScaleRegime::SolarType => {
            semimajor_axis_from_period_days(sampled_scale, combined_mass_msun)
        }
    }
}

fn period_days_from_semimajor_axis(semimajor_axis_au: f64, combined_mass_msun: f64) -> f64 {
    365.25 * (semimajor_axis_au.powi(3) / combined_mass_msun).sqrt()
}

fn stable_orbit_draw_id(system_id: u64, attempt: u16, slot: u8) -> u64 {
    let mut input = Vec::with_capacity(56);
    input.extend_from_slice(b"star_sim/stellar_orbit_draw/v1");
    input.extend_from_slice(&system_id.to_le_bytes());
    input.extend_from_slice(&attempt.to_le_bytes());
    input.push(slot);
    let hash = blake3::hash(&input);
    u64::from_le_bytes(
        hash.as_bytes()[..8]
            .try_into()
            .expect("eight-byte hash prefix"),
    )
}

fn valid_stellar_orbital_hierarchy_model(model: StellarOrbitalHierarchyModel) -> bool {
    let m = model.m_dwarf_scale;
    let solar = model.solar_type_scale;
    let eccentricity = model.eccentricity;
    let topology = model.quadruple_topology;
    let stability = model.stability;
    let contact_proxy = model.low_mass_contact_radius_proxy;
    [
        m.minimum_primary_mass_msun,
        m.source_minimum_primary_mass_msun,
        m.source_maximum_primary_mass_msun,
        m.maximum_primary_mass_msun,
        m.log10_semimajor_axis_au_mean,
        m.log10_semimajor_axis_au_standard_deviation,
        m.maximum_semimajor_axis_au,
        solar.minimum_primary_mass_msun,
        solar.maximum_primary_mass_msun,
        solar.source_anchor_minimum_primary_mass_msun,
        solar.source_anchor_maximum_primary_mass_msun,
        solar.log10_period_days_mean,
        solar.log10_period_days_standard_deviation,
        solar.minimum_log10_period_days,
        solar.maximum_log10_period_days,
        eccentricity.circularization_period_days,
        eccentricity.absolute_maximum,
        topology.probability_two_plus_two,
        topology.probability_three_plus_one,
        stability.mardling_aarseth_coefficient,
        stability.mutual_inclination_rad,
        contact_proxy.minimum_mass_msun,
        contact_proxy.maximum_mass_msun_exclusive,
        contact_proxy.minimum_age_gyr,
        contact_proxy.old_field_age_gyr,
        contact_proxy.young_field_radius_rsun,
        contact_proxy.old_field_radius_rsun,
    ]
    .into_iter()
    .all(f64::is_finite)
        && m.minimum_primary_mass_msun > 0.0
        && m.minimum_primary_mass_msun < m.source_minimum_primary_mass_msun
        && m.source_minimum_primary_mass_msun < m.source_maximum_primary_mass_msun
        && m.source_maximum_primary_mass_msun < m.maximum_primary_mass_msun
        && m.log10_semimajor_axis_au_standard_deviation > 0.0
        && m.maximum_semimajor_axis_au > 0.0
        && solar.minimum_primary_mass_msun == m.maximum_primary_mass_msun
        && solar.minimum_primary_mass_msun < solar.source_anchor_minimum_primary_mass_msun
        && solar.source_anchor_minimum_primary_mass_msun
            <= solar.source_anchor_maximum_primary_mass_msun
        && solar.source_anchor_maximum_primary_mass_msun <= solar.maximum_primary_mass_msun
        && solar.log10_period_days_standard_deviation > 0.0
        && solar.minimum_log10_period_days < solar.maximum_log10_period_days
        && eccentricity.circularization_period_days > 0.0
        && (0.0..1.0).contains(&eccentricity.absolute_maximum)
        && topology.probability_two_plus_two >= 0.0
        && topology.probability_three_plus_one >= 0.0
        && ((topology.probability_two_plus_two + topology.probability_three_plus_one) - 1.0).abs()
            < 1e-9
        && stability.mardling_aarseth_coefficient > 0.0
        && (0.0..=std::f64::consts::PI).contains(&stability.mutual_inclination_rad)
        && stability.maximum_sampling_attempts > 0
        && contact_proxy.minimum_mass_msun > 0.0
        && contact_proxy.minimum_mass_msun < contact_proxy.maximum_mass_msun_exclusive
        && contact_proxy.minimum_age_gyr >= 0.0
        && contact_proxy.minimum_age_gyr < contact_proxy.old_field_age_gyr
        && contact_proxy.young_field_radius_rsun > 0.0
        && contact_proxy.old_field_radius_rsun > 0.0
        && contact_proxy.old_field_radius_rsun <= contact_proxy.young_field_radius_rsun
}

pub(crate) fn low_mass_contact_radius_input(
    model: LowMassContactRadiusProxyModel,
    birth: &StellarBirthMember,
    history: StellarPopulationHistory,
    error: &StellarEvolutionError,
) -> Option<StellarOrbitMemberInput> {
    if !matches!(error, StellarEvolutionError::OutsideMassGrid { .. })
        || !(model.minimum_mass_msun..model.maximum_mass_msun_exclusive)
            .contains(&birth.initial_mass_msun)
        || history.age_gyr < model.minimum_age_gyr
    {
        return None;
    }
    let radius_rsun = if history.age_gyr >= model.old_field_age_gyr {
        model.old_field_radius_rsun
    } else {
        model.young_field_radius_rsun
    };
    Some(StellarOrbitMemberInput {
        id: birth.id,
        role: birth.role,
        mass_msun: birth.initial_mass_msun,
        radius_rsun,
        provenance: StellarOrbitMemberProvenance::LowMassContactRadiusProxy {
            solar_composition_proxy: history.chemistry.global_metallicity_mh.abs() > 1e-12,
            hydrogen_burning_boundary_ambiguous: birth.initial_mass_msun <= model.minimum_mass_msun,
        },
    })
}
