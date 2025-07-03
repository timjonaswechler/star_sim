#![allow(non_snake_case)]
//! Pressure units for atmospheric and interior conditions in stellar systems.
//!
//! This module provides pressure units for modeling atmospheric conditions,
//! interior pressures, and fluid dynamics in celestial bodies.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Pascal** (`Pa`) - SI base unit for pressure (N/m² or kg⋅m⁻¹⋅s⁻²)
//! - **Bar** (`bar`) - Common atmospheric pressure unit (10⁵ Pa)
//!
//! # Physical Applications
//!
//! - **Planetary atmospheric pressure** at various altitudes
//! - **Stellar interior pressure** and hydrostatic equilibrium
//! - **Gas giant atmospheric** structure and dynamics
//! - **Radiation pressure** in stellar atmospheres
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth atmospheric pressure at sea level
//! let earth_sea_level = Pressure::<Bar>::new(1.0); // 1 bar
//! println!("Earth sea level pressure: {:.1} bar", earth_sea_level.value());
//!
//! // Convert to Pascals for calculations
//! let pressure_pascals: Pressure<Pascal> = earth_sea_level.into();
//! println!("In Pascals: {:.0} Pa", pressure_pascals.value());
//!
//! // Extreme pressure in stellar core
//! let stellar_core_pressure = Pressure::<Pascal>::new(1e16); // 10¹⁶ Pa
//! println!("Stellar core pressure: {:.2e} Pa", stellar_core_pressure.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Pressure, -1, 1, -2, 0, 0, 0, 0); // Mass/(Length×Time²)

// Define Pressure units (Mass/(Length×Time²))
define_unit_dimension! {
    dimension Pressure {
        base_unit: Pascal = 1.0,
        units: {
            Pascal = 1.0,
            Bar = 100_000.0,
        },
        symbols: {
            Pascal = "Pa",
            Bar = "bar",
        }
    }
}
