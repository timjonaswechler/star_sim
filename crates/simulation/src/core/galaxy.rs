//! Galactic density models, positions, and region requests.

use super::*;

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
