use crate::physics::units::UnitSymbol;
use crate::physics::units::power::{Power, PowerConvertTo, PowerUnit, SolarLuminosity, Watt};
use std::fmt;

const WATTS_PER_SOLAR_LUMINOSITY: f64 = 3.828e26; // 1 Solar Luminosity in Watts

impl PowerConvertTo<Watt> for Power<SolarLuminosity> {
    fn convert(self) -> Power<Watt> {
        Power::<Watt>::new(self.value * WATTS_PER_SOLAR_LUMINOSITY)
    }
}

impl PowerConvertTo<SolarLuminosity> for Power<Watt> {
    fn convert(self) -> Power<SolarLuminosity> {
        Power::<SolarLuminosity>::new(self.value / WATTS_PER_SOLAR_LUMINOSITY)
    }
}

impl<U: PowerUnit + UnitSymbol> fmt::Display for Power<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
