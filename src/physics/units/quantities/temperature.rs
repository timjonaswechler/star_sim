//! Temperature units for stellar and planetary thermal modeling.
//!
//! This module provides temperature units essential for modeling stellar atmospheres,
//! planetary surface conditions, and thermal processes in stellar systems.
//!
//! # Available Units
//!
//! ## Standard Temperature Units
//! - **Kelvin** (`K`) - SI base unit for thermodynamic temperature
//!
//! # Future Extensions
//!
//! Planned additions include:
//! - **Celsius** - For more familiar temperature scales
//! - **Stellar temperature scales** - Effective temperature classifications
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Stellar surface temperatures
//! let sun_surface = Temperature::<Kelvin>::new(5778.0); // Sun's effective temperature
//! let red_dwarf = Temperature::<Kelvin>::new(3500.0);   // Cool red dwarf
//! let blue_giant = Temperature::<Kelvin>::new(25000.0); // Hot blue giant
//!
//! // Planetary temperatures
//! let earth_avg = Temperature::<Kelvin>::new(288.0);    // Earth's average temperature
//! let mars_avg = Temperature::<Kelvin>::new(210.0);     // Mars average temperature
//! let venus_surface = Temperature::<Kelvin>::new(737.0); // Venus surface temperature
//!
//! // Core temperatures
//! let sun_core = Temperature::<Kelvin>::new(15_000_000.0); // Sun's core temperature
//!
//! println!("Sun surface: {}", sun_surface); // "5778 K"
//! println!("Earth: {}", earth_avg);         // "288 K"
//! ```
//!
//! # Conversion Hierarchy
//!
//! Currently uses Kelvin as the base unit. Future Celsius support will use
//! the standard conversion: K = °C + 273.15

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

/// Physical quantity representing temperature with dimensional type safety.
///
/// Dimensional analysis: `[Temperature¹]` - pure temperature dimension (Kelvin)
define_quantity!(Temperature, 0, 0, 0, 1, 0, 0, 0); // Temperature

// Define Temperature units
define_unit_dimension! {
    dimension Temperature {
        base_unit: Kelvin = 1.0,
        units: {
            Kelvin = 1.0,
            //TODO: Celsius
        },
        symbols: {
            Kelvin = "K",
        }
    }
}
