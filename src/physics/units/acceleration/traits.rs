use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

pub trait AccelerationUnit {}

pub struct MeterPerSecondSquared;
pub struct StandardGravity;

impl AccelerationUnit for MeterPerSecondSquared {}
impl AccelerationUnit for StandardGravity {}

impl UnitSymbol for MeterPerSecondSquared {
    fn symbol() -> String {
        "m/s²".into()
    }
}

impl UnitSymbol for StandardGravity {
    fn symbol() -> String {
        "g".into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Acceleration<U: AccelerationUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: AccelerationUnit> Acceleration<U> {
    pub fn new(value: f64) -> Self {
        Acceleration {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait AccelerationConvertTo<V: AccelerationUnit> {
    fn convert(self) -> Acceleration<V>;
}

impl<U: AccelerationUnit> Acceleration<U> {
    pub fn get<V: AccelerationUnit>(self) -> Acceleration<V>
    where
        Self: AccelerationConvertTo<V>,
    {
        self.convert()
    }
}
