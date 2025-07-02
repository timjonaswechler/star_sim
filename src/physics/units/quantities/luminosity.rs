//! Luminosity units for stellar brightness and energy output calculations.
//!
//! This module provides luminosity units essential for modeling stellar energy output,
//! brightness measurements, and radiative processes in stellar systems.
//!
//! # Available Units
//!
//! ## Standard Luminosity Units
//! - **Watt** (`W`) - SI base unit for power/luminosity
//! - **SolarLuminosity** (`L☉`) - Luminosity of the Sun (3.828 × 10²⁶ W)
//! - **ErgPerSecond** (`erg/s`) - CGS unit common in astrophysics
//! - **Kilowatt** (`kW`) - Larger terrestrial power scales
//! - **Megawatt** (`MW`) - Very large power outputs
//!
//! # Applications
//!
//! ## Stellar Evolution
//! - Main sequence stellar luminosities
//! - Giant and supergiant energy outputs
//! - White dwarf cooling luminosities
//!
//! ## Comparative Stellar Physics
//! - M-dwarf vs solar luminosity ratios
//! - Binary star component luminosities
//! - Variable star brightness changes
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Stellar luminosities
//! let sun_luminosity = Luminosity::<SolarLuminosity>::new(1.0); // Reference
//! let red_dwarf = Luminosity::<SolarLuminosity>::new(0.0001); // Very dim M-dwarf
//! let blue_giant = Luminosity::<SolarLuminosity>::new(10000.0); // Massive hot star
//!
//! // Convert to watts for calculations
//! let sun_watts = sun_luminosity.convert_to::<Watt>();
//! let dwarf_watts = red_dwarf.convert_to::<Watt>();
//!
//! // Very luminous objects
//! let supergiant = Luminosity::<SolarLuminosity>::new(100000.0); // Red supergiant
//!
//! // Convert to CGS for astrophysics papers
//! let sun_cgs = sun_luminosity.convert_to::<ErgPerSecond>();
//!
//! println!("Sun luminosity: {}", sun_luminosity); // "1 L☉"
//! println!("Red dwarf: {}", red_dwarf); // "0.0001 L☉"
//! println!("In watts: {}", sun_watts);
//! ```
//!
//! # Stellar Classification Context
//!
//! Common stellar luminosity ranges:
//! - **M-dwarfs**: 0.0001 - 0.1 L☉
//! - **Sun (G-dwarf)**: 1.0 L☉ (reference)
//! - **A-stars**: 10 - 100 L☉
//! - **B-stars**: 100 - 10,000 L☉
//! - **Supergiants**: 10,000 - 1,000,000 L☉
//!
//! # Conversion Hierarchy
//!
//! All conversions use watts as the hub unit:
//! - Other units → Watts → Target unit
//! - Maintains precision across enormous luminosity ranges

use crate::define_quantity;
use crate::define_unit_dimension;
use crate::physics::units::{constants::*, core::*, prefix::*};

/// Physical quantity representing luminosity (power output) with dimensional type safety.
///
/// Dimensional analysis: `[Length² Mass¹ Time⁻³]` - power/luminosity dimensions
define_quantity!(Luminosity, 2, 1, -3, 0, 0, 0, 0); // ML²T⁻³

// Define Luminosity units (reusing Power infrastructure)
define_unit_dimension! {
    dimension Luminosity {
        base_unit: LuminosityWatt = 1.0,
        units: {
            LuminosityWatt = 1.0,
            SolarLuminosityUnit = WATTS_PER_SOLAR_LUMINOSITY,
            ErgPerSecond = 1e-7, // 1 erg/s = 1e-7 W

        },
        symbols: {
            LuminosityWatt = "W",
            SolarLuminosityUnit = "L☉",
            ErgPerSecond = "erg/s",

        }
    }
}
