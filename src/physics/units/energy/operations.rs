use crate::physics::units::energy::{Energy, EnergyUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: EnergyUnit> Add for Energy<U> {
    type Output = Energy<U>;
    fn add(self, other: Energy<U>) -> Energy<U> {
        Energy::new(self.value + other.value)
    }
}

impl<U: EnergyUnit> Sub for Energy<U> {
    type Output = Energy<U>;
    fn sub(self, other: Energy<U>) -> Energy<U> {
        Energy::new(self.value - other.value)
    }
}

impl<U: EnergyUnit> Mul<f64> for Energy<U> {
    type Output = Energy<U>;
    fn mul(self, scalar: f64) -> Energy<U> {
        Energy::new(self.value * scalar)
    }
}

impl<U: EnergyUnit> Div<f64> for Energy<U> {
    type Output = Energy<U>;
    fn div(self, scalar: f64) -> Energy<U> {
        Energy::new(self.value / scalar)
    }
}

impl<U: EnergyUnit> Neg for Energy<U> {
    type Output = Energy<U>;
    fn neg(self) -> Energy<U> {
        Energy::new(-self.value)
    }
}
