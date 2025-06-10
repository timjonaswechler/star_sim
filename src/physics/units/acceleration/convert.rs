use crate::physics::units::UnitSymbol;
use crate::physics::units::acceleration::{
    Acceleration, AccelerationConvertTo, AccelerationUnit, MeterPerSecondSquared, StandardGravity,
};
use std::fmt;

const MPS2_PER_G: f64 = 9.80665;

impl AccelerationConvertTo<MeterPerSecondSquared> for Acceleration<StandardGravity> {
    fn convert(self) -> Acceleration<MeterPerSecondSquared> {
        Acceleration::<MeterPerSecondSquared>::new(self.value * MPS2_PER_G)
    }
}

impl AccelerationConvertTo<StandardGravity> for Acceleration<MeterPerSecondSquared> {
    fn convert(self) -> Acceleration<StandardGravity> {
        Acceleration::<StandardGravity>::new(self.value / MPS2_PER_G)
    }
}

impl<U: AccelerationUnit + UnitSymbol> fmt::Display for Acceleration<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
