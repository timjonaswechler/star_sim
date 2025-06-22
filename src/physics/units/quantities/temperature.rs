use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Temperature, 0, 0, 0, 1, 0, 0, 0); // Temperature

// Define Temperature units
define_unit_dimension! {
    dimension Temperature {
        base_unit: Kelvin = 1.0,
        units: {
            Kelvin = 1.0,
            //TODO: Celsius
        },
        symbols: {
            Kelvin = "K",
        }
    }
}
