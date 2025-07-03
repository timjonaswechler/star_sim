#![allow(non_snake_case)]
//! Area units for surface calculations in stellar systems.
//!
//! This module provides area units for calculating surface areas, cross-sections,
//! and other two-dimensional measurements in astronomical contexts.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **SquareMeter** (`m²`) - SI base unit for area
//! - **SquareKilometer** (`km²`) - Larger areas in square kilometers
//!
//! # Physical Applications
//!
//! - **Planetary surface areas** and continental scales
//! - **Stellar photosphere areas** for luminosity calculations
//! - **Cross-sectional areas** for collision and interaction calculations
//! - **Orbital sweep areas** for Kepler's second law
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth's surface area
//! let earth_surface = Area::<SquareKilometer>::new(510_100_000.0); // ~510 million km²
//! println!("Earth surface area: {:.0} km²", earth_surface.value());
//!
//! // Convert to square meters for detailed calculations
//! let earth_m2: Area<SquareMeter> = earth_surface.into();
//! println!("In m²: {:.2e}", earth_m2.value());
//!
//! // Cross-sectional area of a small asteroid
//! let asteroid_cross_section = Area::<SquareMeter>::new(1000.0);
//! println!("Asteroid cross-section: {:.0} m²", asteroid_cross_section.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Area, 2, 0, 0, 0, 0, 0, 0); // Length²

// Define Area units (Length²)
define_unit_dimension! {
    dimension Area {
        base_unit: SquareMeter = 1.0,
        units: {
            SquareMeter = 1.0,
            SquareKilometer = 1_000_000.0,
        },
        symbols: {
            SquareMeter = "m²",
            SquareKilometer = "km²",
        }
    }
}
