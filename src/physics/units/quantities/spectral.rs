//! Spectral units for astrophysics and electromagnetic radiation.
//!
//! This module defines units for wavelength, frequency, and related spectral quantities
//! used in astronomical observations and stellar physics.

use crate::physics::units::constants::*;
use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Wavelength is just Distance, but we define it separately for clarity
define_quantity!(Wavelength, 1, 0, 0, 0, 0, 0, 0); // Length

// Define Wavelength units with spectral focus
define_unit_dimension! {
    dimension Wavelength {
        base_unit: WavelengthMeter = 1.0,
        units: {
            WavelengthMeter = 1.0,
            Angstrom = 1e-10,
        },
        symbols: {
            WavelengthMeter = "m",
            Angstrom = "Å",
        }
    }
}

// Frequency already defined in dimensions.rs, but we can add spectral-specific units
// We'll extend the existing Frequency dimension

// Wavenumber (1/Length) - common in spectroscopy
define_quantity!(Wavenumber, -1, 0, 0, 0, 0, 0, 0); // 1/Length

define_unit_dimension! {
    dimension Wavenumber {
        base_unit: PerMeter = 1.0,
        units: {
            PerMeter = 1.0,
            PerAngstrom = 1e10,
        },
        symbols: {
            PerMeter = "m⁻¹",
            PerAngstrom = "Å⁻¹",
        }
    }
}

// Photon energy is Energy, but useful for spectral calculations
// We can use the existing Energy dimension

// Convenience functions for spectral calculations
impl Wavelength<WavelengthMeter> {
    /// Calculate frequency from wavelength using c = λν
    pub fn to_frequency(&self) -> f64 {
        SPEED_OF_LIGHT / self.to_si()
    }

    /// Calculate photon energy from wavelength using E = hc/λ  
    pub fn to_photon_energy(&self) -> f64 {
        PLANCK_CONSTANT * SPEED_OF_LIGHT / self.to_si()
    }
}

// Helper functions for spectral conversions
pub fn wavelength_to_frequency(wavelength_m: f64) -> f64 {
    SPEED_OF_LIGHT / wavelength_m
}

pub fn frequency_to_wavelength(frequency_hz: f64) -> f64 {
    SPEED_OF_LIGHT / frequency_hz
}

pub fn wavelength_to_photon_energy(wavelength_m: f64) -> f64 {
    PLANCK_CONSTANT * SPEED_OF_LIGHT / wavelength_m
}

pub fn photon_energy_to_wavelength(energy_j: f64) -> f64 {
    PLANCK_CONSTANT * SPEED_OF_LIGHT / energy_j
}
