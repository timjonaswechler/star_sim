use crate::physics::units::area::{Area, AreaUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: AreaUnit> Add for Area<U> {
    type Output = Area<U>;
    fn add(self, other: Area<U>) -> Area<U> {
        Area::new(self.value + other.value)
    }
}

impl<U: AreaUnit> Sub for Area<U> {
    type Output = Area<U>;
    fn sub(self, other: Area<U>) -> Area<U> {
        Area::new(self.value - other.value)
    }
}

impl<U: AreaUnit> Mul<f64> for Area<U> {
    type Output = Area<U>;
    fn mul(self, scalar: f64) -> Area<U> {
        Area::new(self.value * scalar)
    }
}

impl<U: AreaUnit> Div<f64> for Area<U> {
    type Output = Area<U>;
    fn div(self, scalar: f64) -> Area<U> {
        Area::new(self.value / scalar)
    }
}

impl<U: AreaUnit> Neg for Area<U> {
    type Output = Area<U>;
    fn neg(self) -> Area<U> {
        Area::new(-self.value)
    }
}
