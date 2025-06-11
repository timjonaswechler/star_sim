use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait AngularAccelerationUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct RadianPerSecondSquared;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DegreePerSecondSquared;

impl AngularAccelerationUnit for RadianPerSecondSquared {}
impl AngularAccelerationUnit for DegreePerSecondSquared {}

impl UnitSymbol for RadianPerSecondSquared {
    fn symbol() -> String {
        "rad/s²".into()
    }
}

impl UnitSymbol for DegreePerSecondSquared {
    fn symbol() -> String {
        "°/s²".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AngularAcceleration<U: AngularAccelerationUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: AngularAccelerationUnit> AngularAcceleration<U> {
    pub fn new(value: f64) -> Self {
        AngularAcceleration { value, _unit: PhantomData }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait AngularAccelerationConvertTo<V: AngularAccelerationUnit> {
    fn convert(self) -> AngularAcceleration<V>;
}

impl<U: AngularAccelerationUnit> AngularAcceleration<U> {
    pub fn get<V: AngularAccelerationUnit>(self) -> AngularAcceleration<V>
    where
        Self: AngularAccelerationConvertTo<V>,
    {
        self.convert()
    }
}
