use crate::physics::units::force::{Force, ForceUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: ForceUnit> Add for Force<U> {
    type Output = Force<U>;
    fn add(self, other: Force<U>) -> Force<U> {
        Force::new(self.value + other.value)
    }
}

impl<U: ForceUnit> Sub for Force<U> {
    type Output = Force<U>;
    fn sub(self, other: Force<U>) -> Force<U> {
        Force::new(self.value - other.value)
    }
}

impl<U: ForceUnit> Mul<f64> for Force<U> {
    type Output = Force<U>;
    fn mul(self, scalar: f64) -> Force<U> {
        Force::new(self.value * scalar)
    }
}

impl<U: ForceUnit> Div<f64> for Force<U> {
    type Output = Force<U>;
    fn div(self, scalar: f64) -> Force<U> {
        Force::new(self.value / scalar)
    }
}

impl<U: ForceUnit> Neg for Force<U> {
    type Output = Force<U>;
    fn neg(self) -> Force<U> {
        Force::new(-self.value)
    }
}
