use crate::physics::units::UnitSymbol;
use crate::physics::units::elastic_modulus::{ElasticModulus, ElasticModulusConvertTo, ElasticModulusUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: ElasticModulusUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: ElasticModulusUnit> ElasticModulusUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: ElasticModulusUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: ElasticModulusUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> ElasticModulusConvertTo<U> for ElasticModulus<Prefixed<P, U>>
where
    P: Prefix,
    U: ElasticModulusUnit,
{
    fn convert(self) -> ElasticModulus<U> {
        ElasticModulus::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> ElasticModulusConvertTo<Prefixed<P, U>> for ElasticModulus<U>
where
    P: Prefix,
    U: ElasticModulusUnit,
{
    fn convert(self) -> ElasticModulus<Prefixed<P, U>> {
        ElasticModulus::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
