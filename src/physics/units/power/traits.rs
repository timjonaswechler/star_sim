use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

pub trait PowerUnit {}

pub struct Watt;
pub struct SolarLuminosity;

impl PowerUnit for Watt {}
impl PowerUnit for SolarLuminosity {}

impl UnitSymbol for Watt {
    fn symbol() -> String {
        "W".into()
    }
}

impl UnitSymbol for SolarLuminosity {
    fn symbol() -> String {
        "L☉".into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Power<U: PowerUnit> {
    pub value: f64,
    _unit: PhantomData<U>,
}

impl<U: PowerUnit> Power<U> {
    pub fn new(value: f64) -> Self {
        Power {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub trait PowerConvertTo<V: PowerUnit> {
    fn convert(self) -> Power<V>;
}

impl<U: PowerUnit> Power<U> {
    pub fn get<V: PowerUnit>(self) -> Power<V>
    where
        Self: PowerConvertTo<V>,
    {
        self.convert()
    }
}
