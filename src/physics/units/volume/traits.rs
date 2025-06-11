use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait VolumeUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct CubicMeter;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Liter;

impl VolumeUnit for CubicMeter {}
impl VolumeUnit for Liter {}

impl UnitSymbol for CubicMeter {
    fn symbol() -> String {
        "m³".into()
    }
}

impl UnitSymbol for Liter {
    fn symbol() -> String {
        "L".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Volume<U: VolumeUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: VolumeUnit> Volume<U> {
    pub fn new(value: f64) -> Self {
        Volume {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait VolumeConvertTo<V: VolumeUnit> {
    fn convert(self) -> Volume<V>;
}

impl<U: VolumeUnit> Volume<U> {
    pub fn get<V: VolumeUnit>(self) -> Volume<V>
    where
        Self: VolumeConvertTo<V>,
    {
        self.convert()
    }
}
