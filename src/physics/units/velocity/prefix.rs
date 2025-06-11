use crate::physics::units::UnitSymbol;
use crate::physics::units::prefix::Prefix;
use crate::physics::units::velocity::{Velocity, VelocityConvertTo, VelocityUnit};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Einheiten-Typ mit Präfix.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: VelocityUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: VelocityUnit> VelocityUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: VelocityUnit + UnitSymbol,
{
    // Laufzeit-Konkat. ist hier OK; sie wird nur beim ersten Aufruf gebaut.
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: VelocityUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> VelocityConvertTo<U> for Velocity<Prefixed<P, U>>
where
    P: Prefix,
    U: VelocityUnit,
{
    fn convert(self) -> Velocity<U> {
        Velocity::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> VelocityConvertTo<Prefixed<P, U>> for Velocity<U>
where
    P: Prefix,
    U: VelocityUnit,
{
    fn convert(self) -> Velocity<Prefixed<P, U>> {
        Velocity::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
