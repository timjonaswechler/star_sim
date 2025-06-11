use crate::physics::units::UnitSymbol;
use crate::physics::units::area::{Area, AreaConvertTo, AreaUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: AreaUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: AreaUnit> AreaUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: AreaUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: AreaUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> AreaConvertTo<U> for Area<Prefixed<P, U>>
where
    P: Prefix,
    U: AreaUnit,
{
    fn convert(self) -> Area<U> {
        Area::<U>::new(self.value * P::FACTOR.powi(2))
    }
}

impl<P, U> AreaConvertTo<Prefixed<P, U>> for Area<U>
where
    P: Prefix,
    U: AreaUnit,
{
    fn convert(self) -> Area<Prefixed<P, U>> {
        Area::<Prefixed<P, U>>::new(self.value / P::FACTOR.powi(2))
    }
}
