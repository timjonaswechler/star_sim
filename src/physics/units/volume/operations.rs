use crate::physics::units::volume::{Volume, VolumeUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: VolumeUnit> Add for Volume<U> {
    type Output = Volume<U>;
    fn add(self, other: Volume<U>) -> Volume<U> {
        Volume::new(self.value + other.value)
    }
}

impl<U: VolumeUnit> Sub for Volume<U> {
    type Output = Volume<U>;
    fn sub(self, other: Volume<U>) -> Volume<U> {
        Volume::new(self.value - other.value)
    }
}

impl<U: VolumeUnit> Mul<f64> for Volume<U> {
    type Output = Volume<U>;
    fn mul(self, scalar: f64) -> Volume<U> {
        Volume::new(self.value * scalar)
    }
}

impl<U: VolumeUnit> Div<f64> for Volume<U> {
    type Output = Volume<U>;
    fn div(self, scalar: f64) -> Volume<U> {
        Volume::new(self.value / scalar)
    }
}

impl<U: VolumeUnit> Neg for Volume<U> {
    type Output = Volume<U>;
    fn neg(self) -> Volume<U> {
        Volume::new(-self.value)
    }
}
