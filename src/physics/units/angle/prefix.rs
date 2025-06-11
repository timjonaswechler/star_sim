use crate::physics::units::UnitSymbol;
use crate::physics::units::angle::{Angle, AngleConvertTo, AngleUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: AngleUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: AngleUnit> AngleUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: AngleUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: AngleUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> AngleConvertTo<U> for Angle<Prefixed<P, U>>
where
    P: Prefix,
    U: AngleUnit,
{
    fn convert(self) -> Angle<U> {
        Angle::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> AngleConvertTo<Prefixed<P, U>> for Angle<U>
where
    P: Prefix,
    U: AngleUnit,
{
    fn convert(self) -> Angle<Prefixed<P, U>> {
        Angle::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
