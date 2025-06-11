use crate::physics::units::mass::{Mass, MassUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

// Basic math operations
impl<U: MassUnit> Add for Mass<U> {
    type Output = Mass<U>;
    fn add(self, other: Mass<U>) -> Mass<U> {
        Mass::new(self.value + other.value)
    }
}

impl<U: MassUnit> Sub for Mass<U> {
    type Output = Mass<U>;
    fn sub(self, other: Mass<U>) -> Mass<U> {
        Mass::new(self.value - other.value)
    }
}

impl<U: MassUnit> Mul<f64> for Mass<U> {
    type Output = Mass<U>;
    fn mul(self, scalar: f64) -> Mass<U> {
        Mass::new(self.value * scalar)
    }
}

impl<U: MassUnit> Div<f64> for Mass<U> {
    type Output = Mass<U>;
    fn div(self, scalar: f64) -> Mass<U> {
        Mass::new(self.value / scalar)
    }
}

impl<U: MassUnit> Neg for Mass<U> {
    type Output = Mass<U>;
    fn neg(self) -> Mass<U> {
        Mass::new(-self.value)
    }
}
