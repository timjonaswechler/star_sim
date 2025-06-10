use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

// Marker trait
pub trait LengthUnit {}

// Length unit types
pub struct Meter;
pub struct Kilometer;
pub struct AstronomicalUnit;
pub struct EarthRadius;

impl LengthUnit for Meter {}
impl LengthUnit for Kilometer {}
impl LengthUnit for AstronomicalUnit {}
impl LengthUnit for EarthRadius {}

impl UnitSymbol for Meter {
    fn symbol() -> String {
        "m".into()
    }
}

impl UnitSymbol for Kilometer {
    fn symbol() -> String {
        "km".into()
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

// Quantity struct
#[derive(Debug, Clone, Copy)]
pub struct Distance<U: LengthUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

// Basic constructors and accessors
impl<U: LengthUnit> Distance<U> {
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
pub trait LengthConvertTo<V: LengthUnit> {
    fn convert(self) -> Distance<V>;
}

// Generic get method
impl<U: LengthUnit> Distance<U> {
    pub fn get<V: LengthUnit>(self) -> Distance<V>
    where
        Self: LengthConvertTo<V>,
    {
        self.convert()
    }
}
