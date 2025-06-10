use crate::physics::constants::common::{KM_TO_M, M_TO_KM};
use crate::physics::constants::stellar::{AU_IN_METERS, EARTH_RADIUS_IN_METERS};
use crate::physics::units::length::{
    AstronomicalUnit, Distance, EarthRadius, Kilometer, LengthConvertTo, LengthUnit, Meter,
};
use crate::physics::units::UnitSymbol;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

const METERS_PER_AU: f64 = AU_IN_METERS;
const AU_PER_METER: f64 = 1.0 / METERS_PER_AU;
const METERS_PER_EARTH_RADIUS: f64 = EARTH_RADIUS_IN_METERS;
const EARTH_RADII_PER_METER: f64 = 1.0 / METERS_PER_EARTH_RADIUS;

impl LengthConvertTo<Meter> for Distance<Kilometer> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * KM_TO_M)
    }
}

impl LengthConvertTo<Kilometer> for Distance<Meter> {
    fn convert(self) -> Distance<Kilometer> {
        Distance::<Kilometer>::new(self.value * M_TO_KM)
    }
}

impl LengthConvertTo<Meter> for Distance<AstronomicalUnit> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_AU)
    }
}

impl LengthConvertTo<AstronomicalUnit> for Distance<Meter> {
    fn convert(self) -> Distance<AstronomicalUnit> {
        Distance::<AstronomicalUnit>::new(self.value * AU_PER_METER)
    }
}

impl LengthConvertTo<Meter> for Distance<EarthRadius> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_EARTH_RADIUS)
    }
}

impl LengthConvertTo<EarthRadius> for Distance<Meter> {
    fn convert(self) -> Distance<EarthRadius> {
        Distance::<EarthRadius>::new(self.value * EARTH_RADII_PER_METER)
    }
}

// Display
impl<U: LengthUnit + UnitSymbol> fmt::Display for Distance<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}

impl<U: LengthUnit> Add for Distance<U> {
    type Output = Distance<U>;
    fn add(self, other: Distance<U>) -> Distance<U> {
        Distance::new(self.value + other.value)
    }
}

impl<U: LengthUnit> Sub for Distance<U> {
    type Output = Distance<U>;
    fn sub(self, other: Distance<U>) -> Distance<U> {
        Distance::new(self.value - other.value)
    }
}

impl<U: LengthUnit> Mul<f64> for Distance<U> {
    type Output = Distance<U>;
    fn mul(self, scalar: f64) -> Distance<U> {
        Distance::new(self.value * scalar)
    }
}

impl<U: LengthUnit> Div<f64> for Distance<U> {
    type Output = Distance<U>;
    fn div(self, scalar: f64) -> Distance<U> {
        Distance::new(self.value / scalar)
    }
}
