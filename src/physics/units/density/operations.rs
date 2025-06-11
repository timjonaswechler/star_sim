use crate::physics::units::density::{Density, DensityUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: DensityUnit> Add for Density<U> {
    type Output = Density<U>;
    fn add(self, other: Density<U>) -> Density<U> {
        Density::new(self.value + other.value)
    }
}

impl<U: DensityUnit> Sub for Density<U> {
    type Output = Density<U>;
    fn sub(self, other: Density<U>) -> Density<U> {
        Density::new(self.value - other.value)
    }
}

impl<U: DensityUnit> Mul<f64> for Density<U> {
    type Output = Density<U>;
    fn mul(self, scalar: f64) -> Density<U> {
        Density::new(self.value * scalar)
    }
}

impl<U: DensityUnit> Div<f64> for Density<U> {
    type Output = Density<U>;
    fn div(self, scalar: f64) -> Density<U> {
        Density::new(self.value / scalar)
    }
}

impl<U: DensityUnit> Neg for Density<U> {
    type Output = Density<U>;
    fn neg(self) -> Density<U> {
        Density::new(-self.value)
    }
}
