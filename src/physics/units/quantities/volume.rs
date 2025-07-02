//! Volume units for three-dimensional measurements in stellar systems.
//!
//! This module provides volume units for calculating sizes, capacities,
//! and three-dimensional extents of celestial bodies and structures.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **CubicMeter** (`m³`) - SI base unit for volume
//! - **Liter** (`L`) - Common laboratory volume unit (10⁻³ m³)
//!
//! # Physical Applications
//!
//! - **Planetary and stellar volumes** for mass-radius relationships
//! - **Atmospheric volume calculations** and scale heights
//! - **Gas cloud and nebular volumes** in interstellar space
//! - **Satellite and asteroid volume** estimation from shape models
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Earth's volume
//! let earth_volume = Volume::<CubicMeter>::new(1.08e21); // ~1.08×10²¹ m³
//! println!("Earth volume: {:.2e} m³", earth_volume.value());
//!
//! // Small laboratory sample
//! let sample_volume = Volume::<Liter>::new(0.5); // 500 mL
//! println!("Sample volume: {:.1} L", sample_volume.value());
//!
//! // Convert to cubic meters
//! let sample_m3: Volume<CubicMeter> = sample_volume.into();
//! println!("Sample in m³: {:.4} m³", sample_m3.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Volume, 3, 0, 0, 0, 0, 0, 0); // Length³

// Define Volume units (Length³)
define_unit_dimension! {
    dimension Volume {
        base_unit: CubicMeter = 1.0,
        units: {
            CubicMeter = 1.0,
            Liter = 0.001,
        },
        symbols: {
            CubicMeter = "m³",
            Liter = "L",
        }
    }
}
