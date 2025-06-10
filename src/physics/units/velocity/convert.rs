// Velocity conversions
impl VelocityConvertTo<MeterPerSecond> for Velocity<KilometerPerHour> {
    fn convert(self) -> Velocity<MeterPerSecond> {
        Velocity::<MeterPerSecond>::new(self.value * METER_PER_SECONDS_IN_KILOMETER_PER_HOUR)
    }
}
impl VelocityConvertTo<KilometerPerHour> for Velocity<MeterPerSecond> {
    fn convert(self) -> Velocity<KilometerPerHour> {
        Velocity::<KilometerPerHour>::new(self.value / METER_PER_SECONDS_IN_KILOMETER_PER_HOUR)
    }
}

// Display implementations
impl<U: VelocityUnit + UnitSymbol> fmt::Display for Velocity<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: VelocityUnit> Add for Velocity<U> {
    type Output = Velocity<U>;
    fn add(self, other: Velocity<U>) -> Velocity<U> {
        Velocity::new(self.value + other.value)
    }
}

impl<U: VelocityUnit> Sub for Velocity<U> {
    type Output = Velocity<U>;
    fn sub(self, other: Velocity<U>) -> Velocity<U> {
        Velocity::new(self.value - other.value)
    }
}

impl<U: VelocityUnit> Mul<f64> for Velocity<U> {
    type Output = Velocity<U>;
    fn mul(self, scalar: f64) -> Velocity<U> {
        Velocity::new(self.value * scalar)
    }
}

impl<U: VelocityUnit> Div<f64> for Velocity<U> {
    type Output = Velocity<U>;
    fn div(self, scalar: f64) -> Velocity<U> {
        Velocity::new(self.value / scalar)
    }
}
