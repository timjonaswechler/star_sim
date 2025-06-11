use crate::physics::units::UnitSymbol;
use crate::physics::units::frequency::{Frequency, FrequencyConvertTo, FrequencyUnit, Hertz};
use std::fmt;

impl FrequencyConvertTo<Hertz> for Frequency<Hertz> {
    fn convert(self) -> Frequency<Hertz> {
        self
    }
}

impl<U: FrequencyUnit + UnitSymbol> fmt::Display for Frequency<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
