use crate::physics::units::UnitSymbol;
use crate::physics::units::compression_modulus::{CompressionModulus, CompressionModulusConvertTo, CompressionModulusUnit, Pascal};
use std::fmt;

impl CompressionModulusConvertTo<Pascal> for CompressionModulus<Pascal> {
    fn convert(self) -> CompressionModulus<Pascal> {
        self
    }
}

impl<U: CompressionModulusUnit + UnitSymbol> fmt::Display for CompressionModulus<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
