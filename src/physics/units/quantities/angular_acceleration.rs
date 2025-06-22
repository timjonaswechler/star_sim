use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(AngularAcceleration, 0, 0, -2, 0, 0, 0, 0); // 1/Time²

// Define AngularAcceleration units (angle/time²)
define_unit_dimension! {
    dimension AngularAcceleration {
        base_unit: RadianPerSecondSquared = 1.0,
        units: {
            RadianPerSecondSquared = 1.0,
            DegreePerSecondSquared = RADIANS_PER_DEGREE,
        },
        symbols: {
            RadianPerSecondSquared = "rad/s²",
            DegreePerSecondSquared = "°/s²",
        }
    }
}
