//! Magnetic flux units for electromagnetic phenomena in stellar systems.
//!
//! This module provides magnetic flux units for modeling magnetic field interactions,
//! stellar magnetospheres, and electromagnetic induction processes.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Weber** (`Wb`) - SI base unit for magnetic flux (kg⋅m²⋅s⁻²⋅A⁻¹)
//! - **Maxwell** (`Mx`) - CGS unit for magnetic flux (1 Mx = 10⁻⁸ Wb)
//!
//! # Physical Applications
//!
//! - **Stellar magnetic field** strength and topology
//! - **Planetary magnetosphere** dynamics and reconnection
//! - **Solar wind interactions** with magnetic fields
//! - **Dynamo processes** in stellar and planetary interiors
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Solar magnetic flux tube
//! let solar_flux_tube = MagneticFlux::<Weber>::new(1e15); // Strong solar field
//! println!("Solar flux tube: {:.2e} Wb", solar_flux_tube.value());
//!
//! // Convert to Maxwell units for comparison
//! let flux_maxwell: MagneticFlux<Maxwell> = solar_flux_tube.into();
//! println!("In Maxwell: {:.2e} Mx", flux_maxwell.value());
//!
//! // Planetary magnetic field flux
//! let earth_magnetic_flux = MagneticFlux::<Weber>::new(8e14); // Earth's dipole moment
//! println!("Earth magnetic flux: {:.2e} Wb", earth_magnetic_flux.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Magnetic flux (Weber) - Mass×Length²/(Current×Time²)
define_quantity!(MagneticFlux, 2, 1, -2, 0, -1, 0, 0); // Mass×Length²/(Current×Time²)

define_unit_dimension! {
    dimension MagneticFlux {
        base_unit: Weber = 1.0,
        units: {
            Weber = 1.0,
            Maxwell = 1e-8, // CGS unit
        },
        symbols: {
            Weber = "Wb",
            Maxwell = "Mx",
        }
    }
}
