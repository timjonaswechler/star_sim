use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait AngularVelocityUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct RadianPerSecond;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct DegreePerSecond;

impl AngularVelocityUnit for RadianPerSecond {}
impl AngularVelocityUnit for DegreePerSecond {}

impl UnitSymbol for RadianPerSecond {
    fn symbol() -> String {
        "rad/s".into()
    }
}

impl UnitSymbol for DegreePerSecond {
    fn symbol() -> String {
        "°/s".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct AngularVelocity<U: AngularVelocityUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: AngularVelocityUnit> AngularVelocity<U> {
    pub fn new(value: f64) -> Self {
        AngularVelocity { value, _unit: PhantomData }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait AngularVelocityConvertTo<V: AngularVelocityUnit> {
    fn convert(self) -> AngularVelocity<V>;
}

impl<U: AngularVelocityUnit> AngularVelocity<U> {
    pub fn get<V: AngularVelocityUnit>(self) -> AngularVelocity<V>
    where
        Self: AngularVelocityConvertTo<V>,
    {
        self.convert()
    }
}
