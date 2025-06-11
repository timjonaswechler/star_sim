use crate::physics::units::UnitSymbol;
use crate::physics::units::shear_modulus::{ShearModulus, ShearModulusConvertTo, ShearModulusUnit, Pascal};
use std::fmt;

impl ShearModulusConvertTo<Pascal> for ShearModulus<Pascal> {
    fn convert(self) -> ShearModulus<Pascal> {
        self
    }
}

impl<U: ShearModulusUnit + UnitSymbol> fmt::Display for ShearModulus<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
