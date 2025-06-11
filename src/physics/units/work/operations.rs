use crate::physics::units::work::{Work, WorkUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: WorkUnit> Add for Work<U> {
    type Output = Work<U>;
    fn add(self, other: Work<U>) -> Work<U> {
        Work::new(self.value + other.value)
    }
}

impl<U: WorkUnit> Sub for Work<U> {
    type Output = Work<U>;
    fn sub(self, other: Work<U>) -> Work<U> {
        Work::new(self.value - other.value)
    }
}

impl<U: WorkUnit> Mul<f64> for Work<U> {
    type Output = Work<U>;
    fn mul(self, scalar: f64) -> Work<U> {
        Work::new(self.value * scalar)
    }
}

impl<U: WorkUnit> Div<f64> for Work<U> {
    type Output = Work<U>;
    fn div(self, scalar: f64) -> Work<U> {
        Work::new(self.value / scalar)
    }
}

impl<U: WorkUnit> Neg for Work<U> {
    type Output = Work<U>;
    fn neg(self) -> Work<U> {
        Work::new(-self.value)
    }
}
