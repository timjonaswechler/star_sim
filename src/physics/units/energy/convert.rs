use crate::physics::units::UnitSymbol;
use crate::physics::units::energy::{Energy, EnergyUnit};
use std::fmt;

impl<U: EnergyUnit + UnitSymbol> fmt::Display for Energy<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
