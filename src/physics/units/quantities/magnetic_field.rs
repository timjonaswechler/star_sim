#![allow(non_snake_case)]
//! Magnetic field units for electromagnetic phenomena in stellar systems.
//!
//! This module defines units for magnetic field strength and related electromagnetic
//! quantities used in astrophysical calculations, stellar magnetism, and plasma physics.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Tesla** (`T`) - SI base unit for magnetic field strength (kg⋅s⁻²⋅A⁻¹)
//! - **Gauss** (`G`) - CGS unit for magnetic field strength (1 G = 10⁻⁴ T)
//!
//! # Physical Applications
//!
//! - **Stellar magnetic fields** and solar activity cycles
//! - **Planetary magnetospheres** and magnetic shielding
//! - **Pulsar magnetic field** strengths and spin-down rates
//! - **Interstellar magnetic fields** and cosmic ray propagation
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth's magnetic field at surface
//! let earth_field = MagneticField::<Gauss>::new(0.5); // ~0.5 Gauss
//! println!("Earth magnetic field: {:.1} G", earth_field.value());
//!
//! // Convert to Tesla for SI calculations
//! let earth_tesla: MagneticField<Tesla> = earth_field.into();
//! println!("Earth field in Tesla: {:.2e} T", earth_tesla.value());
//!
//! // Strong pulsar magnetic field
//! let pulsar_field = MagneticField::<Tesla>::new(1e8); // 100 million Tesla
//! println!("Pulsar magnetic field: {:.2e} T", pulsar_field.value());
//!
//! // Using convenience type aliases
//! let solar_field = TeslaField::new(0.01); // 0.01 T = 100 G
//! let stellar_field = GaussField::new(1000.0); // 1000 G field
//! println!("Solar field: {:.2} T", solar_field.value());
//! println!("Stellar field: {:.0} G", stellar_field.value());
//! ```

use crate::physics::units::constants::*;
use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Define magnetic field quantity (Mass/(Current×Time²))
define_quantity!(MagneticField, 0, 1, -2, 0, -1, 0, 0); // Mass/(Current×Time²)

// Define MagneticField units
define_unit_dimension! {
    dimension MagneticField {
        base_unit: Tesla = 1.0,
        units: {
            Tesla = 1.0,
            Gauss = TESLA_PER_GAUSS,

        },
        symbols: {
            Tesla = "T",
            Gauss = "G",
        }
    }
}

// Magnetic flux density is the same as magnetic field, but sometimes distinguished
pub type MagneticFluxDensity<U> = MagneticField<U>;

// Convenience type aliases
pub type TeslaField = MagneticField<Tesla>;
pub type GaussField = MagneticField<Gauss>;
