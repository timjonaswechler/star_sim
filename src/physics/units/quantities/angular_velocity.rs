//! Angular velocity units for rotational motion in stellar systems.
//!
//! This module provides angular velocity units for modeling rotational properties
//! of celestial bodies, from planetary rotation to stellar spin rates.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **RadianPerSecond** (`rad/s`) - SI base unit for angular velocity
//! - **DegreePerSecond** (`°/s`) - Angular velocity in degrees per second
//!
//! # Physical Applications
//!
//! - **Planetary rotation periods** and spin rates
//! - **Stellar rotation** and differential rotation
//! - **Binary orbital motion** and synchronous rotation
//! - **Pulsar spin frequencies** and period evolution
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth's rotation rate
//! let earth_rotation = AngularVelocity::<DegreePerSecond>::new(0.004178); // ~15°/hour
//! println!("Earth rotation: {:.6} °/s", earth_rotation.value());
//!
//! // Convert to radians for physics calculations
//! let earth_rad_per_sec: AngularVelocity<RadianPerSecond> = earth_rotation.into();
//! println!("In rad/s: {:.2e}", earth_rad_per_sec.value());
//!
//! // Fast-spinning neutron star (pulsar)
//! let pulsar_spin = AngularVelocity::<RadianPerSecond>::new(1000.0); // 1000 rad/s
//! println!("Pulsar spin rate: {:.0} rad/s", pulsar_spin.value());
//! ```

use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(AngularVelocity, 0, 0, -1, 0, 0, 0, 0); // 1/Time

// Define AngularVelocity units (angle/time)
define_unit_dimension! {
    dimension AngularVelocity {
        base_unit: RadianPerSecond = 1.0,
        units: {
            RadianPerSecond = 1.0,
            DegreePerSecond = RADIANS_PER_DEGREE,
        },
        symbols: {
            RadianPerSecond = "rad/s",
            DegreePerSecond = "°/s",
        }
    }
}
