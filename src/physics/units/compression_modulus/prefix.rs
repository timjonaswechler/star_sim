use crate::physics::units::UnitSymbol;
use crate::physics::units::compression_modulus::{CompressionModulus, CompressionModulusConvertTo, CompressionModulusUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: CompressionModulusUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: CompressionModulusUnit> CompressionModulusUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: CompressionModulusUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: CompressionModulusUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> CompressionModulusConvertTo<U> for CompressionModulus<Prefixed<P, U>>
where
    P: Prefix,
    U: CompressionModulusUnit,
{
    fn convert(self) -> CompressionModulus<U> {
        CompressionModulus::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> CompressionModulusConvertTo<Prefixed<P, U>> for CompressionModulus<U>
where
    P: Prefix,
    U: CompressionModulusUnit,
{
    fn convert(self) -> CompressionModulus<Prefixed<P, U>> {
        CompressionModulus::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
