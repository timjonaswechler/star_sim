use crate::physics::units::UnitSymbol;
use crate::physics::units::prefix::Prefix;
use crate::physics::units::volume::{Volume, VolumeConvertTo, VolumeUnit};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: VolumeUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: VolumeUnit> VolumeUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: VolumeUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: VolumeUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> VolumeConvertTo<U> for Volume<Prefixed<P, U>>
where
    P: Prefix,
    U: VolumeUnit,
{
    fn convert(self) -> Volume<U> {
        Volume::<U>::new(self.value * P::FACTOR.powi(3))
    }
}

impl<P, U> VolumeConvertTo<Prefixed<P, U>> for Volume<U>
where
    P: Prefix,
    U: VolumeUnit,
{
    fn convert(self) -> Volume<Prefixed<P, U>> {
        Volume::<Prefixed<P, U>>::new(self.value / P::FACTOR.powi(3))
    }
}
