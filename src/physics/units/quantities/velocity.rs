use crate::physics::units::{
    core::*,
    quantities::{Distance, Meter, Second, Time},
};
use crate::{define_quantity, define_unit_dimension};

define_quantity!(Velocity, 1, 0, -1, 0, 0, 0, 0); // Length/Time

// Define Velocity units (Length/Time)
define_unit_dimension! {
    dimension Velocity {
        base_unit: MeterPerSecond = 1.0,
        units: {
            MeterPerSecond = 1.0,
            KilometerPerHour = 1000.0 / 3600.0,
        },
        symbols: {
            MeterPerSecond = "m/s",
            KilometerPerHour = "km/h",
        }
    }
}

// Common unit operations using helper functions
// Distance / Time = Velocity (simplified - returns value in SI units)
pub fn calculate_velocity(distance: Distance<Meter>, time: Time<Second>) -> f64 {
    divide_quantities(distance, time)
}
