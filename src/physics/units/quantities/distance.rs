use crate::physics::units::{constants::*, core::*, prefix::*};
use crate::{define_quantity, define_unit_dimension};
// Define basic quantity types using dimensional analysis
define_quantity!(Distance, 1, 0, 0, 0, 0, 0, 0); // Length

define_unit_dimension! {
    dimension Distance {
        base_unit: Meter = 1.0,
        units: {
            Meter = 1.0,
            AstronomicalUnit = METERS_PER_AU,
            EarthRadius = METERS_PER_EARTH_RADIUS,
            SunRadius = METERS_PER_SUN_RADIUS,
            LightYear = METERS_PER_LIGHT_YEAR,
            Parsec = METERS_PER_PARSEC,

        },
        symbols: {
            Meter = "m",
            AstronomicalUnit = "AU",
            EarthRadius = "R⊕",
            SunRadius = "R☉",
            LightYear = "ly",
            Parsec = "pc",
        }
    }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES FOR COMMON PREFIXED UNITS
// ================================================================================================

pub type KiloParsec = Prefixed<Kilo, Parsec>;
