//! Angular measurement units for orbital mechanics and stellar rotation.
//!
//! This module provides angular units essential for modeling orbital inclinations,
//! stellar rotation, precession, and other rotational phenomena in stellar systems.
//!
//! # Available Units
//!
//! ## Standard Angular Units
//! - **Radian** (`rad`) - SI base unit for plane angles
//! - **Degree** (`°`) - Traditional degree unit (π/180 radians)
//! - **Arcminute** (`'`) - 1/60 of a degree
//! - **Arcsecond** (`"`) - 1/3600 of a degree (crucial for astrometry)
//! - **Milliarcsecond** (`mas`) - 1/1000 arcsecond (stellar parallax measurements)
//!
//! # Applications
//!
//! ## Orbital Mechanics
//! - Orbital inclinations and node angles
//! - Argument of periapsis and true anomaly
//! - Precession rates and libration amplitudes
//!
//! ## Stellar Astronomy
//! - Stellar parallax measurements (milliarcseconds)
//! - Proper motion components
//! - Binary star orbital positions
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Orbital inclinations
//! let earth_inclination = Angle::<Degree>::new(0.0); // Earth's orbital inclination
//! let pluto_inclination = Angle::<Degree>::new(17.16); // Pluto's steep inclination
//!
//! // Stellar parallax measurements
//! let proxima_parallax = Angle::<Milliarcsecond>::new(768.5); // Proxima Centauri
//! let sirius_parallax = Angle::<Milliarcsecond>::new(379.21); // Sirius
//!
//! // Convert to radians for calculations
//! let inclination_rad = earth_inclination.convert_to::<Radian>();
//!
//! // Very precise measurements
//! let stellar_diameter = Angle::<Milliarcsecond>::new(0.5); // Angular diameter
//!
//! println!("Proxima parallax: {}", proxima_parallax); // "768.5 mas"
//! println!("In degrees: {}", proxima_parallax.convert_to::<Degree>()); 
//! ```
//!
//! # Conversion Hierarchy
//!
//! All conversions use radians as the hub unit:
//! - Other units → Radians → Target unit
//! - Maintains precision for very small angles (milliarcseconds)

use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

/// Physical quantity representing angles with dimensional type safety.
///
/// Dimensional analysis: `[dimensionless]` - angles are dimensionless ratios
define_quantity!(Angle, 0, 0, 0, 0, 0, 0, 0); // Dimensionless

// Define Angle units (dimensionless but physically important)
define_unit_dimension! {
    dimension Angle {
        base_unit: Radian = 1.0,
        units: {
            Radian = 1.0,
            Degree = RADIANS_PER_DEGREE,
            Arcminute = RADIANS_PER_DEGREE / 60.0,
            Arcsecond = RADIANS_PER_DEGREE / 3600.0,
            Milliarcsecond = RADIANS_PER_DEGREE / 3_600_000.0,
        },
        symbols: {
            Radian = "rad",
            Degree = "°",
            Arcminute = "'",
            Arcsecond = "\"",
            Milliarcsecond = "mas",
        }
    }
}
