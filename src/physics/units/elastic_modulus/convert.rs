use crate::physics::units::UnitSymbol;
use crate::physics::units::elastic_modulus::{ElasticModulus, ElasticModulusConvertTo, ElasticModulusUnit, Pascal};
use std::fmt;

impl ElasticModulusConvertTo<Pascal> for ElasticModulus<Pascal> {
    fn convert(self) -> ElasticModulus<Pascal> {
        self
    }
}

impl<U: ElasticModulusUnit + UnitSymbol> fmt::Display for ElasticModulus<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
