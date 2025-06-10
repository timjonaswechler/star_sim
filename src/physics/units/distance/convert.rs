use crate::physics::units::{
    AstronomicalUnit, Distance, DistanceConvertTo, DistanceUnit, EarthRadius, Meter,
};
use crate::physics::units::{SunRadius, UnitSymbol};
use std::fmt;

const METERS_PER_AU: f64 = 149_597_870_700.0; // 1 AU in meters
const METERS_PER_EARTH_RADIUS: f64 = 6_371_000.0; // 1 Earth radius in meters
const METERS_PER_SUN_RADIUS: f64 = 696_340_000.0; // 1 Sun radius in meters

impl DistanceConvertTo<Meter> for Distance<AstronomicalUnit> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_AU)
    }
}

impl DistanceConvertTo<AstronomicalUnit> for Distance<Meter> {
    fn convert(self) -> Distance<AstronomicalUnit> {
        Distance::<AstronomicalUnit>::new(self.value / METERS_PER_AU)
    }
}

impl DistanceConvertTo<Meter> for Distance<EarthRadius> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_EARTH_RADIUS)
    }
}

impl DistanceConvertTo<EarthRadius> for Distance<Meter> {
    fn convert(self) -> Distance<EarthRadius> {
        Distance::<EarthRadius>::new(self.value / METERS_PER_EARTH_RADIUS)
    }
}

impl DistanceConvertTo<Meter> for Distance<SunRadius> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_SUN_RADIUS)
    }
}

impl DistanceConvertTo<SunRadius> for Distance<Meter> {
    fn convert(self) -> Distance<SunRadius> {
        Distance::<SunRadius>::new(self.value / METERS_PER_SUN_RADIUS)
    }
}

// Display
impl<U: DistanceUnit + UnitSymbol> fmt::Display for Distance<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
