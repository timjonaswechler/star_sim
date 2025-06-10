use super::{Day, Hour, Minute, Second, Time, TimeConvertTo, TimeUnit, Year};
use crate::physics::units::UnitSymbol;
use std::fmt;

const SECONDS_PER_MINUTE: f64 = 60.0;
const MINUTES_PER_HOUR: f64 = 60.0;
const HOURS_PER_DAY: f64 = 24.0;
const DAYS_PER_YEAR: f64 = 365.25;

// Time conversions
impl TimeConvertTo<Second> for Time<Minute> {
    fn convert(self) -> Time<Second> {
        Time::<Second>::new(self.value * SECONDS_PER_MINUTE)
    }
}

impl TimeConvertTo<Minute> for Time<Second> {
    fn convert(self) -> Time<Minute> {
        Time::<Minute>::new(self.value / SECONDS_PER_MINUTE)
    }
}

impl TimeConvertTo<Second> for Time<Hour> {
    fn convert(self) -> Time<Second> {
        Time::<Second>::new(self.value * SECONDS_PER_MINUTE * MINUTES_PER_HOUR)
    }
}

impl TimeConvertTo<Hour> for Time<Second> {
    fn convert(self) -> Time<Hour> {
        Time::<Hour>::new(self.value / (SECONDS_PER_MINUTE * MINUTES_PER_HOUR))
    }
}
impl TimeConvertTo<Second> for Time<Day> {
    fn convert(self) -> Time<Second> {
        Time::<Second>::new(self.value * HOURS_PER_DAY * SECONDS_PER_MINUTE * MINUTES_PER_HOUR)
    }
}
impl TimeConvertTo<Day> for Time<Second> {
    fn convert(self) -> Time<Day> {
        Time::<Day>::new(self.value / (HOURS_PER_DAY * SECONDS_PER_MINUTE * MINUTES_PER_HOUR))
    }
}
impl TimeConvertTo<Second> for Time<Year> {
    fn convert(self) -> Time<Second> {
        Time::<Second>::new(
            self.value * DAYS_PER_YEAR * HOURS_PER_DAY * SECONDS_PER_MINUTE * MINUTES_PER_HOUR,
        )
    }
}
impl TimeConvertTo<Year> for Time<Second> {
    fn convert(self) -> Time<Year> {
        Time::<Year>::new(
            self.value / (DAYS_PER_YEAR * HOURS_PER_DAY * SECONDS_PER_MINUTE * MINUTES_PER_HOUR),
        )
    }
}
impl TimeConvertTo<Minute> for Time<Hour> {
    fn convert(self) -> Time<Minute> {
        Time::<Minute>::new(self.value * MINUTES_PER_HOUR)
    }
}
impl TimeConvertTo<Hour> for Time<Minute> {
    fn convert(self) -> Time<Hour> {
        Time::<Hour>::new(self.value / MINUTES_PER_HOUR)
    }
}
impl TimeConvertTo<Minute> for Time<Day> {
    fn convert(self) -> Time<Minute> {
        Time::<Minute>::new(self.value * HOURS_PER_DAY * MINUTES_PER_HOUR)
    }
}
impl TimeConvertTo<Day> for Time<Minute> {
    fn convert(self) -> Time<Day> {
        Time::<Day>::new(self.value / (HOURS_PER_DAY * MINUTES_PER_HOUR))
    }
}
impl TimeConvertTo<Minute> for Time<Year> {
    fn convert(self) -> Time<Minute> {
        Time::<Minute>::new(self.value * DAYS_PER_YEAR * HOURS_PER_DAY * MINUTES_PER_HOUR)
    }
}
impl TimeConvertTo<Year> for Time<Minute> {
    fn convert(self) -> Time<Year> {
        Time::<Year>::new(self.value / (DAYS_PER_YEAR * HOURS_PER_DAY * MINUTES_PER_HOUR))
    }
}
impl TimeConvertTo<Hour> for Time<Day> {
    fn convert(self) -> Time<Hour> {
        Time::<Hour>::new(self.value * HOURS_PER_DAY)
    }
}
impl TimeConvertTo<Day> for Time<Hour> {
    fn convert(self) -> Time<Day> {
        Time::<Day>::new(self.value / HOURS_PER_DAY)
    }
}
impl TimeConvertTo<Hour> for Time<Year> {
    fn convert(self) -> Time<Hour> {
        Time::<Hour>::new(self.value * DAYS_PER_YEAR * HOURS_PER_DAY)
    }
}
impl TimeConvertTo<Year> for Time<Hour> {
    fn convert(self) -> Time<Year> {
        Time::<Year>::new(self.value / (DAYS_PER_YEAR * HOURS_PER_DAY))
    }
}

impl TimeConvertTo<Day> for Time<Year> {
    fn convert(self) -> Time<Day> {
        Time::<Day>::new(self.value * DAYS_PER_YEAR)
    }
}
impl TimeConvertTo<Year> for Time<Day> {
    fn convert(self) -> Time<Year> {
        Time::<Year>::new(self.value / DAYS_PER_YEAR)
    }
}

// Display implementations
impl<U: TimeUnit + UnitSymbol> fmt::Display for Time<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
