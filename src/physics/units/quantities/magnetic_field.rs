//! Magnetic field units for electromagnetic calculations.
//!
//! This module defines units for magnetic field strength and related electromagnetic
//! quantities used in astrophysical calculations.

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
