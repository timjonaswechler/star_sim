use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait AreaUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct SquareMeter;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct SquareKilometer;

impl AreaUnit for SquareMeter {}
impl AreaUnit for SquareKilometer {}

impl UnitSymbol for SquareMeter {
    fn symbol() -> String {
        "m²".into()
    }
}

impl UnitSymbol for SquareKilometer {
    fn symbol() -> String {
        "km²".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Area<U: AreaUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: AreaUnit> Area<U> {
    pub fn new(value: f64) -> Self {
        Area {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait AreaConvertTo<V: AreaUnit> {
    fn convert(self) -> Area<V>;
}

impl<U: AreaUnit> Area<U> {
    pub fn get<V: AreaUnit>(self) -> Area<V>
    where
        Self: AreaConvertTo<V>,
    {
        self.convert()
    }
}
