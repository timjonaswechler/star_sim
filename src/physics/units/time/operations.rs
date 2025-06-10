use std::ops::{Add, Div, Mul, Neg, Sub};
// Basic math operations
impl<U: TimeUnit> Add for Time<U> {
    type Output = Time<U>;
    fn add(self, other: Time<U>) -> Time<U> {
        Time::new(self.value + other.value)
    }
}
impl<U: TimeUnit> Sub for Time<U> {
    type Output = Time<U>;
    fn sub(self, other: Time<U>) -> Time<U> {
        Time::new(self.value - other.value)
    }
}
impl<U: TimeUnit> Mul<f64> for Time<U> {
    type Output = Time<U>;
    fn mul(self, scalar: f64) -> Time<U> {
        Time::new(self.value * scalar)
    }
}
impl<U: TimeUnit> Div<f64> for Time<U> {
    type Output = Time<U>;
    fn div(self, scalar: f64) -> Time<U> {
        Time::new(self.value / scalar)
    }
}
impl<U: TimeUnit> Neg for Time<U> {
    type Output = Time<U>;
    fn neg(self) -> Time<U> {
        Time::new(-self.value)
    }
}
