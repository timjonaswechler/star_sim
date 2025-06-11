use crate::physics::units::UnitSymbol;
use crate::physics::units::angular_velocity::{AngularVelocity, AngularVelocityConvertTo, AngularVelocityUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: AngularVelocityUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: AngularVelocityUnit> AngularVelocityUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: AngularVelocityUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: AngularVelocityUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> AngularVelocityConvertTo<U> for AngularVelocity<Prefixed<P, U>>
where
    P: Prefix,
    U: AngularVelocityUnit,
{
    fn convert(self) -> AngularVelocity<U> {
        AngularVelocity::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> AngularVelocityConvertTo<Prefixed<P, U>> for AngularVelocity<U>
where
    P: Prefix,
    U: AngularVelocityUnit,
{
    fn convert(self) -> AngularVelocity<Prefixed<P, U>> {
        AngularVelocity::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
