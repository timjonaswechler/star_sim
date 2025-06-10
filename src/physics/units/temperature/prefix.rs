use crate::physics::units::UnitSymbol;
use crate::physics::units::prefix::Prefix;
use crate::physics::units::temperature::{Temperature, TemperatureConvertTo, TemperatureUnit};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: TemperatureUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: TemperatureUnit> TemperatureUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: TemperatureUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: TemperatureUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> TemperatureConvertTo<U> for Temperature<Prefixed<P, U>>
where
    P: Prefix,
    U: TemperatureUnit,
{
    fn convert(self) -> Temperature<U> {
        Temperature::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> TemperatureConvertTo<Prefixed<P, U>> for Temperature<U>
where
    P: Prefix,
    U: TemperatureUnit,
{
    fn convert(self) -> Temperature<Prefixed<P, U>> {
        Temperature::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
