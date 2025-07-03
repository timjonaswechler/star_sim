#![allow(non_snake_case)]
//! Velocity units for orbital mechanics and stellar motion calculations.
//!
//! This module provides velocity units essential for modeling orbital dynamics,
//! stellar motions, and relativistic effects in stellar systems.
//!
//! # Available Units
//!
//! ## Standard Velocity Units
//! - **MeterPerSecond** (`m/s`) - SI base unit for velocity
//! - **KilometerPerHour** (`km/h`) - Common terrestrial velocity unit
//!
//! # Applications
//!
//! ## Orbital Mechanics
//! - Orbital velocities (circular, elliptical, hyperbolic)
//! - Escape velocities from planets and stars
//! - Orbital insertion and transfer maneuvers
//!
//! ## Stellar Dynamics
//! - Stellar proper motions
//! - Galaxy rotation curves
//! - Stellar wind velocities
//!
//! # Key Velocity Scales
//!
//! - **Earth orbital velocity**: ~29.8 km/s
//! - **Solar escape velocity**: ~617.5 km/s
//! - **Galactic orbital velocity**: ~220 km/s
//! - **Speed of light**: ~299,792,458 m/s
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Orbital velocities
//! let earth_orbital = Velocity::<MeterPerSecond>::new(29800.0); // Earth around Sun
//! let escape_earth = Velocity::<MeterPerSecond>::new(11200.0);  // Earth escape velocity
//!
//! // Convert between units
//! let earth_orbital_kmh = earth_orbital.convert_to::<KilometerPerHour>();
//!
//! // Calculate velocity from distance and time
//! let distance = Distance::<Meter>::new(1000.0);
//! let time = Time::<Second>::new(10.0);
//! let calculated_velocity = calculate_velocity(distance, time); // Returns m/s
//!
//! println!("Earth orbital: {}", earth_orbital); // "29800 m/s"
//! println!("In km/h: {}", earth_orbital_kmh);   // "107280 km/h"
//! ```
//!
//! # Dimensional Analysis
//!
//! Velocity has dimensions `[Length¹ Time⁻¹]`, enabling compile-time checking
//! of physics equations involving motion.
//!
//! # Conversion Hierarchy
//!
//! All conversions use meters per second as the hub unit:
//! - Other units → m/s → Target unit

use crate::physics::units::{
    core::*,
    quantities::{Distance, Meter, Second, Time},
};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Velocity, 1, 0, -1, 0, 0, 0, 0); // Length/Time

// Define Velocity units (Length/Time)
define_unit_dimension! {
    dimension Velocity {
        base_unit: MeterPerSecond = 1.0,
        units: {
            MeterPerSecond = 1.0,
            KilometerPerHour = 1000.0 / 3600.0,
        },
        symbols: {
            MeterPerSecond = "m/s",
            KilometerPerHour = "km/h",
        }
    }
}

// Common unit operations using helper functions
// Distance / Time = Velocity (simplified - returns value in SI units)
pub fn calculate_velocity(distance: Distance<Meter>, time: Time<Second>) -> f64 {
    divide_quantities(distance, time)
}
