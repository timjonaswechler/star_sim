use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

// Marker traits
pub trait VelocityUnit {}

// Velocity unit types
pub struct MeterPerSecond;
pub struct KilometerPerHour;

impl VelocityUnit for MeterPerSecond {}
impl VelocityUnit for KilometerPerHour {}

impl UnitSymbol for MeterPerSecond {
    fn symbol() -> String {
        "m/s".into()
    }
}
impl UnitSymbol for KilometerPerHour {
    fn symbol() -> String {
        "km/h".into()
    }
}

// Quantity structs
#[derive(Debug, Clone, Copy)]
pub struct Velocity<U: VelocityUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

// Basic constructors and accessors
impl<U: VelocityUnit> Velocity<U> {
    pub fn new(value: f64) -> Self {
        Velocity {
            value,
            _unit: PhantomData,
        }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
}

// Conversion traits
pub trait VelocityConvertTo<V: VelocityUnit> {
    fn convert(self) -> Velocity<V>;
}

// Generic get methods
impl<U: VelocityUnit> Velocity<U> {
    pub fn get<V: VelocityUnit>(self) -> Velocity<V>
    where
        Self: VelocityConvertTo<V>,
    {
        self.convert()
    }
}
