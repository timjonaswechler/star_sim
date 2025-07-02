//! Frequency units for periodic phenomena in stellar systems.
//!
//! This module provides frequency units for modeling periodic events,
//! oscillations, and wave phenomena in astronomical contexts.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Hertz** (`Hz`) - SI base unit for frequency (cycles per second)
//!
//! # Physical Applications
//!
//! - **Stellar pulsation frequencies** in variable stars
//! - **Orbital frequencies** and resonances in planetary systems
//! - **Electromagnetic radiation** and spectral line frequencies
//! - **Seismic oscillations** in stars and planets
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Pulsar rotation frequency
//! let pulsar_frequency = Frequency::<Hertz>::new(30.0); // 30 Hz rotation
//! println!("Pulsar frequency: {:.1} Hz", pulsar_frequency.value());
//!
//! // Stellar oscillation mode
//! let stellar_oscillation = Frequency::<Hertz>::new(0.003); // ~3 mHz
//! println!("Stellar oscillation: {:.3} Hz", stellar_oscillation.value());
//!
//! // Radio emission frequency
//! let radio_frequency = Frequency::<Hertz>::new(1.42e9); // 1.42 GHz (H-I line)
//! println!("Radio frequency: {:.2e} Hz", radio_frequency.value());
//! ```

use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Frequency, 0, 0, -1, 0, 0, 0, 0); // 1/Time

// Define Frequency units (1/Time)
define_unit_dimension! {
    dimension Frequency {
        base_unit: Hertz = 1.0,
        units: {
            Hertz = 1.0,
        },
        symbols: {
            Hertz = "Hz",
        }
    }
}
