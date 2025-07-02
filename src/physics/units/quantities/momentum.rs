//! Momentum units for motion and collision dynamics in stellar systems.
//!
//! This module provides momentum units for modeling the motion of celestial bodies,
//! impact events, and momentum transfer processes.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **KilogramMeterPerSecond** (`kg⋅m/s`) - SI base unit for momentum
//!
//! # Physical Applications
//!
//! - **Orbital momentum** of planets and satellites
//! - **Stellar wind momentum** transfer and pressure
//! - **Impact momentum** in collision events
//! - **Angular momentum** conservation in rotating systems
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth's orbital momentum around the Sun
//! let earth_orbital_momentum = Momentum::<KilogramMeterPerSecond>::new(1.78e29);
//! println!("Earth orbital momentum: {:.2e} kg⋅m/s", earth_orbital_momentum.value());
//!
//! // Asteroid impact momentum
//! let asteroid_momentum = Momentum::<KilogramMeterPerSecond>::new(1e15); // Large asteroid
//! println!("Asteroid momentum: {:.2e} kg⋅m/s", asteroid_momentum.value());
//!
//! // Solar wind particle momentum
//! let solar_wind_momentum = Momentum::<KilogramMeterPerSecond>::new(1e-18); // Single proton
//! println!("Solar wind particle: {:.2e} kg⋅m/s", solar_wind_momentum.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Additional derived quantities
define_quantity!(Momentum, 1, 1, -1, 0, 0, 0, 0); // Mass×Length/Time

// Define Momentum units (Mass×Length/Time)
define_unit_dimension! {
    dimension Momentum {
        base_unit: KilogramMeterPerSecond = 1.0,
        units: {
            KilogramMeterPerSecond = 1.0,
        },
        symbols: {
            KilogramMeterPerSecond = "kg⋅m/s",
        }
    }
}
