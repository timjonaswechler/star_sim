use crate::physics::units::UnitSymbol;
use crate::physics::units::pressure::{Bar, Pascal, Pressure, PressureConvertTo, PressureUnit};
use std::fmt;

const PASCALS_PER_BAR: f64 = 100_000.0;
const BARS_PER_PASCAL: f64 = 1.0 / PASCALS_PER_BAR;

impl PressureConvertTo<Pascal> for Pressure<Bar> {
    fn convert(self) -> Pressure<Pascal> {
        Pressure::<Pascal>::new(self.value * PASCALS_PER_BAR)
    }
}

impl PressureConvertTo<Bar> for Pressure<Pascal> {
    fn convert(self) -> Pressure<Bar> {
        Pressure::<Bar>::new(self.value * BARS_PER_PASCAL)
    }
}

impl<U: PressureUnit + UnitSymbol> fmt::Display for Pressure<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
