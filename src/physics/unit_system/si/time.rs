use crate::physics::constants::*;
use crate::physics::unit_system::{Quantity, Unit};
use serde::{Deserialize, Serialize};

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
            TimeUnit::Megayear => {
                value * YEARS_PER_KILOYEAR * KILOYEARS_PER_MEGAYEAR * SECONDS_PER_YEAR
            }
            TimeUnit::Gigayear => {
                value
                    * YEARS_PER_KILOYEAR
                    * KILOYEARS_PER_MEGAYEAR
                    * MEGAYEARS_PER_GIGAYEAR
                    * SECONDS_PER_YEAR
            }
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
            TimeUnit::Megayear => {
                value * YEARS_PER_SECONDS * KILOYEARS_PER_YEAR * MEGAYEARS_PER_KILOYEAR
            }
            TimeUnit::Gigayear => {
                value
                    * YEARS_PER_SECONDS
                    * KILOYEARS_PER_YEAR
                    * MEGAYEARS_PER_KILOYEAR
                    * GIGAYEARS_PER_MEGAYEAR
            }
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
