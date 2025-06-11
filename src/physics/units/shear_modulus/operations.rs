use crate::physics::units::shear_modulus::{ShearModulus, ShearModulusUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: ShearModulusUnit> Add for ShearModulus<U> {
    type Output = ShearModulus<U>;
    fn add(self, other: ShearModulus<U>) -> ShearModulus<U> {
        ShearModulus::new(self.value + other.value)
    }
}

impl<U: ShearModulusUnit> Sub for ShearModulus<U> {
    type Output = ShearModulus<U>;
    fn sub(self, other: ShearModulus<U>) -> ShearModulus<U> {
        ShearModulus::new(self.value - other.value)
    }
}

impl<U: ShearModulusUnit> Mul<f64> for ShearModulus<U> {
    type Output = ShearModulus<U>;
    fn mul(self, scalar: f64) -> ShearModulus<U> {
        ShearModulus::new(self.value * scalar)
    }
}

impl<U: ShearModulusUnit> Div<f64> for ShearModulus<U> {
    type Output = ShearModulus<U>;
    fn div(self, scalar: f64) -> ShearModulus<U> {
        ShearModulus::new(self.value / scalar)
    }
}

impl<U: ShearModulusUnit> Neg for ShearModulus<U> {
    type Output = ShearModulus<U>;
    fn neg(self) -> ShearModulus<U> {
        ShearModulus::new(-self.value)
    }
}
