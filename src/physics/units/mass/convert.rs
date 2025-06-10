use crate::physics::constants::stellar::{EARTH_MASS_IN_KG, SOLAR_MASS_IN_KG};
use crate::physics::units::mass::{EarthMass, Kilogram, Mass, MassConvertTo, MassUnit, SolarMass};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// Mass conversions
const KILOGRAMS_PER_EARTH_MASS: f64 = EARTH_MASS_IN_KG;
const EARTH_MASSES_PER_KILOGRAM: f64 = 1.0 / KILOGRAMS_PER_EARTH_MASS;
const KILOGRAMS_PER_SOLAR_MASS: f64 = SOLAR_MASS_IN_KG;
const SOLAR_MASSES_PER_KILOGRAM: f64 = 1.0 / KILOGRAMS_PER_SOLAR_MASS;

impl MassConvertTo<Kilogram> for Mass<EarthMass> {
    fn convert(self) -> Mass<Kilogram> {
        Mass::<Kilogram>::new(self.value * KILOGRAMS_PER_EARTH_MASS)
    }
}

impl MassConvertTo<EarthMass> for Mass<Kilogram> {
    fn convert(self) -> Mass<EarthMass> {
        Mass::<EarthMass>::new(self.value * EARTH_MASSES_PER_KILOGRAM)
    }
}

impl MassConvertTo<Kilogram> for Mass<SolarMass> {
    fn convert(self) -> Mass<Kilogram> {
        Mass::<Kilogram>::new(self.value * KILOGRAMS_PER_SOLAR_MASS)
    }
}

impl MassConvertTo<SolarMass> for Mass<Kilogram> {
    fn convert(self) -> Mass<SolarMass> {
        Mass::<SolarMass>::new(self.value * SOLAR_MASSES_PER_KILOGRAM)
    }
}

// Display implementation
impl<U: MassUnit + UnitSymbol> fmt::Display for Mass<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: MassUnit> Add for Mass<U> {
    type Output = Mass<U>;
    fn add(self, other: Mass<U>) -> Mass<U> {
        Mass::new(self.value + other.value)
    }
}

impl<U: MassUnit> Sub for Mass<U> {
    type Output = Mass<U>;
    fn sub(self, other: Mass<U>) -> Mass<U> {
        Mass::new(self.value - other.value)
    }
}

impl<U: MassUnit> Mul<f64> for Mass<U> {
    type Output = Mass<U>;
    fn mul(self, scalar: f64) -> Mass<U> {
        Mass::new(self.value * scalar)
    }
}

impl<U: MassUnit> Div<f64> for Mass<U> {
    type Output = Mass<U>;
    fn div(self, scalar: f64) -> Mass<U> {
        Mass::new(self.value / scalar)
    }
}
