use crate::physics::units::angular_acceleration::{AngularAcceleration, AngularAccelerationUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: AngularAccelerationUnit> Add for AngularAcceleration<U> {
    type Output = AngularAcceleration<U>;
    fn add(self, other: AngularAcceleration<U>) -> AngularAcceleration<U> {
        AngularAcceleration::new(self.value + other.value)
    }
}

impl<U: AngularAccelerationUnit> Sub for AngularAcceleration<U> {
    type Output = AngularAcceleration<U>;
    fn sub(self, other: AngularAcceleration<U>) -> AngularAcceleration<U> {
        AngularAcceleration::new(self.value - other.value)
    }
}

impl<U: AngularAccelerationUnit> Mul<f64> for AngularAcceleration<U> {
    type Output = AngularAcceleration<U>;
    fn mul(self, scalar: f64) -> AngularAcceleration<U> {
        AngularAcceleration::new(self.value * scalar)
    }
}

impl<U: AngularAccelerationUnit> Div<f64> for AngularAcceleration<U> {
    type Output = AngularAcceleration<U>;
    fn div(self, scalar: f64) -> AngularAcceleration<U> {
        AngularAcceleration::new(self.value / scalar)
    }
}

impl<U: AngularAccelerationUnit> Neg for AngularAcceleration<U> {
    type Output = AngularAcceleration<U>;
    fn neg(self) -> AngularAcceleration<U> {
        AngularAcceleration::new(-self.value)
    }
}
