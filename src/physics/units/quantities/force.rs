#![allow(non_snake_case)]
//! Force units for gravitational and mechanical interactions in stellar systems.
//!
//! This module provides force units for calculating gravitational attractions,
//! tidal forces, and other mechanical interactions between celestial bodies.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Newton** (`N`) - SI base unit for force (kg⋅m/s²)
//!
//! # Physical Applications
//!
//! - **Gravitational forces** between celestial bodies
//! - **Tidal forces** causing orbital evolution and heating
//! - **Stellar wind pressure** and radiation pressure
//! - **Magnetic forces** in stellar and planetary magnetospheres
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Gravitational force between Earth and Moon
//! let earth_moon_force = Force::<Newton>::new(1.98e20); // ~1.98×10²⁰ N
//! println!("Earth-Moon gravitational force: {:.2e} N", earth_moon_force.value());
//!
//! // Solar radiation pressure on a spacecraft
//! let radiation_pressure_force = Force::<Newton>::new(9e-6); // ~9 μN per m²
//! println!("Solar radiation force: {:.2e} N", radiation_pressure_force.value());
//!
//! // Tidal force gradient across Earth
//! let tidal_force = Force::<Newton>::new(2.2e20);
//! println!("Tidal force: {:.2e} N", tidal_force.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Force, 1, 1, -2, 0, 0, 0, 0); // Mass×Length/Time²

// Define Force units (Mass×Length/Time²)
define_unit_dimension! {
    dimension Force {
        base_unit: Newton = 1.0,
        units: {
            Newton = 1.0,
        },
        symbols: {
            Newton = "N",
        }
    }
}
