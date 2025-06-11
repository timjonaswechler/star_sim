use super::Prefixed;
use crate::physics::units::{Kilo, UnitSymbol};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// Marker trait
pub trait MassUnit {}

// Mass unit types
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Gram;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct EarthMass;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct SolarMass;

pub type Kilogram = Prefixed<Kilo, Gram>;
impl MassUnit for Gram {}
impl MassUnit for EarthMass {}
impl MassUnit for SolarMass {}

impl UnitSymbol for Gram {
    fn symbol() -> String {
        "g".into()
    }
}

impl UnitSymbol for EarthMass {
    fn symbol() -> String {
        "M⊕".into()
    }
}

impl UnitSymbol for SolarMass {
    fn symbol() -> String {
        "M☉".into()
    }
}

// Quantity struct
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Mass<U: MassUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

// Basic constructors and accessors
impl<U: MassUnit> Mass<U> {
    pub fn new(value: f64) -> Self {
        Mass {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

// Conversion trait
pub trait MassConvertTo<V: MassUnit> {
    fn convert(self) -> Mass<V>;
}

// Generic get method
impl<U: MassUnit> Mass<U> {
    pub fn get<V: MassUnit>(self) -> Mass<V>
    where
        Self: MassConvertTo<V>,
    {
        self.convert()
    }
}
