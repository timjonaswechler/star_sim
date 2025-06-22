//! Gravitational units for celestial mechanics.
//!
//! This module defines units related to gravitational parameters and orbital mechanics,
//! which are fundamental for astronomical calculations.

use crate::physics::units::constants::*;
use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Define gravitational parameter quantity (Length³/Time²)
define_quantity!(GravitationalParameter, 3, 0, -2, 0, 0, 0, 0); // Length³/Time²

// Define GravitationalParameter units
define_unit_dimension! {
    dimension GravitationalParameter {
        base_unit: CubicMeterPerSecondSquared = 1.0,
        units: {
            CubicMeterPerSecondSquared = 1.0,
            SolarGravitationalParameter = SOLAR_GRAVITATIONAL_PARAMETER,
            EarthGravitationalParameter = EARTH_GRAVITATIONAL_PARAMETER,
        },
        symbols: {
            CubicMeterPerSecondSquared = "m³/s²",
            SolarGravitationalParameter = "GM☉",
            EarthGravitationalParameter = "GM⊕",
        }
    }
}

// Convenience type aliases
pub type SolarGM = GravitationalParameter<SolarGravitationalParameter>;
pub type EarthGM = GravitationalParameter<EarthGravitationalParameter>;