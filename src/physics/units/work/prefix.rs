use crate::physics::units::UnitSymbol;
use crate::physics::units::work::{Work, WorkConvertTo, WorkUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: WorkUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: WorkUnit> WorkUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: WorkUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: WorkUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> WorkConvertTo<U> for Work<Prefixed<P, U>>
where
    P: Prefix,
    U: WorkUnit,
{
    fn convert(self) -> Work<U> {
        Work::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> WorkConvertTo<Prefixed<P, U>> for Work<U>
where
    P: Prefix,
    U: WorkUnit,
{
    fn convert(self) -> Work<Prefixed<P, U>> {
        Work::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
