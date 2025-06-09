use crate::physics::constants::*;
use crate::physics::unit_system::{Quantity, Unit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccelerationUnit {
    MeterPerSecondSquared,
    KilometerPerSecondSquared,
    MeterPerMinuteSquared,
    AUPerYearSquared,
}

impl Unit for AccelerationUnit {
    fn to_base(self, value: f64) -> f64 {
        match self {
            AccelerationUnit::MeterPerSecondSquared => value,
            AccelerationUnit::KilometerPerSecondSquared => value * KM_TO_M,
            AccelerationUnit::MeterPerMinuteSquared => {
                let sec2 = SECONDS_PER_MINUTE * SECONDS_PER_MINUTE;
                value / sec2
            }
            AccelerationUnit::AUPerYearSquared => {
                value * AU_TO_M * (YEARS_PER_SECONDS * YEARS_PER_SECONDS)
            }
        }
    }

    fn from_base(self, value: f64) -> f64 {
        match self {
            AccelerationUnit::MeterPerSecondSquared => value,
            AccelerationUnit::KilometerPerSecondSquared => value * M_TO_KM,
            AccelerationUnit::MeterPerMinuteSquared => {
                let sec2 = SECONDS_PER_MINUTE * SECONDS_PER_MINUTE;
                value * sec2
            }
            AccelerationUnit::AUPerYearSquared => {
                value * M_TO_AU * (SECONDS_PER_YEAR * SECONDS_PER_YEAR)
            }
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            AccelerationUnit::MeterPerSecondSquared => "m/s²",
            AccelerationUnit::KilometerPerSecondSquared => "km/s²",
            AccelerationUnit::MeterPerMinuteSquared => "m/min²",
            AccelerationUnit::AUPerYearSquared => "AU/yr²",
        }
    }
}

pub type Acceleration = Quantity<AccelerationUnit>;
