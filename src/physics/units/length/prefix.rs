use crate::physics::units::length::{Distance, LengthConvertTo, LengthUnit};
use crate::physics::units::prefix::Prefix;
use crate::physics::units::UnitSymbol;
use std::marker::PhantomData;

/// Unit type with prefix.
pub struct Prefixed<P: Prefix, U: LengthUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: LengthUnit> LengthUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: LengthUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: LengthUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> LengthConvertTo<U> for Distance<Prefixed<P, U>>
where
    P: Prefix,
    U: LengthUnit,
{
    fn convert(self) -> Distance<U> {
        Distance::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> LengthConvertTo<Prefixed<P, U>> for Distance<U>
where
    P: Prefix,
    U: LengthUnit,
{
    fn convert(self) -> Distance<Prefixed<P, U>> {
        Distance::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
