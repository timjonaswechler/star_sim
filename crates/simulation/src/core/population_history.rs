//! Population history and stellar-chemistry sampling.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TruncatedNormal {
    pub mean: f64,
    pub standard_deviation: f64,
    pub minimum: f64,
    pub maximum: f64,
}

/// First-order change in mean iron abundance with galactocentric radius.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RadialMetallicityGradient {
    /// Radius at which `iron_abundance_feh.mean` applies.
    pub reference_radius_pc: f64,
    /// Change in [Fe/H] per kiloparsec away from the reference radius.
    pub dex_per_kpc: f64,
    /// Inner edge of the observational calibration range.
    pub calibration_min_radius_pc: f64,
    /// Outer edge of the observational calibration range.
    pub calibration_max_radius_pc: f64,
}

/// Population-dependent alpha enhancement conditioned on sampled [Fe/H].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AlphaEnhancementDistribution {
    pub mean_at_solar_iron: f64,
    pub mean_slope_per_feh: f64,
    pub mean_minimum: f64,
    pub mean_maximum: f64,
    pub standard_deviation: f64,
    pub minimum: f64,
    pub maximum: f64,
}

/// Parameters used to convert logarithmic abundances to initial mass fractions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChemicalCompositionModel {
    pub protosolar_hydrogen_mass_fraction_x: f64,
    pub solar_metal_mass_fraction_z: f64,
    pub primordial_helium_mass_fraction_y: f64,
    pub helium_to_metal_enrichment_ratio: f64,
    /// Solar metal mixture fraction assigned to alpha-capture elements.
    pub alpha_mixture_fraction: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PopulationHistoryDistribution {
    pub age_gyr: TruncatedNormal,
    pub iron_abundance_feh: TruncatedNormal,
    pub iron_abundance_radial_gradient: RadialMetallicityGradient,
    pub alpha_enhancement: AlphaEnhancementDistribution,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PopulationHistoryModelVersion {
    SpatialIronAndAlphaV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PopulationHistoryModel {
    pub model_version: PopulationHistoryModelVersion,
    pub chemical_composition: ChemicalCompositionModel,
    pub thin_disk: PopulationHistoryDistribution,
    pub thick_disk: PopulationHistoryDistribution,
    pub halo: PopulationHistoryDistribution,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StellarPopulationHistory {
    /// Time elapsed since the system formed; not its predicted total lifetime.
    pub age_gyr: f64,
    pub chemistry: StellarChemistry,
}

/// Coherent initial chemistry for a stellar system.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StellarChemistry {
    /// Logarithmic iron abundance relative to the Sun.
    pub iron_abundance_feh: f64,
    /// Logarithmic alpha-element enhancement relative to iron and the Sun.
    pub alpha_enhancement_alpha_fe: f64,
    /// Alpha-corrected logarithmic global metallicity relative to the Sun.
    pub global_metallicity_mh: f64,
    /// Initial hydrogen mass fraction.
    pub hydrogen_mass_fraction_x: f64,
    /// Initial mass fraction in elements heavier than helium.
    pub metal_mass_fraction_z: f64,
    /// Initial helium mass fraction.
    pub helium_mass_fraction_y: f64,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum PopulationHistoryError {
    #[error(
        "distribution `{field}` must have finite ordered bounds, a positive deviation, and a mean within its bounds"
    )]
    InvalidDistribution { field: &'static str },
    #[error(
        "radial metallicity gradient `{field}` must have finite ordered calibration bounds containing its non-negative reference radius"
    )]
    InvalidRadialGradient { field: &'static str },
    #[error("alpha-enhancement distribution `{field}` is invalid")]
    InvalidAlphaEnhancement { field: &'static str },
    #[error("chemical-composition parameter `{field}` is invalid")]
    InvalidChemicalComposition { field: &'static str },
}

#[derive(Debug, Clone, Copy)]
pub struct PopulationHistorySampler {
    model: PopulationHistoryModel,
}

impl PopulationHistorySampler {
    pub fn new(model: PopulationHistoryModel) -> Result<Self, PopulationHistoryError> {
        for (field, distribution) in [
            ("thin_disk.age_gyr", model.thin_disk.age_gyr),
            (
                "thin_disk.iron_abundance_feh",
                model.thin_disk.iron_abundance_feh,
            ),
            ("thick_disk.age_gyr", model.thick_disk.age_gyr),
            (
                "thick_disk.iron_abundance_feh",
                model.thick_disk.iron_abundance_feh,
            ),
            ("halo.age_gyr", model.halo.age_gyr),
            ("halo.iron_abundance_feh", model.halo.iron_abundance_feh),
        ] {
            if !valid_truncated_normal(distribution) {
                return Err(PopulationHistoryError::InvalidDistribution { field });
            }
        }
        for (field, gradient) in [
            (
                "thin_disk.iron_abundance_radial_gradient",
                model.thin_disk.iron_abundance_radial_gradient,
            ),
            (
                "thick_disk.iron_abundance_radial_gradient",
                model.thick_disk.iron_abundance_radial_gradient,
            ),
            (
                "halo.iron_abundance_radial_gradient",
                model.halo.iron_abundance_radial_gradient,
            ),
        ] {
            if !gradient.reference_radius_pc.is_finite()
                || gradient.reference_radius_pc < 0.0
                || !gradient.dex_per_kpc.is_finite()
                || !gradient.calibration_min_radius_pc.is_finite()
                || !gradient.calibration_max_radius_pc.is_finite()
                || gradient.calibration_min_radius_pc < 0.0
                || gradient.calibration_min_radius_pc > gradient.reference_radius_pc
                || gradient.reference_radius_pc > gradient.calibration_max_radius_pc
            {
                return Err(PopulationHistoryError::InvalidRadialGradient { field });
            }
        }
        for (field, alpha) in [
            (
                "thin_disk.alpha_enhancement",
                model.thin_disk.alpha_enhancement,
            ),
            (
                "thick_disk.alpha_enhancement",
                model.thick_disk.alpha_enhancement,
            ),
            ("halo.alpha_enhancement", model.halo.alpha_enhancement),
        ] {
            if !valid_alpha_enhancement(alpha) {
                return Err(PopulationHistoryError::InvalidAlphaEnhancement { field });
            }
        }
        let chemistry = model.chemical_composition;
        for (field, valid) in [
            (
                "protosolar_hydrogen_mass_fraction_x",
                chemistry.protosolar_hydrogen_mass_fraction_x.is_finite()
                    && (0.0..1.0).contains(&chemistry.protosolar_hydrogen_mass_fraction_x),
            ),
            (
                "solar_metal_mass_fraction_z",
                chemistry.solar_metal_mass_fraction_z.is_finite()
                    && chemistry.solar_metal_mass_fraction_z > 0.0,
            ),
            (
                "primordial_helium_mass_fraction_y",
                chemistry.primordial_helium_mass_fraction_y.is_finite()
                    && (0.0..1.0).contains(&chemistry.primordial_helium_mass_fraction_y),
            ),
            (
                "helium_to_metal_enrichment_ratio",
                chemistry.helium_to_metal_enrichment_ratio.is_finite()
                    && chemistry.helium_to_metal_enrichment_ratio >= 0.0,
            ),
            (
                "alpha_mixture_fraction",
                chemistry.alpha_mixture_fraction.is_finite()
                    && (0.0..1.0).contains(&chemistry.alpha_mixture_fraction),
            ),
        ] {
            if !valid {
                return Err(PopulationHistoryError::InvalidChemicalComposition { field });
            }
        }
        if chemistry.protosolar_hydrogen_mass_fraction_x + chemistry.solar_metal_mass_fraction_z
            >= 1.0
        {
            return Err(PopulationHistoryError::InvalidChemicalComposition {
                field: "solar_mass_fractions",
            });
        }
        Ok(Self { model })
    }

    pub fn sample(
        &self,
        seed: u64,
        system_id: u64,
        population: StellarPopulation,
        position: GalacticPosition,
    ) -> StellarPopulationHistory {
        let distribution = match population {
            StellarPopulation::ThinDisk => self.model.thin_disk,
            StellarPopulation::ThickDisk => self.model.thick_disk,
            StellarPopulation::Halo => self.model.halo,
        };
        let mut age_rng = domain_rng(seed, b"population_history/age/v1", Some(system_id));
        let mut metallicity_rng =
            domain_rng(seed, b"population_history/metallicity/v1", Some(system_id));
        let mut alpha_rng = domain_rng(seed, b"stellar_chemistry/alpha/v1", Some(system_id));
        let gradient = distribution.iron_abundance_radial_gradient;
        let calibrated_radius_pc = position.radius_pc.clamp(
            gradient.calibration_min_radius_pc,
            gradient.calibration_max_radius_pc,
        );
        let radial_offset_kpc = (calibrated_radius_pc - gradient.reference_radius_pc) / 1_000.0;
        let iron_abundance_feh_distribution = TruncatedNormal {
            mean: distribution.iron_abundance_feh.mean + gradient.dex_per_kpc * radial_offset_kpc,
            ..distribution.iron_abundance_feh
        };
        let iron_abundance_feh =
            sample_truncated_normal(iron_abundance_feh_distribution, &mut metallicity_rng);
        let alpha = distribution.alpha_enhancement;
        let alpha_mean = (alpha.mean_at_solar_iron + alpha.mean_slope_per_feh * iron_abundance_feh)
            .clamp(alpha.mean_minimum, alpha.mean_maximum);
        let alpha_enhancement_alpha_fe = sample_truncated_normal(
            TruncatedNormal {
                mean: alpha_mean,
                standard_deviation: alpha.standard_deviation,
                minimum: alpha.minimum,
                maximum: alpha.maximum,
            },
            &mut alpha_rng,
        );
        let chemistry = derive_stellar_chemistry(
            iron_abundance_feh,
            alpha_enhancement_alpha_fe,
            self.model.chemical_composition,
        );
        StellarPopulationHistory {
            age_gyr: sample_truncated_normal(distribution.age_gyr, &mut age_rng),
            chemistry,
        }
    }
}

fn valid_alpha_enhancement(alpha: AlphaEnhancementDistribution) -> bool {
    alpha.mean_at_solar_iron.is_finite()
        && alpha.mean_slope_per_feh.is_finite()
        && alpha.mean_minimum.is_finite()
        && alpha.mean_maximum.is_finite()
        && alpha.minimum <= alpha.mean_minimum
        && alpha.mean_minimum <= alpha.mean_maximum
        && alpha.mean_maximum <= alpha.maximum
        && (alpha.mean_minimum..=alpha.mean_maximum).contains(&alpha.mean_at_solar_iron)
        && alpha.standard_deviation.is_finite()
        && alpha.standard_deviation > 0.0
        && alpha.minimum.is_finite()
        && alpha.maximum.is_finite()
        && alpha.minimum < alpha.maximum
        && (alpha.minimum..=alpha.maximum).contains(&alpha.mean_at_solar_iron)
}

fn derive_stellar_chemistry(
    iron_abundance_feh: f64,
    alpha_enhancement_alpha_fe: f64,
    model: ChemicalCompositionModel,
) -> StellarChemistry {
    let alpha_term = model.alpha_mixture_fraction * 10_f64.powf(alpha_enhancement_alpha_fe)
        + (1.0 - model.alpha_mixture_fraction);
    let global_metallicity_mh = iron_abundance_feh + alpha_term.log10();

    let metal_to_hydrogen = 10_f64.powf(global_metallicity_mh) * model.solar_metal_mass_fraction_z
        / model.protosolar_hydrogen_mass_fraction_x;
    let metal_mass_fraction_z = metal_to_hydrogen * (1.0 - model.primordial_helium_mass_fraction_y)
        / (1.0 + metal_to_hydrogen * (1.0 + model.helium_to_metal_enrichment_ratio));
    let helium_mass_fraction_y = model.primordial_helium_mass_fraction_y
        + model.helium_to_metal_enrichment_ratio * metal_mass_fraction_z;
    let hydrogen_mass_fraction_x = 1.0 - helium_mass_fraction_y - metal_mass_fraction_z;

    StellarChemistry {
        iron_abundance_feh,
        alpha_enhancement_alpha_fe,
        global_metallicity_mh,
        hydrogen_mass_fraction_x,
        metal_mass_fraction_z,
        helium_mass_fraction_y,
    }
}

fn valid_truncated_normal(distribution: TruncatedNormal) -> bool {
    distribution.mean.is_finite()
        && distribution.standard_deviation.is_finite()
        && distribution.standard_deviation > 0.0
        && distribution.minimum.is_finite()
        && distribution.maximum.is_finite()
        && distribution.minimum < distribution.maximum
        && (distribution.minimum..=distribution.maximum).contains(&distribution.mean)
}

fn sample_truncated_normal(distribution: TruncatedNormal, rng: &mut ChaCha8Rng) -> f64 {
    let normal = Normal::new(distribution.mean, distribution.standard_deviation)
        .expect("validated normal distribution");
    loop {
        let sample = normal.sample(rng);
        if (distribution.minimum..=distribution.maximum).contains(&sample) {
            return sample;
        }
    }
}
