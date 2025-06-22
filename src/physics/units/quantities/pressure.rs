use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Pressure, -1, 1, -2, 0, 0, 0, 0); // Mass/(Length×Time²)

// Define Pressure units (Mass/(Length×Time²))
define_unit_dimension! {
    dimension Pressure {
        base_unit: Pascal = 1.0,
        units: {
            Pascal = 1.0,
            Bar = 100_000.0,
        },
        symbols: {
            Pascal = "Pa",
            Bar = "bar",
        }
    }
}
