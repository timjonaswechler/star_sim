use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait PressureUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Pascal;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Bar;

impl PressureUnit for Pascal {}
impl PressureUnit for Bar {}

impl UnitSymbol for Pascal {
    fn symbol() -> String {
        "Pa".into()
    }
}

impl UnitSymbol for Bar {
    fn symbol() -> String {
        "bar".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Pressure<U: PressureUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: PressureUnit> Pressure<U> {
    pub fn new(value: f64) -> Self {
        Pressure {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait PressureConvertTo<V: PressureUnit> {
    fn convert(self) -> Pressure<V>;
}

impl<U: PressureUnit> Pressure<U> {
    pub fn get<V: PressureUnit>(self) -> Pressure<V>
    where
        Self: PressureConvertTo<V>,
    {
        self.convert()
    }
}
