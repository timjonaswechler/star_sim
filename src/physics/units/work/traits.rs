use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait WorkUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Joule;

impl WorkUnit for Joule {}

impl UnitSymbol for Joule {
    fn symbol() -> String {
        "J".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Work<U: WorkUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}
impl<U: WorkUnit> Work<U> {
    pub fn new(value: f64) -> Self {
        Work {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait WorkConvertTo<V: WorkUnit> {
    fn convert(self) -> Work<V>;
}

impl<U: WorkUnit> Work<U> {
    pub fn get<V: WorkUnit>(self) -> Work<V>
    where
        Self: WorkConvertTo<V>,
    {
        self.convert()
    }
}
