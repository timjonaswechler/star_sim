use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

// Marker trait
pub trait MassUnit {}

// Mass unit types
pub struct Kilogram;
pub struct EarthMass;
pub struct SolarMass;

impl MassUnit for Kilogram {}
impl MassUnit for EarthMass {}
impl MassUnit for SolarMass {}

impl UnitSymbol for Kilogram {
    fn symbol() -> String {
        "kg".into()
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
#[derive(Debug, Clone, Copy)]
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
