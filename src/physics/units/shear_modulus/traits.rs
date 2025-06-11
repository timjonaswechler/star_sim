use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait ShearModulusUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pascal;

impl ShearModulusUnit for Pascal {}

impl UnitSymbol for Pascal {
    fn symbol() -> String {
        "Pa".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ShearModulus<U: ShearModulusUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: ShearModulusUnit> ShearModulus<U> {
    pub fn new(value: f64) -> Self {
        ShearModulus { value, _unit: PhantomData }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait ShearModulusConvertTo<V: ShearModulusUnit> {
    fn convert(self) -> ShearModulus<V>;
}

impl<U: ShearModulusUnit> ShearModulus<U> {
    pub fn get<V: ShearModulusUnit>(self) -> ShearModulus<V>
    where
        Self: ShearModulusConvertTo<V>,
    {
        self.convert()
    }
}
