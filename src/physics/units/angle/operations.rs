use crate::physics::units::angle::{Angle, AngleUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: AngleUnit> Add for Angle<U> {
    type Output = Angle<U>;
    fn add(self, other: Angle<U>) -> Angle<U> {
        Angle::new(self.value + other.value)
    }
}

impl<U: AngleUnit> Sub for Angle<U> {
    type Output = Angle<U>;
    fn sub(self, other: Angle<U>) -> Angle<U> {
        Angle::new(self.value - other.value)
    }
}

impl<U: AngleUnit> Mul<f64> for Angle<U> {
    type Output = Angle<U>;
    fn mul(self, scalar: f64) -> Angle<U> {
        Angle::new(self.value * scalar)
    }
}

impl<U: AngleUnit> Div<f64> for Angle<U> {
    type Output = Angle<U>;
    fn div(self, scalar: f64) -> Angle<U> {
        Angle::new(self.value / scalar)
    }
}

impl<U: AngleUnit> Neg for Angle<U> {
    type Output = Angle<U>;
    fn neg(self) -> Angle<U> {
        Angle::new(-self.value)
    }
}
