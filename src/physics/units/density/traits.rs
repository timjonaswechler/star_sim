use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait DensityUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct KilogramPerCubicMeter;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GramPerCubicCentimeter;

impl DensityUnit for KilogramPerCubicMeter {}
impl DensityUnit for GramPerCubicCentimeter {}

impl UnitSymbol for KilogramPerCubicMeter {
    fn symbol() -> String {
        "kg/m³".into()
    }
}

impl UnitSymbol for GramPerCubicCentimeter {
    fn symbol() -> String {
        "g/cm³".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Density<U: DensityUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: DensityUnit> Density<U> {
    pub fn new(value: f64) -> Self {
        Density {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait DensityConvertTo<V: DensityUnit> {
    fn convert(self) -> Density<V>;
}

impl<U: DensityUnit> Density<U> {
    pub fn get<V: DensityUnit>(self) -> Density<V>
    where
        Self: DensityConvertTo<V>,
    {
        self.convert()
    }
}
