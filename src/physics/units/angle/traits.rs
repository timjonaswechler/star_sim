use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait AngleUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Radian;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Degree;

impl AngleUnit for Radian {}
impl AngleUnit for Degree {}

impl UnitSymbol for Radian {
    fn symbol() -> String {
        "rad".into()
    }
}

impl UnitSymbol for Degree {
    fn symbol() -> String {
        "°".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Angle<U: AngleUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: AngleUnit> Angle<U> {
    pub fn new(value: f64) -> Self {
        Angle { value, _unit: PhantomData }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait AngleConvertTo<V: AngleUnit> {
    fn convert(self) -> Angle<V>;
}

impl<U: AngleUnit> Angle<U> {
    pub fn get<V: AngleUnit>(self) -> Angle<V>
    where
        Self: AngleConvertTo<V>,
    {
        self.convert()
    }
}
