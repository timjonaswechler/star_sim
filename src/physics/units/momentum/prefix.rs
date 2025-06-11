use crate::physics::units::UnitSymbol;
use crate::physics::units::momentum::{Momentum, MomentumConvertTo, MomentumUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: MomentumUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: MomentumUnit> MomentumUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: MomentumUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: MomentumUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> MomentumConvertTo<U> for Momentum<Prefixed<P, U>>
where
    P: Prefix,
    U: MomentumUnit,
{
    fn convert(self) -> Momentum<U> {
        Momentum::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> MomentumConvertTo<Prefixed<P, U>> for Momentum<U>
where
    P: Prefix,
    U: MomentumUnit,
{
    fn convert(self) -> Momentum<Prefixed<P, U>> {
        Momentum::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
