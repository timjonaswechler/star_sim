#![allow(non_snake_case)]
//! Time units for stellar system calculations and evolution modeling.
//!
//! This module provides time units spanning from seconds to astronomical timescales,
//! essential for modeling stellar evolution, orbital dynamics, and system age.
//!
//! # Available Units
//!
//! ## Standard Time Units
//! - **Second** (`s`) - SI base unit for time
//! - **Minute** (`min`) - 60 seconds
//! - **Hour** (`h`) - 3,600 seconds
//! - **Day** (`d`) - 86,400 seconds
//! - **Year** (`yr`) - 365.25 days (31,557,600 seconds)
//!
//! ## Astronomical Time Units
//! - **Kiloyear** (`kyr`) - 1,000 years (recent geological/astronomical events)
//! - **Megayear** (`Myr`) - 1 million years (stellar evolution timescales)
//! - **Gigayear** (`Gyr`) - 1 billion years (cosmic evolution, stellar lifetimes)
//!
//! # Design Philosophy
//!
//! Time units are designed to support both:
//! - **Short-term dynamics**: Orbital periods, rotational periods
//! - **Long-term evolution**: Stellar lifetimes, system ages
//!
//! For very long timescales (gigayears), consider using prefixed units
//! or custom scaling in your calculations.
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Orbital periods
//! let earth_year = Time::<Year>::new(1.0);
//! let earth_year_seconds = earth_year.convert_to::<Second>();
//!
//! // Stellar rotation
//! let sun_rotation = Time::<Day>::new(25.4); // Sun's rotation period
//!
//! // Short timescales
//! let light_travel_au = Time::<Minute>::new(8.3); // Light travel time for 1 AU
//!
//! // Convert between units
//! let orbital_period_days = Time::<Day>::new(365.25);
//! let orbital_period_hours = orbital_period_days.convert_to::<Hour>();
//! let orbital_period_seconds = orbital_period_days.convert_to::<Second>();
//!
//! println!("Earth year: {}", earth_year); // "1 yr"
//! println!("In seconds: {}", earth_year_seconds); // "31557600 s"
//! ```
//!
//! # Conversion Hierarchy
//!
//! All conversions use seconds as the hub unit:
//! - Other units → Seconds → Target unit
//! - Maintains precision for both short and long timescales

use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Time, 0, 0, 1, 0, 0, 0, 0); // Time

// Define Time units with astronomical focus
define_unit_dimension! {
    dimension Time {
        base_unit: Second = 1.0,
        units: {
            Second = 1.0,
            Minute = SECONDS_PER_MINUTE,
            Hour = SECONDS_PER_HOUR,
            Day = SECONDS_PER_DAY,
            Year = SECONDS_PER_YEAR,
            Kiloyear = SECONDS_PER_YEAR * 1000.0,
            Megayear = SECONDS_PER_YEAR * 1e6,
            Gigayear = SECONDS_PER_YEAR * 1e9,
        },
        symbols: {
            Second = "s",
            Minute = "min",
            Hour = "h",
            Day = "d",
            Year = "yr",
            Kiloyear = "kyr",
            Megayear = "Myr",
            Gigayear = "Gyr",
        }
    }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES FOR COMMON PREFIXED UNITS
// ================================================================================================
