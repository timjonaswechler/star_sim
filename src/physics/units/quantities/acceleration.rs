use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Acceleration, 1, 0, -2, 0, 0, 0, 0); // Length/Time²

// Define Acceleration units (Length/Time²)
define_unit_dimension! {
    dimension Acceleration {
        base_unit: MeterPerSecondSquared = 1.0,
        units: {
            MeterPerSecondSquared = 1.0,
            StandardGravity = 9.80665,
        },
        symbols: {
            MeterPerSecondSquared = "m/s²",
            StandardGravity = "g₀",
        }
    }
}
