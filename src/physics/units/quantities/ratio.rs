#![allow(non_snake_case)]
//! Dimensionless ratio and factor units for scientific calculations.
//!
//! This module provides dimensionless units for expressing ratios, factors, and
//! other dimensionless quantities common in stellar physics and astronomy.
//!
//! # Available Units
//!
//! ## Ratio Units
//! - **Unit** - Base dimensionless unit (value = 1.0)
//! - **Percent** (%) - Percentage (1% = 0.01)
//! - **PartsPerMillion** (ppm) - Parts per million (1 ppm = 1e-6)
//! - **PartsPerBillion** (ppb) - Parts per billion (1 ppb = 1e-9)
//! - **Fraction** - Alias for Unit (same as base unit)
//!
//! # Applications
//!
//! ## Stellar Physics
//! - Metallicity ratios (e.g., Fe/H abundance ratios)
//! - Mass fractions in stellar composition
//! - Efficiency factors for stellar processes
//!
//! ## Planetary Science
//! - Atmospheric composition ratios
//! - Albedo values (dimensionless reflectivity)
//! - Eccentricity values for orbital ellipses
//!
//! ## General Science
//! - Concentration ratios
//! - Scaling factors
//! - Dimensionless physical parameters
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Stellar metallicity (typically expressed as ratio to solar)
//! let solar_metallicity = Ratio::<Unit>::new(1.0); // Solar reference
//! let metal_poor_star = Ratio::<Unit>::new(0.1); // 10% of solar metallicity
//!
//! // Orbital eccentricity (dimensionless, 0 = circular, 1 = parabolic)
//! let earth_eccentricity = Ratio::<Unit>::new(0.0167); // Earth's orbital eccentricity
//! let mars_eccentricity = Ratio::<Unit>::new(0.0934); // Mars' orbital eccentricity
//!
//! // Atmospheric composition
//! let co2_concentration = Ratio::<PartsPerMillion>::new(415.0); // ~415 ppm CO2
//! let o2_percentage = Ratio::<Percent>::new(21.0); // 21% oxygen
//!
//! // Convert between units
//! let co2_as_percent = co2_concentration.convert_to::<Percent>();
//! let o2_as_fraction = o2_percentage.convert_to::<Unit>();
//!
//! println!("CO2 concentration: {}", co2_concentration); // "415 ppm"
//! println!("As percentage: {}", co2_as_percent); // "0.0415 %"
//! println!("O2 as fraction: {}", o2_as_fraction); // "0.21"
//! ```
//!
//! # Conversion Hierarchy
//!
//! All conversions use the base Unit (1.0) as the hub:
//! - Other units → Unit → Target unit
//! - Maintains precision across different scales

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Ratio, 0, 0, 0, 0, 0, 0, 0); // Dimensionless

// Define Ratio units (dimensionless)
define_unit_dimension! {
    dimension Ratio {
        base_unit: Unit = 1.0,
        units: {
            Unit = 1.0,
            Fraction = 1.0, // Alias for Unit
            Percent = 0.01,
            PartsPerMillion = 1e-6,
            PartsPerBillion = 1e-9,
        },
        symbols: {
            Unit = "",
            Fraction = "",
            Percent = "%",
            PartsPerMillion = "ppm",
            PartsPerBillion = "ppb",
        }
    }
}
