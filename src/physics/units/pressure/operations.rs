use crate::physics::units::pressure::{Pressure, PressureUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl<U: PressureUnit> Neg for Pressure<U> {
    type Output = Pressure<U>;
    fn neg(self) -> Pressure<U> {
        Pressure::new(-self.value)
    }
}
