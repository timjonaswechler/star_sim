use crate::physics::units::{
    AstronomicalUnit, Distance, DistanceConvertTo, DistanceUnit, EarthRadius, LightYear, Meter,
    Parsec,
};
use crate::physics::units::{SunRadius, UnitSymbol};
use std::fmt;

const METERS_PER_AU: f64 = 149_597_870_700.0; // 1 AU in meters
const METERS_PER_EARTH_RADIUS: f64 = 6_371_000.0; // 1 Earth radius in meters
const METERS_PER_SUN_RADIUS: f64 = 696_340_000.0; // 1 Sun radius in meters
const METERS_PER_LIGHT_YEAR: f64 = 9_461_000_000_000.0; // 1 light year in meters
const METERS_PER_PARSEC: f64 = 3.085677581491367e16; // 1 parsec in meters

//Meter
impl DistanceConvertTo<Meter> for Distance<AstronomicalUnit> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_AU)
    }
}
impl DistanceConvertTo<Meter> for Distance<EarthRadius> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_EARTH_RADIUS)
    }
}
impl DistanceConvertTo<Meter> for Distance<SunRadius> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_SUN_RADIUS)
    }
}
impl DistanceConvertTo<Meter> for Distance<LightYear> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_LIGHT_YEAR)
    }
}
impl DistanceConvertTo<Meter> for Distance<Parsec> {
    fn convert(self) -> Distance<Meter> {
        Distance::<Meter>::new(self.value * METERS_PER_PARSEC)
    }
}

// AstronomicalUnit
impl DistanceConvertTo<AstronomicalUnit> for Distance<Meter> {
    fn convert(self) -> Distance<AstronomicalUnit> {
        Distance::<AstronomicalUnit>::new(self.value / METERS_PER_AU)
    }
}
impl DistanceConvertTo<AstronomicalUnit> for Distance<EarthRadius> {
    fn convert(self) -> Distance<AstronomicalUnit> {
        Distance::<AstronomicalUnit>::new(self.value / METERS_PER_AU * METERS_PER_EARTH_RADIUS)
    }
}
impl DistanceConvertTo<AstronomicalUnit> for Distance<SunRadius> {
    fn convert(self) -> Distance<AstronomicalUnit> {
        Distance::<AstronomicalUnit>::new(self.value / METERS_PER_AU * METERS_PER_SUN_RADIUS)
    }
}
impl DistanceConvertTo<AstronomicalUnit> for Distance<LightYear> {
    fn convert(self) -> Distance<AstronomicalUnit> {
        Distance::<AstronomicalUnit>::new(self.value * METERS_PER_LIGHT_YEAR / METERS_PER_AU)
    }
}
impl DistanceConvertTo<AstronomicalUnit> for Distance<Parsec> {
    fn convert(self) -> Distance<AstronomicalUnit> {
        Distance::<AstronomicalUnit>::new(self.value * METERS_PER_PARSEC / METERS_PER_AU)
    }
}

// EarthRadius
impl DistanceConvertTo<EarthRadius> for Distance<Meter> {
    fn convert(self) -> Distance<EarthRadius> {
        Distance::<EarthRadius>::new(self.value / METERS_PER_EARTH_RADIUS)
    }
}
impl DistanceConvertTo<EarthRadius> for Distance<AstronomicalUnit> {
    fn convert(self) -> Distance<EarthRadius> {
        Distance::<EarthRadius>::new(self.value * METERS_PER_AU / METERS_PER_EARTH_RADIUS)
    }
}
impl DistanceConvertTo<EarthRadius> for Distance<SunRadius> {
    fn convert(self) -> Distance<EarthRadius> {
        Distance::<EarthRadius>::new(self.value * METERS_PER_SUN_RADIUS / METERS_PER_EARTH_RADIUS)
    }
}
impl DistanceConvertTo<EarthRadius> for Distance<LightYear> {
    fn convert(self) -> Distance<EarthRadius> {
        Distance::<EarthRadius>::new(self.value * METERS_PER_LIGHT_YEAR / METERS_PER_EARTH_RADIUS)
    }
}
impl DistanceConvertTo<EarthRadius> for Distance<Parsec> {
    fn convert(self) -> Distance<EarthRadius> {
        Distance::<EarthRadius>::new(self.value * METERS_PER_PARSEC / METERS_PER_EARTH_RADIUS)
    }
}

// SunRadius
impl DistanceConvertTo<SunRadius> for Distance<Meter> {
    fn convert(self) -> Distance<SunRadius> {
        Distance::<SunRadius>::new(self.value / METERS_PER_SUN_RADIUS)
    }
}
impl DistanceConvertTo<SunRadius> for Distance<AstronomicalUnit> {
    fn convert(self) -> Distance<SunRadius> {
        Distance::<SunRadius>::new(self.value * METERS_PER_AU / METERS_PER_SUN_RADIUS)
    }
}
impl DistanceConvertTo<SunRadius> for Distance<EarthRadius> {
    fn convert(self) -> Distance<SunRadius> {
        Distance::<SunRadius>::new(self.value * METERS_PER_EARTH_RADIUS / METERS_PER_SUN_RADIUS)
    }
}
impl DistanceConvertTo<SunRadius> for Distance<LightYear> {
    fn convert(self) -> Distance<SunRadius> {
        Distance::<SunRadius>::new(self.value * METERS_PER_LIGHT_YEAR / METERS_PER_SUN_RADIUS)
    }
}
impl DistanceConvertTo<SunRadius> for Distance<Parsec> {
    fn convert(self) -> Distance<SunRadius> {
        Distance::<SunRadius>::new(self.value * METERS_PER_PARSEC / METERS_PER_SUN_RADIUS)
    }
}

// LightYear
impl DistanceConvertTo<LightYear> for Distance<Meter> {
    fn convert(self) -> Distance<LightYear> {
        Distance::<LightYear>::new(self.value / METERS_PER_LIGHT_YEAR)
    }
}
impl DistanceConvertTo<LightYear> for Distance<AstronomicalUnit> {
    fn convert(self) -> Distance<LightYear> {
        Distance::<LightYear>::new(self.value * METERS_PER_AU / METERS_PER_LIGHT_YEAR)
    }
}
impl DistanceConvertTo<LightYear> for Distance<EarthRadius> {
    fn convert(self) -> Distance<LightYear> {
        Distance::<LightYear>::new(self.value * METERS_PER_EARTH_RADIUS / METERS_PER_LIGHT_YEAR)
    }
}
impl DistanceConvertTo<LightYear> for Distance<SunRadius> {
    fn convert(self) -> Distance<LightYear> {
        Distance::<LightYear>::new(self.value * METERS_PER_SUN_RADIUS / METERS_PER_LIGHT_YEAR)
    }
}
impl DistanceConvertTo<LightYear> for Distance<Parsec> {
    fn convert(self) -> Distance<LightYear> {
        Distance::<LightYear>::new(self.value * METERS_PER_PARSEC / METERS_PER_LIGHT_YEAR)
    }
}

// Parsec
impl DistanceConvertTo<Parsec> for Distance<Meter> {
    fn convert(self) -> Distance<Parsec> {
        Distance::<Parsec>::new(self.value / METERS_PER_PARSEC)
    }
}
impl DistanceConvertTo<Parsec> for Distance<AstronomicalUnit> {
    fn convert(self) -> Distance<Parsec> {
        Distance::<Parsec>::new(self.value * METERS_PER_AU / METERS_PER_PARSEC)
    }
}
impl DistanceConvertTo<Parsec> for Distance<EarthRadius> {
    fn convert(self) -> Distance<Parsec> {
        Distance::<Parsec>::new(self.value * METERS_PER_EARTH_RADIUS / METERS_PER_PARSEC)
    }
}
impl DistanceConvertTo<Parsec> for Distance<SunRadius> {
    fn convert(self) -> Distance<Parsec> {
        Distance::<Parsec>::new(self.value * METERS_PER_SUN_RADIUS / METERS_PER_PARSEC)
    }
}
impl DistanceConvertTo<Parsec> for Distance<LightYear> {
    fn convert(self) -> Distance<Parsec> {
        Distance::<Parsec>::new(self.value * METERS_PER_LIGHT_YEAR / METERS_PER_PARSEC)
    }
}

// Display
impl<U: DistanceUnit + UnitSymbol> fmt::Display for Distance<U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, U::symbol())
    }
}
