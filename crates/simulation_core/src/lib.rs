//! Bevy-independent scientific model and deterministic generation logic.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal, Poisson};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExponentialDisk {
    /// Mid-plane number density at the solar radius, in stars per cubic parsec.
    pub local_stellar_number_density_per_pc3: f64,
    pub radial_scale_length_pc: f64,
    pub vertical_scale_height_pc: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PowerLawHalo {
    /// Number density at the solar radius, in stars per cubic parsec.
    pub local_stellar_number_density_per_pc3: f64,
    pub flattening: f64,
    pub power: f64,
    /// Numerical core used to keep the prototype profile finite at the centre.
    pub core_radius_pc: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AxisymmetricBulgeMass {
    /// Central mass density in solar masses per cubic parsec.
    pub central_mass_density_msun_per_pc3: f64,
    pub power: f64,
    pub scale_radius_pc: f64,
    pub cutoff_radius_pc: f64,
    pub flattening: f64,
}

/// Number-density model for a simple axisymmetric Milky-Way-like galaxy.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GalaxyModel {
    pub solar_radius_pc: f64,
    pub thin_disk: ExponentialDisk,
    pub thick_disk: ExponentialDisk,
    pub halo: PowerLawHalo,
    pub bulge_mass: AxisymmetricBulgeMass,
}

impl Default for GalaxyModel {
    fn default() -> Self {
        Self {
            solar_radius_pc: 8_178.0,
            thin_disk: ExponentialDisk {
                local_stellar_number_density_per_pc3: 0.07102,
                radial_scale_length_pc: 2_600.0,
                vertical_scale_height_pc: 300.0,
            },
            thick_disk: ExponentialDisk {
                local_stellar_number_density_per_pc3: 0.00852,
                radial_scale_length_pc: 3_600.0,
                vertical_scale_height_pc: 900.0,
            },
            halo: PowerLawHalo {
                local_stellar_number_density_per_pc3: 0.000355,
                flattening: 0.64,
                power: 2.8,
                core_radius_pc: 500.0,
            },
            bulge_mass: AxisymmetricBulgeMass {
                central_mass_density_msun_per_pc3: 99.3,
                power: 1.8,
                scale_radius_pc: 75.0,
                cutoff_radius_pc: 2_100.0,
                flattening: 0.5,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StellarPopulation {
    ThinDisk,
    ThickDisk,
    Halo,
}

impl StellarPopulation {
    pub const ALL: [Self; 3] = [Self::ThinDisk, Self::ThickDisk, Self::Halo];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ThinDisk => "Thin disk",
            Self::ThickDisk => "Thick disk",
            Self::Halo => "Stellar halo",
        }
    }
}

/// Contributions to the local stellar number density, in stars per cubic parsec.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PopulationDensity {
    pub thin_disk: f64,
    pub thick_disk: f64,
    pub halo: f64,
}

impl PopulationDensity {
    pub fn total(self) -> f64 {
        self.thin_disk + self.thick_disk + self.halo
    }

    pub const fn for_population(self, population: StellarPopulation) -> f64 {
        match population {
            StellarPopulation::ThinDisk => self.thin_disk,
            StellarPopulation::ThickDisk => self.thick_disk,
            StellarPopulation::Halo => self.halo,
        }
    }

    pub fn fraction(self, population: StellarPopulation) -> f64 {
        self.for_population(population) / self.total()
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum GalaxyModelError {
    #[error("galaxy parameter `{field}` must be finite and greater than zero")]
    InvalidPositiveParameter { field: &'static str },
}

impl GalaxyModel {
    pub fn validate(&self) -> Result<(), GalaxyModelError> {
        let parameters = [
            ("solar_radius_pc", self.solar_radius_pc),
            (
                "thin_disk.local_stellar_number_density_per_pc3",
                self.thin_disk.local_stellar_number_density_per_pc3,
            ),
            (
                "thin_disk.radial_scale_length_pc",
                self.thin_disk.radial_scale_length_pc,
            ),
            (
                "thin_disk.vertical_scale_height_pc",
                self.thin_disk.vertical_scale_height_pc,
            ),
            (
                "thick_disk.local_stellar_number_density_per_pc3",
                self.thick_disk.local_stellar_number_density_per_pc3,
            ),
            (
                "thick_disk.radial_scale_length_pc",
                self.thick_disk.radial_scale_length_pc,
            ),
            (
                "thick_disk.vertical_scale_height_pc",
                self.thick_disk.vertical_scale_height_pc,
            ),
            (
                "halo.local_stellar_number_density_per_pc3",
                self.halo.local_stellar_number_density_per_pc3,
            ),
            ("halo.flattening", self.halo.flattening),
            ("halo.power", self.halo.power),
            ("halo.core_radius_pc", self.halo.core_radius_pc),
            (
                "bulge_mass.central_mass_density_msun_per_pc3",
                self.bulge_mass.central_mass_density_msun_per_pc3,
            ),
            ("bulge_mass.power", self.bulge_mass.power),
            (
                "bulge_mass.scale_radius_pc",
                self.bulge_mass.scale_radius_pc,
            ),
            (
                "bulge_mass.cutoff_radius_pc",
                self.bulge_mass.cutoff_radius_pc,
            ),
            ("bulge_mass.flattening", self.bulge_mass.flattening),
        ];

        for (field, value) in parameters {
            if !value.is_finite() || value <= 0.0 {
                return Err(GalaxyModelError::InvalidPositiveParameter { field });
            }
        }
        Ok(())
    }

    /// Evaluates all stellar-population densities at one galactic position.
    pub fn stellar_number_density_at(&self, position: GalacticPosition) -> PopulationDensity {
        let radial_offset = position.radius_pc - self.solar_radius_pc;
        let abs_height = position.height_pc.abs();

        let thin_disk = disk_density(self.thin_disk, radial_offset, abs_height);
        let thick_disk = disk_density(self.thick_disk, radial_offset, abs_height);

        let elliptical_radius = (position.radius_pc.powi(2)
            + (position.height_pc / self.halo.flattening).powi(2)
            + self.halo.core_radius_pc.powi(2))
        .sqrt();
        let local_elliptical_radius =
            (self.solar_radius_pc.powi(2) + self.halo.core_radius_pc.powi(2)).sqrt();
        let halo = self.halo.local_stellar_number_density_per_pc3
            * (local_elliptical_radius / elliptical_radius).powf(self.halo.power);

        PopulationDensity {
            thin_disk,
            thick_disk,
            halo,
        }
    }

    /// Evaluates the shape-only bulge mass model in solar masses per cubic parsec.
    /// This value must not be added to stellar number densities.
    pub fn bulge_mass_density_at(&self, position: GalacticPosition) -> f64 {
        let bulge = self.bulge_mass;
        let elliptical_radius =
            (position.radius_pc.powi(2) + (position.height_pc / bulge.flattening).powi(2)).sqrt();
        bulge.central_mass_density_msun_per_pc3
            / (1.0 + elliptical_radius / bulge.scale_radius_pc).powf(bulge.power)
            * (-(elliptical_radius / bulge.cutoff_radius_pc).powi(2)).exp()
    }
}

fn disk_density(disk: ExponentialDisk, radial_offset: f64, abs_height: f64) -> f64 {
    disk.local_stellar_number_density_per_pc3
        * (-radial_offset / disk.radial_scale_length_pc).exp()
        * (-abs_height / disk.vertical_scale_height_pc).exp()
}

/// A position relative to the galactic centre and reference plane.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GalacticPosition {
    /// Cylindrical distance from the galactic centre, in parsecs.
    pub radius_pc: f64,
    /// Azimuth around the galactic centre, in radians.
    pub azimuth_rad: f64,
    /// Signed height above the galactic reference plane, in parsecs.
    pub height_pc: f64,
}

impl GalacticPosition {
    /// Converts a small local offset into galactocentric cylindrical coordinates.
    ///
    /// The local axes point radially outwards, along increasing azimuth, and
    /// vertically above the galactic plane. This tangent-plane approximation is
    /// intended for regions that are small compared with the galactic radius.
    pub fn with_local_offset(self, [radial, tangential, vertical]: [f64; 3]) -> Self {
        let local_x = self.radius_pc + radial;
        Self {
            radius_pc: local_x.hypot(tangential),
            azimuth_rad: self.azimuth_rad + tangential.atan2(local_x),
            height_pc: self.height_pc + vertical,
        }
    }
}

/// Input for materialising a spherical region of the galaxy.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RegionRequest {
    pub seed: u64,
    pub centre: GalacticPosition,
    /// Radius of the requested sphere, in parsecs.
    pub radius_pc: f64,
}

impl RegionRequest {
    pub fn new(seed: u64, centre: GalacticPosition, radius_pc: f64) -> Option<Self> {
        (radius_pc.is_finite() && radius_pc > 0.0).then_some(Self {
            seed,
            centre,
            radius_pc,
        })
    }
}

/// Distribution over stellar systems, not over individual stellar objects.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SystemMultiplicityModel {
    pub observed_multiplicity_fraction: f64,
    pub observed_companion_frequency: f64,
    pub single_system_fraction: f64,
    pub binary_system_fraction: f64,
    pub triple_system_fraction: f64,
    pub higher_order_system_fraction: f64,
    pub representative_higher_order_members: u8,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum MultiplicityModelError {
    #[error("multiplicity fractions must be finite, non-negative, and sum to one")]
    InvalidFractions,
    #[error("categorical multiplicities do not reproduce the observed aggregate constraints")]
    InconsistentObservedConstraints,
    #[error("representative higher-order systems must contain at least four stars")]
    InvalidHigherOrderMemberCount,
    #[error("stellar-object density must be finite and non-negative")]
    InvalidStellarDensity,
}

impl SystemMultiplicityModel {
    pub fn mean_stars_per_system(&self) -> Result<f64, MultiplicityModelError> {
        let fractions = [
            self.single_system_fraction,
            self.binary_system_fraction,
            self.triple_system_fraction,
            self.higher_order_system_fraction,
        ];
        let sum: f64 = fractions.iter().sum();
        if fractions
            .iter()
            .any(|fraction| !fraction.is_finite() || *fraction < 0.0)
            || (sum - 1.0).abs() > 1e-9
        {
            return Err(MultiplicityModelError::InvalidFractions);
        }
        if self.representative_higher_order_members < 4 {
            return Err(MultiplicityModelError::InvalidHigherOrderMemberCount);
        }

        let derived_multiplicity_fraction = 1.0 - self.single_system_fraction;
        let derived_companion_frequency = self.binary_system_fraction
            + 2.0 * self.triple_system_fraction
            + (f64::from(self.representative_higher_order_members) - 1.0)
                * self.higher_order_system_fraction;
        if !self.observed_multiplicity_fraction.is_finite()
            || !self.observed_companion_frequency.is_finite()
            || (derived_multiplicity_fraction - self.observed_multiplicity_fraction).abs() > 1e-9
            || (derived_companion_frequency - self.observed_companion_frequency).abs() > 1e-9
        {
            return Err(MultiplicityModelError::InconsistentObservedConstraints);
        }

        Ok(1.0 + self.observed_companion_frequency)
    }

    pub fn system_density(
        &self,
        stellar_object_density: f64,
    ) -> Result<f64, MultiplicityModelError> {
        if !stellar_object_density.is_finite() || stellar_object_density < 0.0 {
            return Err(MultiplicityModelError::InvalidStellarDensity);
        }
        Ok(stellar_object_density / self.mean_stars_per_system()?)
    }
}

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

/// Two-segment stellar initial mass function, dN/dm proportional to m^-alpha.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct KroupaInitialMassFunction {
    pub minimum_mass_msun: f64,
    pub break_mass_msun: f64,
    pub maximum_mass_msun: f64,
    pub low_mass_exponent: f64,
    pub high_mass_exponent: f64,
}

/// Multiplicity and companion-mass model for one primary-mass interval.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MassConditionedMultiplicity {
    pub minimum_primary_mass_msun: f64,
    pub maximum_primary_mass_msun: f64,
    pub single_system_fraction: f64,
    pub binary_system_fraction: f64,
    pub triple_system_fraction: f64,
    pub higher_order_system_fraction: f64,
    pub representative_higher_order_members: u8,
    pub minimum_mass_ratio: f64,
    /// Exponent gamma in p(q) proportional to q^gamma.
    pub mass_ratio_power: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarBirthMassModel {
    pub initial_mass_function: KroupaInitialMassFunction,
    pub minimum_companion_mass_msun: f64,
    pub multiplicity_bins: Vec<MassConditionedMultiplicity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StellarMemberRole {
    Primary,
    Companion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarBirthMember {
    pub id: u64,
    pub role: StellarMemberRole,
    pub initial_mass_msun: f64,
    pub mass_ratio_to_primary: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StellarBirthSystem {
    pub members: Vec<StellarBirthMember>,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum StellarBirthMassError {
    #[error("Kroupa IMF bounds and exponents must be finite, positive, and ordered")]
    InvalidInitialMassFunction,
    #[error("minimum companion mass must be finite and inside the IMF mass range")]
    InvalidMinimumCompanionMass,
    #[error("multiplicity bins must be valid, ordered, contiguous, and cover the IMF range")]
    InvalidMultiplicityBins,
}

#[derive(Debug, Clone)]
pub struct StellarBirthMassSampler {
    model: StellarBirthMassModel,
    low_segment_weight: f64,
    total_imf_weight: f64,
}

impl StellarBirthMassSampler {
    pub fn new(model: StellarBirthMassModel) -> Result<Self, StellarBirthMassError> {
        let imf = model.initial_mass_function;
        if [
            imf.minimum_mass_msun,
            imf.break_mass_msun,
            imf.maximum_mass_msun,
            imf.low_mass_exponent,
            imf.high_mass_exponent,
        ]
        .into_iter()
        .any(|value| !value.is_finite() || value <= 0.0)
            || !(imf.minimum_mass_msun < imf.break_mass_msun
                && imf.break_mass_msun < imf.maximum_mass_msun)
        {
            return Err(StellarBirthMassError::InvalidInitialMassFunction);
        }
        if !model.minimum_companion_mass_msun.is_finite()
            || !(imf.minimum_mass_msun..=imf.maximum_mass_msun)
                .contains(&model.minimum_companion_mass_msun)
        {
            return Err(StellarBirthMassError::InvalidMinimumCompanionMass);
        }
        if !valid_multiplicity_bins(&model) {
            return Err(StellarBirthMassError::InvalidMultiplicityBins);
        }

        let low_segment_weight = power_law_integral(
            imf.minimum_mass_msun,
            imf.break_mass_msun,
            imf.low_mass_exponent,
        );
        let continuity = imf
            .break_mass_msun
            .powf(imf.high_mass_exponent - imf.low_mass_exponent);
        let high_segment_weight = continuity
            * power_law_integral(
                imf.break_mass_msun,
                imf.maximum_mass_msun,
                imf.high_mass_exponent,
            );
        Ok(Self {
            model,
            low_segment_weight,
            total_imf_weight: low_segment_weight + high_segment_weight,
        })
    }

    pub fn sample(&self, seed: u64, system_id: u64) -> StellarBirthSystem {
        let primary_mass_msun = self.sample_primary_mass(seed, system_id);
        let multiplicity = self.multiplicity_for(primary_mass_msun);
        let mut multiplicity_rng = domain_rng(
            seed,
            b"stellar_birth/system_multiplicity/v1",
            Some(system_id),
        );
        let draw = multiplicity_rng.gen_range(0.0..1.0);
        let binary_end = multiplicity.single_system_fraction + multiplicity.binary_system_fraction;
        let triple_end = binary_end + multiplicity.triple_system_fraction;
        let member_count = if draw < multiplicity.single_system_fraction {
            1
        } else if draw < binary_end {
            2
        } else if draw < triple_end {
            3
        } else {
            multiplicity.representative_higher_order_members
        };

        let mut companion_mass_ratios = (1..member_count)
            .map(|rank| {
                let minimum_q = multiplicity
                    .minimum_mass_ratio
                    .max(self.model.minimum_companion_mass_msun / primary_mass_msun);
                let mut rng = domain_rng(
                    seed,
                    b"stellar_birth/companion_mass_ratio/v1",
                    Some(stable_member_id(system_id, rank)),
                );
                sample_power_law(minimum_q, 1.0, -multiplicity.mass_ratio_power, &mut rng)
            })
            .collect::<Vec<_>>();
        companion_mass_ratios.sort_by(|left, right| right.total_cmp(left));

        let mut members = Vec::with_capacity(usize::from(member_count));
        members.push(StellarBirthMember {
            id: stable_member_id(system_id, 0),
            role: StellarMemberRole::Primary,
            initial_mass_msun: primary_mass_msun,
            mass_ratio_to_primary: None,
        });
        members.extend(
            companion_mass_ratios
                .into_iter()
                .enumerate()
                .map(|(index, mass_ratio)| StellarBirthMember {
                    id: stable_member_id(system_id, index as u8 + 1),
                    role: StellarMemberRole::Companion,
                    initial_mass_msun: primary_mass_msun * mass_ratio,
                    mass_ratio_to_primary: Some(mass_ratio),
                }),
        );
        StellarBirthSystem { members }
    }

    pub fn expected_members_per_system(&self) -> f64 {
        self.model
            .multiplicity_bins
            .iter()
            .map(|bin| {
                let probability = self.imf_weight_between(
                    bin.minimum_primary_mass_msun,
                    bin.maximum_primary_mass_msun,
                ) / self.total_imf_weight;
                let mean_members = bin.single_system_fraction
                    + 2.0 * bin.binary_system_fraction
                    + 3.0 * bin.triple_system_fraction
                    + f64::from(bin.representative_higher_order_members)
                        * bin.higher_order_system_fraction;
                probability * mean_members
            })
            .sum()
    }

    pub fn multiplicity_fraction_for_primary_mass(&self, primary_mass_msun: f64) -> Option<f64> {
        let imf = self.model.initial_mass_function;
        if !primary_mass_msun.is_finite()
            || !(imf.minimum_mass_msun..=imf.maximum_mass_msun).contains(&primary_mass_msun)
        {
            return None;
        }
        Some(
            1.0 - self
                .multiplicity_for(primary_mass_msun)
                .single_system_fraction,
        )
    }

    fn sample_primary_mass(&self, seed: u64, system_id: u64) -> f64 {
        let imf = self.model.initial_mass_function;
        let mut rng = domain_rng(seed, b"stellar_birth/primary_mass/v1", Some(system_id));
        let draw = rng.gen_range(0.0..self.total_imf_weight);
        if draw < self.low_segment_weight {
            sample_power_law(
                imf.minimum_mass_msun,
                imf.break_mass_msun,
                imf.low_mass_exponent,
                &mut rng,
            )
        } else {
            sample_power_law(
                imf.break_mass_msun,
                imf.maximum_mass_msun,
                imf.high_mass_exponent,
                &mut rng,
            )
        }
    }

    fn multiplicity_for(&self, primary_mass_msun: f64) -> MassConditionedMultiplicity {
        *self
            .model
            .multiplicity_bins
            .iter()
            .find(|bin| primary_mass_msun <= bin.maximum_primary_mass_msun)
            .expect("validated multiplicity coverage")
    }

    fn imf_weight_between(&self, minimum: f64, maximum: f64) -> f64 {
        let imf = self.model.initial_mass_function;
        let continuity = imf
            .break_mass_msun
            .powf(imf.high_mass_exponent - imf.low_mass_exponent);
        let low_maximum = maximum.min(imf.break_mass_msun);
        let low_weight = if minimum < low_maximum {
            power_law_integral(minimum, low_maximum, imf.low_mass_exponent)
        } else {
            0.0
        };
        let high_minimum = minimum.max(imf.break_mass_msun);
        let high_weight = if high_minimum < maximum {
            continuity * power_law_integral(high_minimum, maximum, imf.high_mass_exponent)
        } else {
            0.0
        };
        low_weight + high_weight
    }
}

fn valid_multiplicity_bins(model: &StellarBirthMassModel) -> bool {
    let imf = model.initial_mass_function;
    if model.multiplicity_bins.is_empty() {
        return false;
    }
    let mut expected_minimum = imf.minimum_mass_msun;
    for bin in &model.multiplicity_bins {
        let fractions = [
            bin.single_system_fraction,
            bin.binary_system_fraction,
            bin.triple_system_fraction,
            bin.higher_order_system_fraction,
        ];
        if !bin.minimum_primary_mass_msun.is_finite()
            || !bin.maximum_primary_mass_msun.is_finite()
            || (bin.minimum_primary_mass_msun - expected_minimum).abs() > 1e-12
            || bin.minimum_primary_mass_msun >= bin.maximum_primary_mass_msun
            || fractions
                .into_iter()
                .any(|fraction| !fraction.is_finite() || fraction < 0.0)
            || (fractions.into_iter().sum::<f64>() - 1.0).abs() > 1e-9
            || bin.representative_higher_order_members < 4
            || !bin.minimum_mass_ratio.is_finite()
            || !(0.0..=1.0).contains(&bin.minimum_mass_ratio)
            || !bin.mass_ratio_power.is_finite()
        {
            return false;
        }
        expected_minimum = bin.maximum_primary_mass_msun;
    }
    (expected_minimum - imf.maximum_mass_msun).abs() <= 1e-12
}

fn power_law_integral(minimum: f64, maximum: f64, exponent: f64) -> f64 {
    if (exponent - 1.0).abs() < 1e-12 {
        (maximum / minimum).ln()
    } else {
        (maximum.powf(1.0 - exponent) - minimum.powf(1.0 - exponent)) / (1.0 - exponent)
    }
}

fn sample_power_law(minimum: f64, maximum: f64, exponent: f64, rng: &mut ChaCha8Rng) -> f64 {
    let draw = rng.gen_range(0.0..1.0);
    if (exponent - 1.0).abs() < 1e-12 {
        minimum * (maximum / minimum).powf(draw)
    } else {
        let power = 1.0 - exponent;
        (minimum.powf(power) + draw * (maximum.powf(power) - minimum.powf(power))).powf(1.0 / power)
    }
}

fn stable_member_id(system_id: u64, rank: u8) -> u64 {
    let mut input = Vec::with_capacity(48);
    input.extend_from_slice(b"star_sim/stellar_member_id/v1");
    input.extend_from_slice(&system_id.to_le_bytes());
    input.push(rank);
    let hash = blake3::hash(&input);
    u64::from_le_bytes(
        hash.as_bytes()[..8]
            .try_into()
            .expect("eight-byte hash prefix"),
    )
}

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

    fn member_mass_msun(&self, candidate_id: u64) -> Option<f64> {
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

    fn direct_parent_companion(&self, member_id: u64) -> Option<DirectParentCompanion> {
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
struct DirectParentCompanion {
    orbit: RelativeStellarOrbit,
    companion_mass_msun: f64,
    companion_is_subtree: bool,
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
enum StellarOrbitMemberProvenance {
    EvolutionSnapshot,
    LowMassContactRadiusProxy {
        solar_composition_proxy: bool,
        hydrogen_burning_boundary_ambiguous: bool,
    },
}

#[derive(Debug, Clone, Copy)]
struct StellarOrbitMemberInput {
    id: u64,
    role: StellarMemberRole,
    mass_msun: f64,
    radius_rsun: f64,
    provenance: StellarOrbitMemberProvenance,
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
    model: StellarOrbitalHierarchyModel,
}

impl StellarOrbitalHierarchySampler {
    pub fn new(model: StellarOrbitalHierarchyModel) -> Result<Self, StellarOrbitalHierarchyError> {
        if !valid_stellar_orbital_hierarchy_model(model) {
            return Err(StellarOrbitalHierarchyError::InvalidModel);
        }
        Ok(Self { model })
    }

    fn generate(
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

fn semimajor_axis_from_period_days(period_days: f64, combined_mass_msun: f64) -> f64 {
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

fn low_mass_contact_radius_input(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
struct PlanetaryStabilityEvaluator {
    model: PlanetaryStabilityModel,
}

impl PlanetaryStabilityEvaluator {
    fn new(model: PlanetaryStabilityModel) -> Result<Self, PlanetaryStabilityError> {
        if !valid_planetary_stability_model(model) {
            return Err(PlanetaryStabilityError::InvalidModel);
        }
        Ok(Self { model })
    }

    fn evaluate(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetOccurrenceQualityFlag {
    PoissonIndependenceApproximation,
    HostAgeDependenceNotModeled,
    MultiplicitySuppressionExtrapolated,
    PlanetPropertiesNotGenerated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanetPopulationSummary {
    pub model_version: PlanetOccurrenceModelVersion,
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
        let (small_planets, giant_planets) = match factor {
            Err(error) => (Err(error.clone()), Err(error)),
            Ok(factor) => {
                let Ok(snapshot) = evolution else {
                    return PlanetPopulationSummary {
                        model_version: self.model.model_version,
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

fn stable_planet_host_id(system_id: u64, member_id: u64) -> u64 {
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
struct ExplicitPlanetGenerator {
    model: ExplicitPlanetModel,
}

impl ExplicitPlanetGenerator {
    fn new(model: ExplicitPlanetModel) -> Result<Self, ExplicitPlanetModelError> {
        if !valid_explicit_planet_model(&model) {
            return Err(ExplicitPlanetModelError::InvalidModel);
        }
        Ok(Self { model })
    }

    fn generate(
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

fn valid_planet_occurrence_model(model: PlanetOccurrenceModel) -> bool {
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

/// Finite cylinder within which a seed may select a galactic position.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GalacticSamplingVolume {
    pub max_radius_pc: f64,
    pub max_abs_height_pc: f64,
    pub radial_bins: usize,
    pub vertical_bins: usize,
}

impl Default for GalacticSamplingVolume {
    fn default() -> Self {
        Self {
            max_radius_pc: 20_000.0,
            max_abs_height_pc: 10_000.0,
            radial_bins: 320,
            vertical_bins: 240,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SampledGalacticLocation {
    pub position: GalacticPosition,
    /// Population whose density contribution selected this position.
    pub sampled_population: StellarPopulation,
    pub local_density: PopulationDensity,
}

#[derive(Debug, Error)]
pub enum GalacticSamplingError {
    #[error(transparent)]
    InvalidGalaxyModel(#[from] GalaxyModelError),
    #[error("sampling volume must have positive finite dimensions and non-zero bin counts")]
    InvalidSamplingVolume,
    #[error("sampling volume contains no finite positive stellar density")]
    EmptySamplingVolume,
}

/// Precomputed density-weighted sampler for deterministic galactic positions.
pub struct GalacticLocationSampler {
    galaxy: GalaxyModel,
    volume: GalacticSamplingVolume,
    population_cdfs: [Vec<f64>; 3],
    population_totals: [f64; 3],
}

impl GalacticLocationSampler {
    pub fn new(
        galaxy: GalaxyModel,
        volume: GalacticSamplingVolume,
    ) -> Result<Self, GalacticSamplingError> {
        galaxy.validate()?;
        if !volume.max_radius_pc.is_finite()
            || volume.max_radius_pc <= 0.0
            || !volume.max_abs_height_pc.is_finite()
            || volume.max_abs_height_pc <= 0.0
            || volume.radial_bins == 0
            || volume.vertical_bins == 0
        {
            return Err(GalacticSamplingError::InvalidSamplingVolume);
        }

        let mut population_cdfs: [Vec<f64>; 3] =
            std::array::from_fn(|_| Vec::with_capacity(volume.radial_bins * volume.vertical_bins));
        let mut population_totals = [0.0; 3];
        let radial_step = volume.max_radius_pc / volume.radial_bins as f64;
        let vertical_step = 2.0 * volume.max_abs_height_pc / volume.vertical_bins as f64;

        for radial_index in 0..volume.radial_bins {
            let radius_inner = radial_index as f64 * radial_step;
            let radius_outer = radius_inner + radial_step;
            let radius_mid = (radius_inner + radius_outer) * 0.5;
            let annular_cell_volume = std::f64::consts::PI
                * (radius_outer.powi(2) - radius_inner.powi(2))
                * vertical_step;

            for vertical_index in 0..volume.vertical_bins {
                let height =
                    -volume.max_abs_height_pc + (vertical_index as f64 + 0.5) * vertical_step;
                let density = galaxy.stellar_number_density_at(GalacticPosition {
                    radius_pc: radius_mid,
                    azimuth_rad: 0.0,
                    height_pc: height,
                });

                for population in StellarPopulation::ALL {
                    let index = population_index(population);
                    population_totals[index] +=
                        density.for_population(population) * annular_cell_volume;
                    population_cdfs[index].push(population_totals[index]);
                }
            }
        }

        let total: f64 = population_totals.iter().sum();
        if !total.is_finite() || total <= 0.0 {
            return Err(GalacticSamplingError::EmptySamplingVolume);
        }

        Ok(Self {
            galaxy,
            volume,
            population_cdfs,
            population_totals,
        })
    }

    pub fn sample(&self, seed: u64) -> SampledGalacticLocation {
        let mut rng = position_rng(seed);
        let total: f64 = self.population_totals.iter().sum();
        let population_draw = rng.gen_range(0.0..total);
        let mut accumulated = 0.0;
        let mut sampled_population = StellarPopulation::Halo;
        for population in StellarPopulation::ALL {
            accumulated += self.population_totals[population_index(population)];
            if population_draw < accumulated {
                sampled_population = population;
                break;
            }
        }

        let population_index = population_index(sampled_population);
        let population_total = self.population_totals[population_index];
        let cell_draw = rng.gen_range(0.0..population_total);
        let cell_index = self.population_cdfs[population_index]
            .partition_point(|&cumulative| cumulative < cell_draw)
            .min(self.population_cdfs[population_index].len() - 1);
        let radial_index = cell_index / self.volume.vertical_bins;
        let vertical_index = cell_index % self.volume.vertical_bins;

        let radial_step = self.volume.max_radius_pc / self.volume.radial_bins as f64;
        let radius_inner = radial_index as f64 * radial_step;
        let radius_outer = radius_inner + radial_step;
        let radius_pc = rng
            .gen_range(radius_inner.powi(2)..radius_outer.powi(2))
            .sqrt();
        let vertical_step = 2.0 * self.volume.max_abs_height_pc / self.volume.vertical_bins as f64;
        let height_lower = -self.volume.max_abs_height_pc + vertical_index as f64 * vertical_step;
        let height_pc = rng.gen_range(height_lower..height_lower + vertical_step);
        let azimuth_rad = rng.gen_range(0.0..std::f64::consts::TAU);
        let position = GalacticPosition {
            radius_pc,
            azimuth_rad,
            height_pc,
        };

        SampledGalacticLocation {
            position,
            sampled_population,
            local_density: self.galaxy.stellar_number_density_at(position),
        }
    }

    pub fn population_probabilities(&self) -> [(StellarPopulation, f64); 3] {
        let total: f64 = self.population_totals.iter().sum();
        StellarPopulation::ALL.map(|population| {
            (
                population,
                self.population_totals[population_index(population)] / total,
            )
        })
    }
}

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
        let birth_mass_model: StellarBirthMassModel =
            ron::from_str(include_str!("../../../config/stellar_birth_masses.ron")).unwrap();
        let population_history_model: PopulationHistoryModel = ron::from_str(include_str!(
            "../../../config/stellar_population_history.ron"
        ))
        .unwrap();
        let evolution_model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();
        let snapshot = evaluator
            .evaluate(15.0, 0.013_822_753_672_319_613, solar_test_chemistry())
            .unwrap();

        assert_eq!(snapshot.state, EvolutionaryState::AdvancedBurningTrackEnd);
        assert_eq!(snapshot.raw_phase, 5);
    }

    #[test]
    fn massive_track_can_enter_a_wolf_rayet_state() {
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
        let evaluator = StellarEvolutionEvaluator::new(model).unwrap();

        assert!(matches!(
            evaluator.evaluate(3.0, 0.5, solar_test_chemistry()),
            Err(StellarEvolutionError::PostAgbTrackIncomplete { last_eep: 1409, .. })
        ));
    }

    #[test]
    fn prematurely_terminated_massive_track_is_not_called_core_collapse() {
        let model: StellarEvolutionModel =
            ron::from_str(include_str!("../../../config/stellar_evolution.ron")).unwrap();
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
