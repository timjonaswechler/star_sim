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

impl<U: TimeUnit> Add for Time<U> {
    type Output = Time<U>;
    fn add(self, other: Time<U>) -> Time<U> {
        Time::new(self.value + other.value)
    }
}

impl<U: TimeUnit> Sub for Time<U> {
    type Output = Time<U>;
    fn sub(self, other: Time<U>) -> Time<U> {
        Time::new(self.value - other.value)
    }
}

impl<U: TimeUnit> Mul<f64> for Time<U> {
    type Output = Time<U>;
    fn mul(self, scalar: f64) -> Time<U> {
        Time::new(self.value * scalar)
    }
}

impl<U: TimeUnit> Div<f64> for Time<U> {
    type Output = Time<U>;
    fn div(self, scalar: f64) -> Time<U> {
        Time::new(self.value / scalar)
    }
}
