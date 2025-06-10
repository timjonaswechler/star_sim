use crate::physics::units::{Prefix, Time, TimeConvertTo, TimeUnit, UnitSymbol};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Einheiten-Typ mit Präfix.
/// Z. B. `Prefixed<Kilo, Second>` repräsentiert „Kilosekunde“.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: TimeUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: TimeUnit> TimeUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: TimeUnit + UnitSymbol,
{
    // Laufzeit-Konkat. ist hier OK; sie wird nur beim ersten Aufruf gebaut.
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: TimeUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> TimeConvertTo<U> for Time<Prefixed<P, U>>
where
    P: Prefix,
    U: TimeUnit,
{
    fn convert(self) -> Time<U> {
        Time::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> TimeConvertTo<Prefixed<P, U>> for Time<U>
where
    P: Prefix,
    U: TimeUnit,
{
    fn convert(self) -> Time<Prefixed<P, U>> {
        Time::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
