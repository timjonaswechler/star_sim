use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Area, 2, 0, 0, 0, 0, 0, 0); // Length²

// Define Area units (Length²)
define_unit_dimension! {
    dimension Area {
        base_unit: SquareMeter = 1.0,
        units: {
            SquareMeter = 1.0,
            SquareKilometer = 1_000_000.0,
        },
        symbols: {
            SquareMeter = "m²",
            SquareKilometer = "km²",
        }
    }
}
