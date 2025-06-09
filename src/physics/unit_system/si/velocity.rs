use crate::physics::constants::*;
use crate::physics::unit_system::{Quantity, Unit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VelocityUnit {
    MeterPerSecond,
    KilometerPerSecond,
    KilometerPerHour,
    AUPerYear,
}

impl Unit for VelocityUnit {
    fn to_base(self, value: f64) -> f64 {
        match self {
            VelocityUnit::MeterPerSecond => value,
            VelocityUnit::KilometerPerSecond => value * KM_TO_M,
            VelocityUnit::KilometerPerHour => value * KM_TO_M * HOURS_PER_SECONDS,
            VelocityUnit::AUPerYear => value * AU_TO_M * YEARS_PER_SECONDS,
        }
    }

    fn from_base(self, value: f64) -> f64 {
        match self {
            VelocityUnit::MeterPerSecond => value,
            VelocityUnit::KilometerPerSecond => value * M_TO_KM,
            VelocityUnit::KilometerPerHour => value * SECONDS_PER_HOUR * M_TO_KM,
            VelocityUnit::AUPerYear => value * M_TO_AU * SECONDS_PER_YEAR,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            VelocityUnit::MeterPerSecond => "m/s",
            VelocityUnit::KilometerPerSecond => "km/s",
            VelocityUnit::KilometerPerHour => "km/h",
            VelocityUnit::AUPerYear => "AU/yr",
        }
    }
}

pub type Velocity = Quantity<VelocityUnit>;
