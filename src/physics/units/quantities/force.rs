use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Force, 1, 1, -2, 0, 0, 0, 0); // Mass×Length/Time²

// Define Force units (Mass×Length/Time²)
define_unit_dimension! {
    dimension Force {
        base_unit: Newton = 1.0,
        units: {
            Newton = 1.0,
        },
        symbols: {
            Newton = "N",
        }
    }
}
