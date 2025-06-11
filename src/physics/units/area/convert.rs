use crate::physics::units::UnitSymbol;
use crate::physics::units::area::{
    Area, AreaConvertTo, AreaUnit, SquareKilometer, SquareMeter,
};
use std::fmt;

const M2_PER_KM2: f64 = 1_000_000.0;

impl AreaConvertTo<SquareMeter> for Area<SquareKilometer> {
    fn convert(self) -> Area<SquareMeter> {
        Area::<SquareMeter>::new(self.value * M2_PER_KM2)
    }
}

impl AreaConvertTo<SquareKilometer> for Area<SquareMeter> {
    fn convert(self) -> Area<SquareKilometer> {
        Area::<SquareKilometer>::new(self.value / M2_PER_KM2)
    }
}

impl<U: AreaUnit + UnitSymbol> fmt::Display for Area<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
