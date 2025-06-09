use crate::physics::unit_system::traits::Unit;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Generic quantity holding a value in the base unit of `U`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Quantity<U: Unit> {
    value_si: f64,
    _unit: PhantomData<U>,
}

impl<U: Unit> Quantity<U> {
    /// Construct a quantity from a value and a unit.
    pub fn new(value: f64, unit: U) -> Self {
        Self {
            value_si: unit.to_base(value),
            _unit: PhantomData,
        }
    }

    /// Retrieve the internal value stored in the SI base unit.
    pub fn in_base_units(&self) -> f64 {
        self.value_si
    }

    /// Get the value converted to a specific unit.
    pub fn get(&self, unit: U) -> f64 {
        unit.from_base(self.value_si)
    }
}
