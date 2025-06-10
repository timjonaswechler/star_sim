use crate::physics::units::UnitSymbol;
use crate::physics::units::energy::{Energy, EnergyConvertTo, EnergyUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Prefixed<P: Prefix, U: EnergyUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: EnergyUnit> EnergyUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: EnergyUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: EnergyUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> EnergyConvertTo<U> for Energy<Prefixed<P, U>>
where
    P: Prefix,
    U: EnergyUnit,
{
    fn convert(self) -> Energy<U> {
        Energy::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> EnergyConvertTo<Prefixed<P, U>> for Energy<U>
where
    P: Prefix,
    U: EnergyUnit,
{
    fn convert(self) -> Energy<Prefixed<P, U>> {
        Energy::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
