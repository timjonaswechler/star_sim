use crate::physics::units::UnitSymbol;
use crate::physics::units::momentum::{Momentum, MomentumConvertTo, MomentumUnit, KilogramMeterPerSecond};
use std::fmt;

impl MomentumConvertTo<KilogramMeterPerSecond> for Momentum<KilogramMeterPerSecond> {
    fn convert(self) -> Momentum<KilogramMeterPerSecond> {
        self
    }
}

impl<U: MomentumUnit + UnitSymbol> fmt::Display for Momentum<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
