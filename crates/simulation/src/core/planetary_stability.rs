//! Circumstellar S-type stability evaluation.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanetaryStabilityModelVersion {
    HolmanWiegertSTypeV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HolmanWiegertSTypeModel {
    pub constant: f64,
    pub mass_ratio_coefficient: f64,
    pub eccentricity_coefficient: f64,
    pub mass_ratio_eccentricity_coefficient: f64,
    pub eccentricity_squared_coefficient: f64,
    pub mass_ratio_eccentricity_squared_coefficient: f64,
    pub minimum_mass_ratio: f64,
    pub maximum_mass_ratio: f64,
    pub minimum_binary_eccentricity: f64,
    pub maximum_binary_eccentricity: f64,
    pub fit_residual_lower_factor: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlanetaryStabilityModel {
    pub model_version: PlanetaryStabilityModelVersion,
    pub s_type: HolmanWiegertSTypeModel,
}

impl Default for PlanetaryStabilityModel {
    fn default() -> Self {
        Self {
            model_version: PlanetaryStabilityModelVersion::HolmanWiegertSTypeV1,
            s_type: HolmanWiegertSTypeModel {
                constant: 0.464,
                mass_ratio_coefficient: -0.380,
                eccentricity_coefficient: -0.631,
                mass_ratio_eccentricity_coefficient: 0.586,
                eccentricity_squared_coefficient: 0.150,
                mass_ratio_eccentricity_squared_coefficient: -0.198,
                minimum_mass_ratio: 0.1,
                maximum_mass_ratio: 0.9,
                minimum_binary_eccentricity: 0.0,
                maximum_binary_eccentricity: 0.8,
                fit_residual_lower_factor: 0.89,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircumstellarStabilityQualityFlag {
    MasslessTestParticleApproximation,
    CircularCoplanarProgradePlanetAssumption,
    TenThousandBinaryPeriodIntegration,
    SiblingSubtreePointMassApproximation,
    HierarchicalMultipleNearestEdgeOnly,
    ApproximateAdditionalPerturbersNotIntegrated,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircumstellarSTypeStabilityZone {
    UnboundedByStellarCompanion {
        model_version: PlanetaryStabilityModelVersion,
        host_member_id: u64,
    },
    CompanionLimited {
        model_version: PlanetaryStabilityModelVersion,
        host_member_id: u64,
        nominal_outer_critical_semimajor_axis_au: f64,
        fit_residual_lower_semimajor_axis_au: f64,
        limiting_relative_orbit: RelativeStellarOrbit,
        limiting_companion_mass_msun: f64,
        companion_mass_fraction: f64,
        quality_flags: Vec<CircumstellarStabilityQualityFlag>,
    },
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PlanetaryStabilityError {
    #[error("planetary-stability model is invalid")]
    InvalidModel,
    #[error("stellar orbital hierarchy is unavailable")]
    MissingStellarHierarchy,
    #[error("stellar member is missing from its orbital hierarchy")]
    MissingStellarMember,
    #[error("companion mass fraction {mass_fraction:.4} is outside S-type calibration")]
    OutsideMassRatioCalibration { mass_fraction: f64 },
    #[error("stellar eccentricity {eccentricity:.4} is outside S-type calibration")]
    OutsideEccentricityCalibration { eccentricity: f64 },
    #[error("the fitted S-type critical semimajor axis is not positive")]
    NonPositiveCriticalSemimajorAxis,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PlanetaryStabilityEvaluator {
    model: PlanetaryStabilityModel,
}

impl PlanetaryStabilityEvaluator {
    pub(crate) fn new(model: PlanetaryStabilityModel) -> Result<Self, PlanetaryStabilityError> {
        if !valid_planetary_stability_model(model) {
            return Err(PlanetaryStabilityError::InvalidModel);
        }
        Ok(Self { model })
    }

    pub(crate) fn evaluate(
        &self,
        member_id: u64,
        system_member_count: usize,
        hierarchy: &Result<StellarOrbitalHierarchy, StellarOrbitalHierarchyError>,
    ) -> Result<CircumstellarSTypeStabilityZone, PlanetaryStabilityError> {
        let hierarchy = hierarchy
            .as_ref()
            .map_err(|_| PlanetaryStabilityError::MissingStellarHierarchy)?;
        let host_mass_msun = hierarchy
            .root
            .member_mass_msun(member_id)
            .ok_or(PlanetaryStabilityError::MissingStellarMember)?;
        let Some(companion) = hierarchy.root.direct_parent_companion(member_id) else {
            return (system_member_count == 1)
                .then_some(
                    CircumstellarSTypeStabilityZone::UnboundedByStellarCompanion {
                        model_version: self.model.model_version,
                        host_member_id: member_id,
                    },
                )
                .ok_or(PlanetaryStabilityError::MissingStellarMember);
        };
        let mass_fraction =
            companion.companion_mass_msun / (host_mass_msun + companion.companion_mass_msun);
        let model = self.model.s_type;
        if !(model.minimum_mass_ratio..=model.maximum_mass_ratio).contains(&mass_fraction) {
            return Err(PlanetaryStabilityError::OutsideMassRatioCalibration { mass_fraction });
        }
        let eccentricity = companion.orbit.eccentricity;
        if !(model.minimum_binary_eccentricity..=model.maximum_binary_eccentricity)
            .contains(&eccentricity)
        {
            return Err(PlanetaryStabilityError::OutsideEccentricityCalibration { eccentricity });
        }
        let eccentricity_squared = eccentricity * eccentricity;
        let critical_fraction = model.constant
            + model.mass_ratio_coefficient * mass_fraction
            + model.eccentricity_coefficient * eccentricity
            + model.mass_ratio_eccentricity_coefficient * mass_fraction * eccentricity
            + model.eccentricity_squared_coefficient * eccentricity_squared
            + model.mass_ratio_eccentricity_squared_coefficient
                * mass_fraction
                * eccentricity_squared;
        let nominal_outer_critical_semimajor_axis_au =
            critical_fraction * companion.orbit.semimajor_axis_au;
        if !nominal_outer_critical_semimajor_axis_au.is_finite()
            || nominal_outer_critical_semimajor_axis_au <= 0.0
        {
            return Err(PlanetaryStabilityError::NonPositiveCriticalSemimajorAxis);
        }
        let mut quality_flags = vec![
            CircumstellarStabilityQualityFlag::MasslessTestParticleApproximation,
            CircumstellarStabilityQualityFlag::CircularCoplanarProgradePlanetAssumption,
            CircumstellarStabilityQualityFlag::TenThousandBinaryPeriodIntegration,
        ];
        if companion.companion_is_subtree {
            quality_flags
                .push(CircumstellarStabilityQualityFlag::SiblingSubtreePointMassApproximation);
        }
        if system_member_count >= 3 {
            quality_flags.extend([
                CircumstellarStabilityQualityFlag::HierarchicalMultipleNearestEdgeOnly,
                CircumstellarStabilityQualityFlag::ApproximateAdditionalPerturbersNotIntegrated,
            ]);
        }
        Ok(CircumstellarSTypeStabilityZone::CompanionLimited {
            model_version: self.model.model_version,
            host_member_id: member_id,
            nominal_outer_critical_semimajor_axis_au,
            fit_residual_lower_semimajor_axis_au: nominal_outer_critical_semimajor_axis_au
                * model.fit_residual_lower_factor,
            limiting_relative_orbit: companion.orbit,
            limiting_companion_mass_msun: companion.companion_mass_msun,
            companion_mass_fraction: mass_fraction,
            quality_flags,
        })
    }
}

fn valid_planetary_stability_model(model: PlanetaryStabilityModel) -> bool {
    let s = model.s_type;
    [
        s.constant,
        s.mass_ratio_coefficient,
        s.eccentricity_coefficient,
        s.mass_ratio_eccentricity_coefficient,
        s.eccentricity_squared_coefficient,
        s.mass_ratio_eccentricity_squared_coefficient,
        s.minimum_mass_ratio,
        s.maximum_mass_ratio,
        s.minimum_binary_eccentricity,
        s.maximum_binary_eccentricity,
        s.fit_residual_lower_factor,
    ]
    .into_iter()
    .all(f64::is_finite)
        && (0.0..1.0).contains(&s.minimum_mass_ratio)
        && (s.minimum_mass_ratio..=1.0).contains(&s.maximum_mass_ratio)
        && s.minimum_binary_eccentricity >= 0.0
        && s.minimum_binary_eccentricity < s.maximum_binary_eccentricity
        && s.maximum_binary_eccentricity < 1.0
        && (0.0..=1.0).contains(&s.fit_residual_lower_factor)
}
