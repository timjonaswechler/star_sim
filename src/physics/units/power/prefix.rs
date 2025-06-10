use crate::physics::units::UnitSymbol;
use crate::physics::units::power::{Power, PowerConvertTo, PowerUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: PowerUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: PowerUnit> PowerUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: PowerUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: PowerUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> PowerConvertTo<U> for Power<Prefixed<P, U>>
where
    P: Prefix,
    U: PowerUnit,
{
    fn convert(self) -> Power<U> {
        Power::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> PowerConvertTo<Prefixed<P, U>> for Power<U>
where
    P: Prefix,
    U: PowerUnit,
{
    fn convert(self) -> Power<Prefixed<P, U>> {
        Power::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
