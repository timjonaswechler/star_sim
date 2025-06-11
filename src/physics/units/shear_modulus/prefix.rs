use crate::physics::units::UnitSymbol;
use crate::physics::units::shear_modulus::{ShearModulus, ShearModulusConvertTo, ShearModulusUnit};
use crate::physics::units::prefix::Prefix;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Prefixed<P: Prefix, U: ShearModulusUnit>(PhantomData<(P, U)>);

impl<P: Prefix, U: ShearModulusUnit> ShearModulusUnit for Prefixed<P, U> {}

impl<P, U> UnitSymbol for Prefixed<P, U>
where
    P: Prefix,
    U: ShearModulusUnit + UnitSymbol,
{
    fn symbol() -> String {
        fn make_symbol<P: Prefix, U: ShearModulusUnit + UnitSymbol>() -> String {
            format!("{}{}", P::symbol(), U::symbol())
        }
        make_symbol::<P, U>()
    }
}

impl<P, U> ShearModulusConvertTo<U> for ShearModulus<Prefixed<P, U>>
where
    P: Prefix,
    U: ShearModulusUnit,
{
    fn convert(self) -> ShearModulus<U> {
        ShearModulus::<U>::new(self.value * P::FACTOR)
    }
}

impl<P, U> ShearModulusConvertTo<Prefixed<P, U>> for ShearModulus<U>
where
    P: Prefix,
    U: ShearModulusUnit,
{
    fn convert(self) -> ShearModulus<Prefixed<P, U>> {
        ShearModulus::<Prefixed<P, U>>::new(self.value / P::FACTOR)
    }
}
