use crate::physics::units::temperature::{Temperature, TemperatureUnit};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<U: TemperatureUnit> Add for Temperature<U> {
    type Output = Temperature<U>;
    fn add(self, other: Temperature<U>) -> Temperature<U> {
        Temperature::new(self.value + other.value)
    }
}

impl<U: TemperatureUnit> Sub for Temperature<U> {
    type Output = Temperature<U>;
    fn sub(self, other: Temperature<U>) -> Temperature<U> {
        Temperature::new(self.value - other.value)
    }
}

impl<U: TemperatureUnit> Mul<f64> for Temperature<U> {
    type Output = Temperature<U>;
    fn mul(self, scalar: f64) -> Temperature<U> {
        Temperature::new(self.value * scalar)
    }
}

impl<U: TemperatureUnit> Div<f64> for Temperature<U> {
    type Output = Temperature<U>;
    fn div(self, scalar: f64) -> Temperature<U> {
        Temperature::new(self.value / scalar)
    }
}

impl<U: TemperatureUnit> Neg for Temperature<U> {
    type Output = Temperature<U>;
    fn neg(self) -> Temperature<U> {
        Temperature::new(-self.value)
    }
}
