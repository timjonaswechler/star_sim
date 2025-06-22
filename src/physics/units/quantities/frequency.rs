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
