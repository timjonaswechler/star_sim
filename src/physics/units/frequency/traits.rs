use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait FrequencyUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Hertz;

impl FrequencyUnit for Hertz {}

impl UnitSymbol for Hertz {
    fn symbol() -> String {
        "Hz".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Frequency<U: FrequencyUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: FrequencyUnit> Frequency<U> {
    pub fn new(value: f64) -> Self {
        Frequency {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait FrequencyConvertTo<V: FrequencyUnit> {
    fn convert(self) -> Frequency<V>;
}

impl<U: FrequencyUnit> Frequency<U> {
    pub fn get<V: FrequencyUnit>(self) -> Frequency<V>
    where
        Self: FrequencyConvertTo<V>,
    {
        self.convert()
    }
}
