use crate::physics::units::pressure::{Bar, Pascal, Pressure, PressureConvertTo, PressureUnit};
use crate::physics::units::UnitSymbol;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

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

impl<U: PressureUnit> Add for Pressure<U> {
    type Output = Pressure<U>;
    fn add(self, other: Pressure<U>) -> Pressure<U> {
        Pressure::new(self.value + other.value)
    }
}

impl<U: PressureUnit> Sub for Pressure<U> {
    type Output = Pressure<U>;
    fn sub(self, other: Pressure<U>) -> Pressure<U> {
        Pressure::new(self.value - other.value)
    }
}

impl<U: PressureUnit> Mul<f64> for Pressure<U> {
    type Output = Pressure<U>;
    fn mul(self, scalar: f64) -> Pressure<U> {
        Pressure::new(self.value * scalar)
    }
}

impl<U: PressureUnit> Div<f64> for Pressure<U> {
    type Output = Pressure<U>;
    fn div(self, scalar: f64) -> Pressure<U> {
        Pressure::new(self.value / scalar)
    }
}
