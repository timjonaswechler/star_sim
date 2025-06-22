use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(AngularVelocity, 0, 0, -1, 0, 0, 0, 0); // 1/Time

// Define AngularVelocity units (angle/time)
define_unit_dimension! {
    dimension AngularVelocity {
        base_unit: RadianPerSecond = 1.0,
        units: {
            RadianPerSecond = 1.0,
            DegreePerSecond = RADIANS_PER_DEGREE,
        },
        symbols: {
            RadianPerSecond = "rad/s",
            DegreePerSecond = "°/s",
        }
    }
}
