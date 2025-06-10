use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait EnergyUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Joule;

impl EnergyUnit for Joule {}

impl UnitSymbol for Joule {
    fn symbol() -> String {
        "J".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Energy<U: EnergyUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: EnergyUnit> Energy<U> {
    pub fn new(value: f64) -> Self {
        Energy {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait EnergyConvertTo<V: EnergyUnit> {
    fn convert(self) -> Energy<V>;
}

impl<U: EnergyUnit> Energy<U> {
    pub fn get<V: EnergyUnit>(self) -> Energy<V>
    where
        Self: EnergyConvertTo<V>,
    {
        self.convert()
    }
}
