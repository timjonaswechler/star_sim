use crate::physics::units::compression_modulus::{CompressionModulus, CompressionModulusUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: CompressionModulusUnit> Add for CompressionModulus<U> {
    type Output = CompressionModulus<U>;
    fn add(self, other: CompressionModulus<U>) -> CompressionModulus<U> {
        CompressionModulus::new(self.value + other.value)
    }
}

impl<U: CompressionModulusUnit> Sub for CompressionModulus<U> {
    type Output = CompressionModulus<U>;
    fn sub(self, other: CompressionModulus<U>) -> CompressionModulus<U> {
        CompressionModulus::new(self.value - other.value)
    }
}

impl<U: CompressionModulusUnit> Mul<f64> for CompressionModulus<U> {
    type Output = CompressionModulus<U>;
    fn mul(self, scalar: f64) -> CompressionModulus<U> {
        CompressionModulus::new(self.value * scalar)
    }
}

impl<U: CompressionModulusUnit> Div<f64> for CompressionModulus<U> {
    type Output = CompressionModulus<U>;
    fn div(self, scalar: f64) -> CompressionModulus<U> {
        CompressionModulus::new(self.value / scalar)
    }
}

impl<U: CompressionModulusUnit> Neg for CompressionModulus<U> {
    type Output = CompressionModulus<U>;
    fn neg(self) -> CompressionModulus<U> {
        CompressionModulus::new(-self.value)
    }
}
