use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Density, -3, 1, 0, 0, 0, 0, 0); // Mass/Length³

// Define Density units (Mass/Length³)
define_unit_dimension! {
    dimension Density {
        base_unit: KilogramPerCubicMeter = 1.0,
        units: {
            KilogramPerCubicMeter = 1.0,
            GramPerCubicCentimeter = 1000.0,
        },
        symbols: {
            KilogramPerCubicMeter = "kg/m³",
            GramPerCubicCentimeter = "g/cm³",
        }
    }
}
