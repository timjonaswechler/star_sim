#![allow(non_snake_case)]
//! Power units for energy transfer rates in stellar systems.
//!
//! This module provides power units for modeling energy output, luminosity,
//! and power consumption in astronomical contexts.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Watt** (`W`) - SI base unit for power (kg⋅m²⋅s⁻³)
//! - **SolarLuminosity** (`L☉`) - Solar luminosity as reference unit (≈3.828×10²⁶ W)
//!
//! # Physical Applications
//!
//! - **Stellar luminosity** and energy output
//! - **Planetary energy budgets** and heat flow
//! - **Accretion disk power** and radiative efficiency
//! - **Tidal heating rates** in satellite systems
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Solar power output
//! let solar_power = Power::<SolarLuminosity>::new(1.0);
//! println!("Solar luminosity: {:.1} L☉", solar_power.value());
//!
//! // Convert to watts for calculations
//! let solar_watts: Power<Watt> = solar_power.into();
//! println!("Solar power: {:.3e} W", solar_watts.value());
//!
//! // Red dwarf star luminosity
//! let red_dwarf = Power::<SolarLuminosity>::new(0.1); // 10% of solar luminosity
//! println!("Red dwarf luminosity: {:.1} L☉", red_dwarf.value());
//! ```

use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Power, 2, 1, -3, 0, 0, 0, 0); // Mass×Length²/Time³

// Define Power units
define_unit_dimension! {
    dimension Power {
        base_unit: Watt = 1.0,
        units: {
            Watt = 1.0,
            SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
        },
        symbols: {
            Watt = "W",
            SolarLuminosity = "L☉",
        }
    }
}
