use super::Prefixed;
use crate::physics::units::{Kilo, UnitSymbol};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// Marker trait
pub trait DistanceUnit {}

// Distance unit types
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Meter;
pub type Kilometer = Prefixed<Kilo, Meter>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AstronomicalUnit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EarthRadius;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SunRadius;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LightYear;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Parsec;
pub type Kiloparsec = Prefixed<Kilo, Parsec>;

impl DistanceUnit for Meter {}
impl DistanceUnit for AstronomicalUnit {}
impl DistanceUnit for EarthRadius {}
impl DistanceUnit for SunRadius {}
impl DistanceUnit for LightYear {}
impl DistanceUnit for Parsec {}

impl UnitSymbol for Meter {
    fn symbol() -> String {
        "m".into()
    }
}

impl UnitSymbol for AstronomicalUnit {
    fn symbol() -> String {
        "AU".into()
    }
}

impl UnitSymbol for EarthRadius {
    fn symbol() -> String {
        "R⊕".into()
    }
}

impl UnitSymbol for SunRadius {
    fn symbol() -> String {
        "R☉".into()
    }
}
impl UnitSymbol for LightYear {
    fn symbol() -> String {
        "ly".into()
    }
}
impl UnitSymbol for Parsec {
    fn symbol() -> String {
        "pc".into()
    }
}

// Quantity struct
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Distance<U: DistanceUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

// Basic constructors and accessors
impl<U: DistanceUnit> Distance<U> {
    pub fn new(value: f64) -> Self {
        Distance {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

// Conversion trait
pub trait DistanceConvertTo<V: DistanceUnit> {
    fn convert(self) -> Distance<V>;
}

// Generic get method
impl<U: DistanceUnit> Distance<U> {
    pub fn get<V: DistanceUnit>(self) -> Distance<V>
    where
        Self: DistanceConvertTo<V>,
    {
        self.convert()
    }
}
