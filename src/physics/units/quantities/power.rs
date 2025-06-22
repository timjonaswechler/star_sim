use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Power, 2, 1, -3, 0, 0, 0, 0); // Mass×Length²/Time³

// Define Power units
define_unit_dimension! {
    dimension Power {
        base_unit: Watt = 1.0,
        units: {
            Watt = 1.0,
            SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
        },
        symbols: {
            Watt = "W",
            SolarLuminosity = "L☉",
        }
    }
}
