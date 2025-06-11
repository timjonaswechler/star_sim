use crate::physics::units::frequency::{Frequency, FrequencyUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: FrequencyUnit> Add for Frequency<U> {
    type Output = Frequency<U>;
    fn add(self, other: Frequency<U>) -> Frequency<U> {
        Frequency::new(self.value + other.value)
    }
}

impl<U: FrequencyUnit> Sub for Frequency<U> {
    type Output = Frequency<U>;
    fn sub(self, other: Frequency<U>) -> Frequency<U> {
        Frequency::new(self.value - other.value)
    }
}

impl<U: FrequencyUnit> Mul<f64> for Frequency<U> {
    type Output = Frequency<U>;
    fn mul(self, scalar: f64) -> Frequency<U> {
        Frequency::new(self.value * scalar)
    }
}

impl<U: FrequencyUnit> Div<f64> for Frequency<U> {
    type Output = Frequency<U>;
    fn div(self, scalar: f64) -> Frequency<U> {
        Frequency::new(self.value / scalar)
    }
}

impl<U: FrequencyUnit> Neg for Frequency<U> {
    type Output = Frequency<U>;
    fn neg(self) -> Frequency<U> {
        Frequency::new(-self.value)
    }
}
