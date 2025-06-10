use crate::physics::units::power::{Power, PowerUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

// Basic math operations
impl<U: PowerUnit> Add for Power<U> {
    type Output = Power<U>;
    fn add(self, other: Power<U>) -> Power<U> {
        Power::new(self.value + other.value)
    }
}

impl<U: PowerUnit> Sub for Power<U> {
    type Output = Power<U>;
    fn sub(self, other: Power<U>) -> Power<U> {
        Power::new(self.value - other.value)
    }
}

impl<U: PowerUnit> Mul<f64> for Power<U> {
    type Output = Power<U>;
    fn mul(self, scalar: f64) -> Power<U> {
        Power::new(self.value * scalar)
    }
}

impl<U: PowerUnit> Div<f64> for Power<U> {
    type Output = Power<U>;
    fn div(self, scalar: f64) -> Power<U> {
        Power::new(self.value / scalar)
    }
}

impl<U: PowerUnit> Neg for Power<U> {
    type Output = Power<U>;
    fn neg(self) -> Power<U> {
        Power::new(-self.value)
    }
}
