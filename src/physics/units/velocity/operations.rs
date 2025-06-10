use crate::physics::units::velocity::{Velocity, VelocityUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};
// Basic math operations
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
impl<U: VelocityUnit> Neg for Velocity<U> {
    type Output = Velocity<U>;
    fn neg(self) -> Velocity<U> {
        Velocity::new(-self.value)
    }
}
