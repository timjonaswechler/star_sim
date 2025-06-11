use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait TemperatureUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Kelvin;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Celsius;

impl TemperatureUnit for Kelvin {}
impl TemperatureUnit for Celsius {}

impl UnitSymbol for Kelvin {
    fn symbol() -> String {
        "K".into()
    }
}

impl UnitSymbol for Celsius {
    fn symbol() -> String {
        "°C".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Temperature<U: TemperatureUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: TemperatureUnit> Temperature<U> {
    pub fn new(value: f64) -> Self {
        Temperature {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait TemperatureConvertTo<V: TemperatureUnit> {
    fn convert(self) -> Temperature<V>;
}

impl<U: TemperatureUnit> Temperature<U> {
    pub fn get<V: TemperatureUnit>(self) -> Temperature<V>
    where
        Self: TemperatureConvertTo<V>,
    {
        self.convert()
    }
}
