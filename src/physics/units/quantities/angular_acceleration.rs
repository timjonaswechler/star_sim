//! Angular acceleration units for rotational dynamics in stellar systems.
//!
//! This module provides angular acceleration units for modeling rotational changes
//! in celestial bodies, from planetary rotation to stellar spin evolution.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **RadianPerSecondSquared** (`rad/s²`) - SI base unit for angular acceleration
//! - **DegreePerSecondSquared** (`°/s²`) - Angular acceleration in degrees per second squared
//!
//! # Physical Applications
//!
//! - **Stellar spin-up/spin-down** during evolution phases
//! - **Planetary rotation changes** due to tidal interactions
//! - **Binary system synchronization** and orbital decay
//! - **Precession acceleration** of rotational axes
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Gradual spin-down of a pulsar
//! let pulsar_spindown = AngularAcceleration::<RadianPerSecondSquared>::new(-1e-15);
//! println!("Pulsar spin-down: {:.2e} rad/s²", pulsar_spindown.value());
//!
//! // Convert to degrees for easier visualization
//! let spindown_degrees: AngularAcceleration<DegreePerSecondSquared> = pulsar_spindown.into();
//! println!("In degrees: {:.2e} °/s²", spindown_degrees.value());
//!
//! // Tidal acceleration of planetary rotation
//! let tidal_acceleration = AngularAcceleration::<DegreePerSecondSquared>::new(1e-12);
//! println!("Tidal angular acceleration: {:.2e} °/s²", tidal_acceleration.value());
//! ```

use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(AngularAcceleration, 0, 0, -2, 0, 0, 0, 0); // 1/Time²

// Define AngularAcceleration units (angle/time²)
define_unit_dimension! {
    dimension AngularAcceleration {
        base_unit: RadianPerSecondSquared = 1.0,
        units: {
            RadianPerSecondSquared = 1.0,
            DegreePerSecondSquared = RADIANS_PER_DEGREE,
        },
        symbols: {
            RadianPerSecondSquared = "rad/s²",
            DegreePerSecondSquared = "°/s²",
        }
    }
}
