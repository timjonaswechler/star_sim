use crate::physics::units::{constants::*, core::*};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Angle, 0, 0, 0, 0, 0, 0, 0); // Dimensionless

// Define Angle units (dimensionless but physically important)
define_unit_dimension! {
    dimension Angle {
        base_unit: Radian = 1.0,
        units: {
            Radian = 1.0,
            Degree = RADIANS_PER_DEGREE,
        },
        symbols: {
            Radian = "rad",
            Degree = "°",
        }
    }
}
