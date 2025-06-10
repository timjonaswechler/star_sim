use crate::physics::units::temperature::{Celsius, Kelvin, Temperature, TemperatureConvertTo, TemperatureUnit};
use crate::physics::units::UnitSymbol;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

const KELVIN_OFFSET: f64 = 273.15;

impl TemperatureConvertTo<Kelvin> for Temperature<Celsius> {
    fn convert(self) -> Temperature<Kelvin> {
        Temperature::<Kelvin>::new(self.value + KELVIN_OFFSET)
    }
}

impl TemperatureConvertTo<Celsius> for Temperature<Kelvin> {
    fn convert(self) -> Temperature<Celsius> {
        Temperature::<Celsius>::new(self.value - KELVIN_OFFSET)
    }
}

impl<U: TemperatureUnit + UnitSymbol> fmt::Display for Temperature<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

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
