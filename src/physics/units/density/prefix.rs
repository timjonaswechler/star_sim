use crate::physics::units::UnitSymbol;
use crate::physics::units::density::{Density, DensityConvertTo, DensityUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: DensityUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: DensityUnit> DensityUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: DensityUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: DensityUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> DensityConvertTo<U> for Density<Prefixed<P, U>>
where
    P: Prefix,
    U: DensityUnit,
{
    fn convert(self) -> Density<U> {
        Density::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> DensityConvertTo<Prefixed<P, U>> for Density<U>
where
    P: Prefix,
    U: DensityUnit,
{
    fn convert(self) -> Density<Prefixed<P, U>> {
        Density::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
