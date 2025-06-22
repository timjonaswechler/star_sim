use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Magnetic flux (Weber) - Mass×Length²/(Current×Time²)
define_quantity!(MagneticFlux, 2, 1, -2, 0, -1, 0, 0); // Mass×Length²/(Current×Time²)

define_unit_dimension! {
    dimension MagneticFlux {
        base_unit: Weber = 1.0,
        units: {
            Weber = 1.0,
            Maxwell = 1e-8, // CGS unit
        },
        symbols: {
            Weber = "Wb",
            Maxwell = "Mx",
        }
    }
}
