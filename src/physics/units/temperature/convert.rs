use crate::physics::units::UnitSymbol;
use crate::physics::units::temperature::{
    Celsius, Kelvin, Temperature, TemperatureConvertTo, TemperatureUnit,
};
use std::fmt;

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
