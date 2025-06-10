use super::{KilometerPerHour, MeterPerSecond, Velocity, VelocityConvertTo, VelocityUnit};
use crate::physics::units::UnitSymbol;
use std::fmt;

const METER_PER_SECONDS_IN_KILOMETER_PER_HOUR: f64 = 1000.0 / 3600.0;

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
