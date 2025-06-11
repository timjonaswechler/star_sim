use crate::physics::units::angular_velocity::{AngularVelocity, AngularVelocityUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: AngularVelocityUnit> Add for AngularVelocity<U> {
    type Output = AngularVelocity<U>;
    fn add(self, other: AngularVelocity<U>) -> AngularVelocity<U> {
        AngularVelocity::new(self.value + other.value)
    }
}

impl<U: AngularVelocityUnit> Sub for AngularVelocity<U> {
    type Output = AngularVelocity<U>;
    fn sub(self, other: AngularVelocity<U>) -> AngularVelocity<U> {
        AngularVelocity::new(self.value - other.value)
    }
}

impl<U: AngularVelocityUnit> Mul<f64> for AngularVelocity<U> {
    type Output = AngularVelocity<U>;
    fn mul(self, scalar: f64) -> AngularVelocity<U> {
        AngularVelocity::new(self.value * scalar)
    }
}

impl<U: AngularVelocityUnit> Div<f64> for AngularVelocity<U> {
    type Output = AngularVelocity<U>;
    fn div(self, scalar: f64) -> AngularVelocity<U> {
        AngularVelocity::new(self.value / scalar)
    }
}

impl<U: AngularVelocityUnit> Neg for AngularVelocity<U> {
    type Output = AngularVelocity<U>;
    fn neg(self) -> AngularVelocity<U> {
        AngularVelocity::new(-self.value)
    }
}
