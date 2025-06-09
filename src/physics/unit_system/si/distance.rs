use crate::physics::constants::*;
use crate::physics::unit_system::{Quantity, Unit};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DistanceUnit {
    Meter,
    Kilometer,
    AstronomicalUnit,
    EarthRadius,
}

impl Unit for DistanceUnit {
    fn to_base(self, value: f64) -> f64 {
        match self {
            DistanceUnit::Meter => value,
            DistanceUnit::Kilometer => value * KM_TO_M,
            DistanceUnit::AstronomicalUnit => value * AU_TO_M,
            DistanceUnit::EarthRadius => value * EARTH_RADIUS_IN_METERS,
        }
    }

    fn from_base(self, value: f64) -> f64 {
        match self {
            DistanceUnit::Meter => value,
            DistanceUnit::Kilometer => value * M_TO_KM,
            DistanceUnit::AstronomicalUnit => value * M_TO_AU,
            DistanceUnit::EarthRadius => value * METERS_TO_EARTH_RADIUS,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            DistanceUnit::Meter => "m",
            DistanceUnit::Kilometer => "km",
            DistanceUnit::AstronomicalUnit => "AU",
            DistanceUnit::EarthRadius => "R⊕",
        }
    }
}

pub type Distance = Quantity<DistanceUnit>;
