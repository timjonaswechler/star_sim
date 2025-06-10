use crate::physics::units::UnitSymbol;
use crate::physics::units::force::{Force, ForceUnit};
use std::fmt;

impl<U: ForceUnit + UnitSymbol> fmt::Display for Force<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
