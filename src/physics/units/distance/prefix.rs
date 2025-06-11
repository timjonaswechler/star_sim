use crate::physics::units::{Distance, DistanceConvertTo, DistanceUnit, Prefix, UnitSymbol};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Unit type with prefix.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: DistanceUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: DistanceUnit> DistanceUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: DistanceUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: DistanceUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> DistanceConvertTo<U> for Distance<Prefixed<P, U>>
where
    P: Prefix,
    U: DistanceUnit,
{
    fn convert(self) -> Distance<U> {
        Distance::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> DistanceConvertTo<Prefixed<P, U>> for Distance<U>
where
    P: Prefix,
    U: DistanceUnit,
{
    fn convert(self) -> Distance<Prefixed<P, U>> {
        Distance::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
