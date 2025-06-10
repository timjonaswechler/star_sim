use crate::physics::units::force::{Dyne, Force, ForceConvertTo, ForceUnit, Newton};
use crate::physics::units::UnitSymbol;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

const NEWTONS_PER_DYNE: f64 = 1e-5;
const DYNES_PER_NEWTON: f64 = 1.0 / NEWTONS_PER_DYNE;

impl ForceConvertTo<Newton> for Force<Dyne> {
    fn convert(self) -> Force<Newton> {
        Force::<Newton>::new(self.value * NEWTONS_PER_DYNE)
    }
}

impl ForceConvertTo<Dyne> for Force<Newton> {
    fn convert(self) -> Force<Dyne> {
        Force::<Dyne>::new(self.value * DYNES_PER_NEWTON)
    }
}

impl<U: ForceUnit + UnitSymbol> fmt::Display for Force<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: ForceUnit> Add for Force<U> {
    type Output = Force<U>;
    fn add(self, other: Force<U>) -> Force<U> {
        Force::new(self.value + other.value)
    }
}

impl<U: ForceUnit> Sub for Force<U> {
    type Output = Force<U>;
    fn sub(self, other: Force<U>) -> Force<U> {
        Force::new(self.value - other.value)
    }
}

impl<U: ForceUnit> Mul<f64> for Force<U> {
    type Output = Force<U>;
    fn mul(self, scalar: f64) -> Force<U> {
        Force::new(self.value * scalar)
    }
}

impl<U: ForceUnit> Div<f64> for Force<U> {
    type Output = Force<U>;
    fn div(self, scalar: f64) -> Force<U> {
        Force::new(self.value / scalar)
    }
}
