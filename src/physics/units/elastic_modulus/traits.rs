use crate::physics::units::UnitSymbol;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait ElasticModulusUnit {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Pascal;

impl ElasticModulusUnit for Pascal {}

impl UnitSymbol for Pascal {
    fn symbol() -> String {
        "Pa".into()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ElasticModulus<U: ElasticModulusUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: ElasticModulusUnit> ElasticModulus<U> {
    pub fn new(value: f64) -> Self {
        ElasticModulus { value, _unit: PhantomData }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait ElasticModulusConvertTo<V: ElasticModulusUnit> {
    fn convert(self) -> ElasticModulus<V>;
}

impl<U: ElasticModulusUnit> ElasticModulus<U> {
    pub fn get<V: ElasticModulusUnit>(self) -> ElasticModulus<V>
    where
        Self: ElasticModulusConvertTo<V>,
    {
        self.convert()
    }
}
