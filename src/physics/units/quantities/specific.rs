//! Specific units for thermodynamics and material properties.
//!
//! This module defines specific quantities (per unit mass) used in thermodynamics,
//! stellar physics, and material science calculations.

use crate::physics::units::constants::*;
use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Specific energy (Energy/Mass) - Length²/Time²
define_quantity!(SpecificEnergy, 2, 0, -2, 0, 0, 0, 0); // Length²/Time²

define_unit_dimension! {
    dimension SpecificEnergy {
        base_unit: JoulePerKilogram = 1.0,
        units: {
            JoulePerKilogram = 1.0,
            CaloriePerGram = JOULES_PER_KG_PER_CAL_PER_G,
            ErgPerGram = 0.1, // 1 erg/g = 0.1 J/kg
        },
        symbols: {
            JoulePerKilogram = "J/kg",
            CaloriePerGram = "cal/g",
            ErgPerGram = "erg/g",
        }
    }
}

// Specific heat capacity (Energy/(Mass×Temperature)) - Length²/(Time²×Temperature)
define_quantity!(SpecificHeatCapacity, 2, 0, -2, -1, 0, 0, 0); // Length²/(Time²×Temperature)

define_unit_dimension! {
    dimension SpecificHeatCapacity {
        base_unit: JoulePerKilogramKelvin = 1.0,
        units: {
            JoulePerKilogramKelvin = 1.0,
            CaloriePerGramKelvin = JOULES_PER_KG_K_PER_CAL_PER_G_K,
        },
        symbols: {
            JoulePerKilogramKelvin = "J/(kg⋅K)",
            CaloriePerGramKelvin = "cal/(g⋅K)",
        }
    }
}

// Specific entropy (same dimensions as specific heat capacity)
pub type SpecificEntropy<U> = SpecificHeatCapacity<U>;

// Specific gas constant (Energy/(Mass×Temperature)) - same as specific heat capacity
pub type SpecificGasConstant<U> = SpecificHeatCapacity<U>;

// Specific volume (Volume/Mass) - Length³/Mass
define_quantity!(SpecificVolume, 3, -1, 0, 0, 0, 0, 0); // Length³/Mass

define_unit_dimension! {
    dimension SpecificVolume {
        base_unit: CubicMeterPerKilogram = 1.0,
        units: {
            CubicMeterPerKilogram = 1.0,
            CubicCentimeterPerGram = 0.001, // 1 cm³/g = 0.001 m³/kg
        },
        symbols: {
            CubicMeterPerKilogram = "m³/kg",
            CubicCentimeterPerGram = "cm³/g",
        }
    }
}

// Convenience type aliases
pub type SpecificEnergyJKg = SpecificEnergy<JoulePerKilogram>;
pub type SpecificHeatJKgK = SpecificHeatCapacity<JoulePerKilogramKelvin>;
pub type SpecificVolumeM3Kg = SpecificVolume<CubicMeterPerKilogram>;

// Useful constants for stellar physics
impl SpecificEnergy<JoulePerKilogram> {
    /// Nuclear binding energy per nucleon for hydrogen fusion (~7 MeV/nucleon)
    pub fn hydrogen_fusion_energy() -> Self {
        Self::new(6.3e14) // J/kg - approximate value for hydrogen to helium
    }
    
    /// Gravitational binding energy per unit mass at stellar surface
    /// GM/R for typical stellar parameters
    pub fn stellar_binding_energy(gm: f64, radius: f64) -> Self {
        Self::new(gm / radius)
    }
}

impl SpecificHeatCapacity<JoulePerKilogramKelvin> {
    /// Specific heat capacity of an ideal monatomic gas (3/2 * R/M)
    pub fn ideal_monatomic_gas(molar_mass_kg: f64) -> Self {
        const GAS_CONSTANT: f64 = 8.314462618; // J/(mol⋅K)
        Self::new(1.5 * GAS_CONSTANT / molar_mass_kg)
    }
    
    /// Specific heat capacity of an ideal diatomic gas (5/2 * R/M)
    pub fn ideal_diatomic_gas(molar_mass_kg: f64) -> Self {
        const GAS_CONSTANT: f64 = 8.314462618; // J/(mol⋅K)
        Self::new(2.5 * GAS_CONSTANT / molar_mass_kg)
    }
}