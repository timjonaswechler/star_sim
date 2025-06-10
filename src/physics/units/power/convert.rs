use crate::physics::constants::stellar::SOLAR_LUMINOSITY_IN_WATTS;
use crate::physics::units::power::{Power, PowerConvertTo, PowerUnit, SolarLuminosity, Watt};
use crate::physics::units::UnitSymbol;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

const WATTS_PER_SOLAR_LUMINOSITY: f64 = SOLAR_LUMINOSITY_IN_WATTS;
const SOLAR_LUMINOSITY_PER_WATT: f64 = 1.0 / WATTS_PER_SOLAR_LUMINOSITY;

impl PowerConvertTo<Watt> for Power<SolarLuminosity> {
    fn convert(self) -> Power<Watt> {
        Power::<Watt>::new(self.value * WATTS_PER_SOLAR_LUMINOSITY)
    }
}

impl PowerConvertTo<SolarLuminosity> for Power<Watt> {
    fn convert(self) -> Power<SolarLuminosity> {
        Power::<SolarLuminosity>::new(self.value * SOLAR_LUMINOSITY_PER_WATT)
    }
}

impl<U: PowerUnit + UnitSymbol> fmt::Display for Power<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: PowerUnit> Add for Power<U> {
    type Output = Power<U>;
    fn add(self, other: Power<U>) -> Power<U> {
        Power::new(self.value + other.value)
    }
}

impl<U: PowerUnit> Sub for Power<U> {
    type Output = Power<U>;
    fn sub(self, other: Power<U>) -> Power<U> {
        Power::new(self.value - other.value)
    }
}

impl<U: PowerUnit> Mul<f64> for Power<U> {
    type Output = Power<U>;
    fn mul(self, scalar: f64) -> Power<U> {
        Power::new(self.value * scalar)
    }
}

impl<U: PowerUnit> Div<f64> for Power<U> {
    type Output = Power<U>;
    fn div(self, scalar: f64) -> Power<U> {
        Power::new(self.value / scalar)
    }
}
