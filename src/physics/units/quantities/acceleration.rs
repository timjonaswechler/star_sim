//! Acceleration units for stellar system dynamics and orbital mechanics.
//!
//! This module provides acceleration units essential for calculating gravitational effects,
//! orbital dynamics, and stellar system evolution.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **MeterPerSecondSquared** (`m/s²`) - SI base unit for acceleration
//! - **StandardGravity** (`g₀`) - Earth's standard gravitational acceleration (≈9.81 m/s²)
//!
//! # Physical Applications
//!
//! - **Surface gravity** calculations for planets and stars
//! - **Orbital acceleration** in elliptical orbits
//! - **Tidal acceleration** effects between celestial bodies
//! - **Gravitational acceleration** at various distances from massive objects
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth's surface gravity
//! let earth_gravity = Acceleration::<StandardGravity>::new(1.0);
//! println!("Earth gravity: {}", earth_gravity);
//!
//! // Convert to m/s² for calculations
//! let gravity_mps2: Acceleration<MeterPerSecondSquared> = earth_gravity.into();
//! println!("In m/s²: {:.2}", gravity_mps2.value());
//!
//! // High acceleration near a neutron star surface
//! let neutron_star_gravity = Acceleration::<MeterPerSecondSquared>::new(1e12);
//! println!("Neutron star surface gravity: {:.2e} m/s²", neutron_star_gravity.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Acceleration, 1, 0, -2, 0, 0, 0, 0); // Length/Time²

// Define Acceleration units (Length/Time²)
define_unit_dimension! {
    dimension Acceleration {
        base_unit: MeterPerSecondSquared = 1.0,
        units: {
            MeterPerSecondSquared = 1.0,
            StandardGravity = 9.80665,
        },
        symbols: {
            MeterPerSecondSquared = "m/s²",
            StandardGravity = "g₀",
        }
    }
}
