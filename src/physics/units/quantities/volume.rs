use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Volume, 3, 0, 0, 0, 0, 0, 0); // Length³

// Define Volume units (Length³)
define_unit_dimension! {
    dimension Volume {
        base_unit: CubicMeter = 1.0,
        units: {
            CubicMeter = 1.0,
            Liter = 0.001,
        },
        symbols: {
            CubicMeter = "m³",
            Liter = "L",
        }
    }
}
