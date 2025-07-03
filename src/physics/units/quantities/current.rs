#![allow(non_snake_case)]
//! Electric current units for electromagnetic phenomena in stellar systems.
//!
//! This module provides electric current units essential for modeling stellar magnetic fields,
//! charged particle flows, and electromagnetic processes in stellar atmospheres and space.
//!
//! # Available Units
//!
//! ## Standard Current Units
//! - **Ampere** (`A`) - SI base unit for electric current
//! - **Milliampere** (`mA`) - Common for low-current applications
//! - **Microampere** (`μA`) - Very low currents in space plasmas
//! - **Kiloampere** (`kA`) - Large currents in stellar magnetic fields
//!
//! # Applications
//!
//! ## Stellar Physics
//! - Stellar magnetic field strengths and current systems
//! - Solar wind particle currents
//! - Coronal mass ejection current sheets
//!
//! ## Planetary Magnetospheres
//! - Magnetospheric current systems
//! - Auroral current circuits
//! - Ring current intensities
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Stellar magnetic currents
//! let solar_current = Current::<Kiloampere>::new(1e9); // Massive solar currents
//! let stellar_wind_current = Current::<Ampere>::new(1e6); // Solar wind currents
//!
//! // Planetary magnetosphere currents
//! let auroral_current = Current::<Kiloampere>::new(1.0); // Auroral current system
//! let ring_current = Current::<Ampere>::new(1e4); // Planetary ring current
//!
//! // Low-level space currents
//! let plasma_current = Current::<Microampere>::new(100.0); // Plasma measurements
//!
//! // Convert between units
//! let current_amps = solar_current.convert_to::<Ampere>();
//!
//! println!("Solar current: {}", solar_current); // "1e9 kA"
//! println!("In amperes: {}", current_amps);
//! ```
//!
//! # Conversion Hierarchy
//!
//! All conversions use amperes as the hub unit:
//! - Other units → Amperes → Target unit
//! - Supports both tiny space currents and massive stellar currents

use crate::define_quantity;
use crate::define_unit_dimension;
use crate::physics::units::core::*;

define_quantity!(Current, 0, 0, 0, 0, 1, 0, 0); // Current

// Define Current units
define_unit_dimension! {
    dimension Current {
        base_unit: Ampere = 1.0,
        units: {
            Ampere = 1.0,

        },
        symbols: {
            Ampere = "A",

        }
    }
}
