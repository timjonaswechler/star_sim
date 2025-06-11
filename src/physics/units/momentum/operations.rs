use crate::physics::units::momentum::{Momentum, MomentumUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: MomentumUnit> Add for Momentum<U> {
    type Output = Momentum<U>;
    fn add(self, other: Momentum<U>) -> Momentum<U> {
        Momentum::new(self.value + other.value)
    }
}

impl<U: MomentumUnit> Sub for Momentum<U> {
    type Output = Momentum<U>;
    fn sub(self, other: Momentum<U>) -> Momentum<U> {
        Momentum::new(self.value - other.value)
    }
}

impl<U: MomentumUnit> Mul<f64> for Momentum<U> {
    type Output = Momentum<U>;
    fn mul(self, scalar: f64) -> Momentum<U> {
        Momentum::new(self.value * scalar)
    }
}

impl<U: MomentumUnit> Div<f64> for Momentum<U> {
    type Output = Momentum<U>;
    fn div(self, scalar: f64) -> Momentum<U> {
        Momentum::new(self.value / scalar)
    }
}

impl<U: MomentumUnit> Neg for Momentum<U> {
    type Output = Momentum<U>;
    fn neg(self) -> Momentum<U> {
        Momentum::new(-self.value)
    }
}
