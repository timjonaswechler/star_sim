use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

pub trait ForceUnit {}

pub struct Newton;

impl ForceUnit for Newton {}

impl UnitSymbol for Newton {
    fn symbol() -> String {
        "N".into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Force<U: ForceUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: ForceUnit> Force<U> {
    pub fn new(value: f64) -> Self {
        Force {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait ForceConvertTo<V: ForceUnit> {
    fn convert(self) -> Force<V>;
}

impl<U: ForceUnit> Force<U> {
    pub fn get<V: ForceUnit>(self) -> Force<V>
    where
        Self: ForceConvertTo<V>,
    {
        self.convert()
    }
}
