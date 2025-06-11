use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait CompressionModulusUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pascal;

impl CompressionModulusUnit for Pascal {}

impl UnitSymbol for Pascal {
    fn symbol() -> String {
        "Pa".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CompressionModulus<U: CompressionModulusUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: CompressionModulusUnit> CompressionModulus<U> {
    pub fn new(value: f64) -> Self {
        CompressionModulus { value, _unit: PhantomData }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait CompressionModulusConvertTo<V: CompressionModulusUnit> {
    fn convert(self) -> CompressionModulus<V>;
}

impl<U: CompressionModulusUnit> CompressionModulus<U> {
    pub fn get<V: CompressionModulusUnit>(self) -> CompressionModulus<V>
    where
        Self: CompressionModulusConvertTo<V>,
    {
        self.convert()
    }
}
