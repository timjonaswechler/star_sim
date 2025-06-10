use crate::physics::units::UnitSymbol;
use crate::physics::units::mass::{Mass, MassConvertTo, MassUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Unit type with SI prefix, e.g. `Prefixed<Kilo, Kilogram>` for kilograms.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: MassUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: MassUnit> MassUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: MassUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: MassUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> MassConvertTo<U> for Mass<Prefixed<P, U>>
where
    P: Prefix,
    U: MassUnit,
{
    fn convert(self) -> Mass<U> {
        Mass::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> MassConvertTo<Prefixed<P, U>> for Mass<U>
where
    P: Prefix,
    U: MassUnit,
{
    fn convert(self) -> Mass<Prefixed<P, U>> {
        Mass::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
