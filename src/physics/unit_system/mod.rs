use serde::{Serialize, Deserialize};
use std::marker::PhantomData;

use crate::physics::constants::*;

/// Trait implemented by all unit enums. Provides conversion
/// to and from the associated SI base unit.
pub trait Unit: Copy {
    fn to_base(self, value: f64) -> f64;
    fn from_base(self, value: f64) -> f64;
    fn symbol(self) -> &'static str;
}

/// Generic quantity holding a value in the base unit of `U`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quantity<U: Unit> {
    value_si: f64,
    _unit: PhantomData<U>,
}

impl<U: Unit> Quantity<U> {
    /// Construct a quantity from a value and a unit.
    pub fn new(value: f64, unit: U) -> Self {
        Self { value_si: unit.to_base(value), _unit: PhantomData }
    }

    /// Retrieve the internal value stored in the SI base unit.
    pub fn in_base_units(&self) -> f64 {
        self.value_si
    }

    /// Get the value converted to a specific unit.
    pub fn get(&self, unit: U) -> f64 {
        unit.from_base(self.value_si)
    }
}

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MassUnit {
    Kilogram,
    Gram,
    Tonne,
    SolarMass,
    EarthMass,
}

impl Unit for MassUnit {
    fn to_base(self, value: f64) -> f64 {
        match self {
            MassUnit::Kilogram => value,
            MassUnit::Gram => value * G_TO_KG,
            MassUnit::Tonne => value * 1000.0,
            MassUnit::SolarMass => value * SOLAR_MASS_TO_KG,
            MassUnit::EarthMass => value * EARTH_MASS_TO_KG,
        }
    }

    fn from_base(self, value: f64) -> f64 {
        match self {
            MassUnit::Kilogram => value,
            MassUnit::Gram => value * KG_TO_G,
            MassUnit::Tonne => value / 1000.0,
            MassUnit::SolarMass => value * KG_TO_SOLAR_MASS,
            MassUnit::EarthMass => value * KG_TO_EARTH_MASS,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            MassUnit::Kilogram => "kg",
            MassUnit::Gram => "g",
            MassUnit::Tonne => "t",
            MassUnit::SolarMass => "M☉",
            MassUnit::EarthMass => "M⊕",
        }
    }
}

pub type Mass = Quantity<MassUnit>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Year,
    Kiloyear,
    Megayear,
    Gigayear,
}

impl Unit for TimeUnit {
    fn to_base(self, value: f64) -> f64 {
        match self {
            TimeUnit::Second => value,
            TimeUnit::Minute => value * SECONDS_PER_MINUTE,
            TimeUnit::Hour => value * SECONDS_PER_MINUTE * MINUTES_PER_HOUR,
            TimeUnit::Day => value * SECONDS_PER_DAY,
            TimeUnit::Year => value * SECONDS_PER_YEAR,
            TimeUnit::Kiloyear => value * YEARS_PER_KILOYEAR * SECONDS_PER_YEAR,
            TimeUnit::Megayear => value
                * YEARS_PER_KILOYEAR
                * KILOYEARS_PER_MEGAYEAR
                * SECONDS_PER_YEAR,
            TimeUnit::Gigayear => value
                * YEARS_PER_KILOYEAR
                * KILOYEARS_PER_MEGAYEAR
                * MEGAYEARS_PER_GIGAYEAR
                * SECONDS_PER_YEAR,
        }
    }

    fn from_base(self, value: f64) -> f64 {
        match self {
            TimeUnit::Second => value,
            TimeUnit::Minute => value * MINUTES_PER_SECONDS,
            TimeUnit::Hour => value * HOURS_PER_SECONDS,
            TimeUnit::Day => value * DAYS_PER_SECONDS,
            TimeUnit::Year => value * YEARS_PER_SECONDS,
            TimeUnit::Kiloyear => value * YEARS_PER_SECONDS * KILOYEARS_PER_YEAR,
            TimeUnit::Megayear => value
                * YEARS_PER_SECONDS
                * KILOYEARS_PER_YEAR
                * MEGAYEARS_PER_KILOYEAR,
            TimeUnit::Gigayear => value
                * YEARS_PER_SECONDS
                * KILOYEARS_PER_YEAR
                * MEGAYEARS_PER_KILOYEAR
                * GIGAYEARS_PER_MEGAYEAR,
        }
    }

    fn symbol(self) -> &'static str {
        match self {
            TimeUnit::Second => "s",
            TimeUnit::Minute => "min",
            TimeUnit::Hour => "hr",
            TimeUnit::Day => "d",
            TimeUnit::Year => "yr",
            TimeUnit::Kiloyear => "kyr",
            TimeUnit::Megayear => "Myr",
            TimeUnit::Gigayear => "Gyr",
        }
    }
}

pub type Time = Quantity<TimeUnit>;

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
            AccelerationUnit::AUPerYearSquared => value * AU_TO_M * (YEARS_PER_SECONDS * YEARS_PER_SECONDS),
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
            AccelerationUnit::AUPerYearSquared => value * M_TO_AU * (SECONDS_PER_YEAR * SECONDS_PER_YEAR),
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
