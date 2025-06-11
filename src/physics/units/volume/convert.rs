use crate::physics::units::UnitSymbol;
use crate::physics::units::volume::{
    CubicMeter, Liter, Volume, VolumeConvertTo, VolumeUnit,
};
use std::fmt;

const LITERS_PER_M3: f64 = 1000.0;

impl VolumeConvertTo<Liter> for Volume<CubicMeter> {
    fn convert(self) -> Volume<Liter> {
        Volume::<Liter>::new(self.value * LITERS_PER_M3)
    }
}

impl VolumeConvertTo<CubicMeter> for Volume<Liter> {
    fn convert(self) -> Volume<CubicMeter> {
        Volume::<CubicMeter>::new(self.value / LITERS_PER_M3)
    }
}

impl<U: VolumeUnit + UnitSymbol> fmt::Display for Volume<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
