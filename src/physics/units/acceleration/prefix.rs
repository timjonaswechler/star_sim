use crate::physics::units::UnitSymbol;
use crate::physics::units::acceleration::{Acceleration, AccelerationConvertTo, AccelerationUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: AccelerationUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: AccelerationUnit> AccelerationUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: AccelerationUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: AccelerationUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> AccelerationConvertTo<U> for Acceleration<Prefixed<P, U>>
where
    P: Prefix,
    U: AccelerationUnit,
{
    fn convert(self) -> Acceleration<U> {
        Acceleration::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> AccelerationConvertTo<Prefixed<P, U>> for Acceleration<U>
where
    P: Prefix,
    U: AccelerationUnit,
{
    fn convert(self) -> Acceleration<Prefixed<P, U>> {
        Acceleration::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
