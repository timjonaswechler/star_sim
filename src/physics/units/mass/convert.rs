use crate::physics::units::{
    EarthMass, Kilogram, Mass, MassConvertTo, MassUnit, SolarMass, UnitSymbol,
};
use std::fmt;

// Mass conversions
const KILOGRAMS_PER_EARTH_MASS: f64 = 5.972e24; // 1 Earth mass in kilograms
const KILOGRAMS_PER_SOLAR_MASS: f64 = 1.989e30; // 1 Solar mass in kilograms

impl MassConvertTo<Kilogram> for Mass<EarthMass> {
    fn convert(self) -> Mass<Kilogram> {
        Mass::<Kilogram>::new(self.value * KILOGRAMS_PER_EARTH_MASS)
    }
}

impl MassConvertTo<EarthMass> for Mass<Kilogram> {
    fn convert(self) -> Mass<EarthMass> {
        Mass::<EarthMass>::new(self.value / KILOGRAMS_PER_EARTH_MASS)
    }
}

impl MassConvertTo<Kilogram> for Mass<SolarMass> {
    fn convert(self) -> Mass<Kilogram> {
        Mass::<Kilogram>::new(self.value * KILOGRAMS_PER_SOLAR_MASS)
    }
}

impl MassConvertTo<SolarMass> for Mass<Kilogram> {
    fn convert(self) -> Mass<SolarMass> {
        Mass::<SolarMass>::new(self.value / KILOGRAMS_PER_SOLAR_MASS)
    }
}

// Display implementation
impl<U: MassUnit + UnitSymbol> fmt::Display for Mass<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
