use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Additional derived quantities
define_quantity!(Momentum, 1, 1, -1, 0, 0, 0, 0); // Mass×Length/Time

// Define Momentum units (Mass×Length/Time)
define_unit_dimension! {
    dimension Momentum {
        base_unit: KilogramMeterPerSecond = 1.0,
        units: {
            KilogramMeterPerSecond = 1.0,
        },
        symbols: {
            KilogramMeterPerSecond = "kg⋅m/s",
        }
    }
}
