use crate::physics::units::UnitSymbol;
use crate::physics::units::angular_acceleration::{
    AngularAcceleration, AngularAccelerationConvertTo, AngularAccelerationUnit,
};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: AngularAccelerationUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: AngularAccelerationUnit> AngularAccelerationUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: AngularAccelerationUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: AngularAccelerationUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> AngularAccelerationConvertTo<U> for AngularAcceleration<Prefixed<P, U>>
where
    P: Prefix,
    U: AngularAccelerationUnit,
{
    fn convert(self) -> AngularAcceleration<U> {
        AngularAcceleration::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> AngularAccelerationConvertTo<Prefixed<P, U>> for AngularAcceleration<U>
where
    P: Prefix,
    U: AngularAccelerationUnit,
{
    fn convert(self) -> AngularAcceleration<Prefixed<P, U>> {
        AngularAcceleration::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
