use crate::physics::units::UnitSymbol;
use crate::physics::units::frequency::{Frequency, FrequencyConvertTo, FrequencyUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: FrequencyUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: FrequencyUnit> FrequencyUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: FrequencyUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: FrequencyUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> FrequencyConvertTo<U> for Frequency<Prefixed<P, U>>
where
    P: Prefix,
    U: FrequencyUnit,
{
    fn convert(self) -> Frequency<U> {
        Frequency::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> FrequencyConvertTo<Prefixed<P, U>> for Frequency<U>
where
    P: Prefix,
    U: FrequencyUnit,
{
    fn convert(self) -> Frequency<Prefixed<P, U>> {
        Frequency::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
