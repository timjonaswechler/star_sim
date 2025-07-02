//! Specific units for thermodynamics and material properties in stellar systems.
//!
//! This module defines specific quantities (per unit mass) used in thermodynamics,
//! stellar physics, and material science calculations. These units are essential
//! for modeling stellar interiors, planetary atmospheres, and material properties.
//!
//! # Available Units
//!
//! ## Specific Energy
//! - **JoulePerKilogram** (`J/kg`) - SI unit for specific energy
//! - **CaloriePerGram** (`cal/g`) - Common laboratory unit
//! - **ErgPerGram** (`erg/g`) - CGS unit for specific energy
//!
//! ## Specific Heat Capacity
//! - **JoulePerKilogramKelvin** (`J/(kg⋅K)`) - SI unit for specific heat capacity
//! - **CaloriePerGramKelvin** (`cal/(g⋅K)`) - Laboratory unit
//!
//! ## Specific Volume
//! - **CubicMeterPerKilogram** (`m³/kg`) - SI unit for specific volume
//! - **CubicCentimeterPerGram** (`cm³/g`) - Laboratory unit
//!
//! # Physical Applications
//!
//! - **Stellar interior equations of state** and thermodynamics
//! - **Planetary atmospheric modeling** and heat transfer
//! - **Nuclear fusion energy** calculations and stellar evolution
//! - **Material property analysis** for astronomical objects
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Nuclear fusion energy release
//! let fusion_energy = SpecificEnergy::<JoulePerKilogram>::new(6.3e14);
//! println!("Hydrogen fusion energy: {:.2e} J/kg", fusion_energy.value());
//!
//! // Specific heat of stellar material
//! let stellar_cp = SpecificHeatCapacity::<JoulePerKilogramKelvin>::new(5000.0);
//! println!("Stellar specific heat: {:.0} J/(kg⋅K)", stellar_cp.value());
//!
//! // Using helper functions for ideal gases
//! let hydrogen_cp = SpecificHeatCapacity::<JoulePerKilogramKelvin>::ideal_diatomic_gas(2e-3);
//! println!("H₂ specific heat: {:.0} J/(kg⋅K)", hydrogen_cp.value());
//!
//! // Stellar binding energy calculation
//! let binding_energy = SpecificEnergy::<JoulePerKilogram>::stellar_binding_energy(1.327e20, 6.96e8);
//! println!("Solar binding energy: {:.2e} J/kg", binding_energy.value());
//! ```

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