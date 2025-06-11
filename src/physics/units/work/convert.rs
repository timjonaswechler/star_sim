use crate::physics::units::UnitSymbol;
use crate::physics::units::work::{Work, WorkConvertTo, WorkUnit, Joule};
use std::fmt;

impl WorkConvertTo<Joule> for Work<Joule> {
    fn convert(self) -> Work<Joule> {
        self
    }
}

impl<U: WorkUnit + UnitSymbol> fmt::Display for Work<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
