use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait MomentumUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct KilogramMeterPerSecond;

impl MomentumUnit for KilogramMeterPerSecond {}

impl UnitSymbol for KilogramMeterPerSecond {
    fn symbol() -> String {
        "kg·m/s".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Momentum<U: MomentumUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: MomentumUnit> Momentum<U> {
    pub fn new(value: f64) -> Self {
        Momentum {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait MomentumConvertTo<V: MomentumUnit> {
    fn convert(self) -> Momentum<V>;
}

impl<U: MomentumUnit> Momentum<U> {
    pub fn get<V: MomentumUnit>(self) -> Momentum<V>
    where
        Self: MomentumConvertTo<V>,
    {
        self.convert()
    }
}
