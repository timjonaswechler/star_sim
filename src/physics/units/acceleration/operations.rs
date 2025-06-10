use crate::physics::units::acceleration::{Acceleration, AccelerationUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: AccelerationUnit> Add for Acceleration<U> {
    type Output = Acceleration<U>;
    fn add(self, other: Acceleration<U>) -> Acceleration<U> {
        Acceleration::new(self.value + other.value)
    }
}

impl<U: AccelerationUnit> Sub for Acceleration<U> {
    type Output = Acceleration<U>;
    fn sub(self, other: Acceleration<U>) -> Acceleration<U> {
        Acceleration::new(self.value - other.value)
    }
}

impl<U: AccelerationUnit> Mul<f64> for Acceleration<U> {
    type Output = Acceleration<U>;
    fn mul(self, scalar: f64) -> Acceleration<U> {
        Acceleration::new(self.value * scalar)
    }
}

impl<U: AccelerationUnit> Div<f64> for Acceleration<U> {
    type Output = Acceleration<U>;
    fn div(self, scalar: f64) -> Acceleration<U> {
        Acceleration::new(self.value / scalar)
    }
}

impl<U: AccelerationUnit> Neg for Acceleration<U> {
    type Output = Acceleration<U>;
    fn neg(self) -> Acceleration<U> {
        Acceleration::new(-self.value)
    }
}
