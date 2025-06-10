use crate::physics::units::length::{Distance, LengthUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

// Basic math operations
impl<U: LengthUnit> Add for Distance<U> {
    type Output = Distance<U>;
    fn add(self, other: Distance<U>) -> Distance<U> {
        Distance::new(self.value + other.value)
    }
}

impl<U: LengthUnit> Sub for Distance<U> {
    type Output = Distance<U>;
    fn sub(self, other: Distance<U>) -> Distance<U> {
        Distance::new(self.value - other.value)
    }
}

impl<U: LengthUnit> Mul<f64> for Distance<U> {
    type Output = Distance<U>;
    fn mul(self, scalar: f64) -> Distance<U> {
        Distance::new(self.value * scalar)
    }
}

impl<U: LengthUnit> Div<f64> for Distance<U> {
    type Output = Distance<U>;
    fn div(self, scalar: f64) -> Distance<U> {
        Distance::new(self.value / scalar)
    }
}

impl<U: LengthUnit> Neg for Distance<U> {
    type Output = Distance<U>;
    fn neg(self) -> Distance<U> {
        Distance::new(-self.value)
    }
}
