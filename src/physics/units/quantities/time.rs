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
        },
        symbols: {
            Second = "s",
            Minute = "min",
            Hour = "h",
            Day = "d",
            Year = "yr",
        }
    }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES FOR COMMON PREFIXED UNITS
// ================================================================================================
