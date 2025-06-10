use crate::physics::units::prefix::Prefix;
use crate::physics::units::pressure::{Pressure, PressureConvertTo, PressureUnit};
use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

pub struct Prefixed<P: Prefix, U: PressureUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: PressureUnit> PressureUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: PressureUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: PressureUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> PressureConvertTo<U> for Pressure<Prefixed<P, U>>
where
    P: Prefix,
    U: PressureUnit,
{
    fn convert(self) -> Pressure<U> {
        Pressure::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> PressureConvertTo<Prefixed<P, U>> for Pressure<U>
where
    P: Prefix,
    U: PressureUnit,
{
    fn convert(self) -> Pressure<Prefixed<P, U>> {
        Pressure::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
