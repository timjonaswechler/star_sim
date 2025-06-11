use crate::physics::units::UnitSymbol;
use crate::physics::units::force::{Force, ForceConvertTo, ForceUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: ForceUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: ForceUnit> ForceUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: ForceUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: ForceUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> ForceConvertTo<U> for Force<Prefixed<P, U>>
where
    P: Prefix,
    U: ForceUnit,
{
    fn convert(self) -> Force<U> {
        Force::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> ForceConvertTo<Prefixed<P, U>> for Force<U>
where
    P: Prefix,
    U: ForceUnit,
{
    fn convert(self) -> Force<Prefixed<P, U>> {
        Force::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
