use crate::physics::units::constants::*;
use crate::physics::units::core::*;
use crate::{define_quantity, define_unit_dimension};

// Define basic quantity types using dimensional analysis
define_quantity!(Distance, 1, 0, 0, 0, 0, 0, 0); // Length
define_quantity!(Mass, 0, 1, 0, 0, 0, 0, 0); // Mass
define_quantity!(Time, 0, 0, 1, 0, 0, 0, 0); // Time
define_quantity!(Temperature, 0, 0, 0, 1, 0, 0, 0); // Temperature
define_quantity!(Current, 0, 0, 0, 0, 1, 0, 0); // Current
define_quantity!(LuminousIntensity, 0, 0, 0, 0, 0, 1, 0); // Luminous Intensity
define_quantity!(AmountOfSubstance, 0, 0, 0, 0, 0, 0, 1); // Amount

// Derived quantities
define_quantity!(Area, 2, 0, 0, 0, 0, 0, 0); // Length²
define_quantity!(Volume, 3, 0, 0, 0, 0, 0, 0); // Length³
define_quantity!(Velocity, 1, 0, -1, 0, 0, 0, 0); // Length/Time
define_quantity!(Acceleration, 1, 0, -2, 0, 0, 0, 0); // Length/Time²
define_quantity!(Force, 1, 1, -2, 0, 0, 0, 0); // Mass×Length/Time²
define_quantity!(Energy, 2, 1, -2, 0, 0, 0, 0); // Mass×Length²/Time²
define_quantity!(Power, 2, 1, -3, 0, 0, 0, 0); // Mass×Length²/Time³
define_quantity!(Pressure, -1, 1, -2, 0, 0, 0, 0); // Mass/(Length×Time²)
define_quantity!(Density, -3, 1, 0, 0, 0, 0, 0); // Mass/Length³
define_quantity!(Frequency, 0, 0, -1, 0, 0, 0, 0); // 1/Time

// Angular quantities (dimensionless in SI but physically important)
define_quantity!(Angle, 0, 0, 0, 0, 0, 0, 0); // Dimensionless
define_quantity!(AngularVelocity, 0, 0, -1, 0, 0, 0, 0); // 1/Time
define_quantity!(AngularAcceleration, 0, 0, -2, 0, 0, 0, 0); // 1/Time²

// Additional derived quantities
define_quantity!(Momentum, 1, 1, -1, 0, 0, 0, 0); // Mass×Length/Time

// Define Distance units with astronomical focus

define_unit_dimension! {
    dimension Distance {
        base_unit: Meter = 1.0,
        units: {
            Meter = 1.0,
            Kilometer = 1000.0,
            AstronomicalUnit = METERS_PER_AU,
            EarthRadius = METERS_PER_EARTH_RADIUS,
            SunRadius = METERS_PER_SUN_RADIUS,
            LightYear = METERS_PER_LIGHT_YEAR,
            Parsec = METERS_PER_PARSEC,

            Kiloparsec = METERS_PER_KILOPARSEC,
        },
        symbols: {
            Meter = "m",
            Kilometer = "km",
            AstronomicalUnit = "AU",
            EarthRadius = "R⊕",
            SunRadius = "R☉",
            LightYear = "ly",
            Parsec = "pc",
            Kiloparsec = "kpc",
        }
    }
}

// Define Mass units with astronomical focus
define_unit_dimension! {
    dimension Mass {
        base_unit: Kilogram = 1.0,
        units: {
            Gram = KG_PER_GRAM,
            Kilogram = 1.0,
            EarthMass = KG_PER_EARTH_MASS,
            SolarMass = KG_PER_SOLAR_MASS,
        },
        symbols: {
            Gram = "g",
            Kilogram = "kg",
            EarthMass = "M⊕",
            SolarMass = "M☉",
        }
    }
}

// Define Time units with astronomical focus
define_unit_dimension! {
    dimension Time {
        base_unit: Second = 1.0,
        units: {
            Second = 1.0,
            Minute = SECONDS_PER_MINUTE,
            Hour = SECONDS_PER_HOUR,
            Day = SECONDS_PER_DAY,
            Year = SECONDS_PER_YEAR,
            Gigayear = SECONDS_PER_GIGAYEAR,
        },
        symbols: {
            Second = "s",
            Minute = "min",
            Hour = "h",
            Day = "d",
            Year = "yr",
            Gigayear = "Gyr",
        }
    }
}

// Define Temperature units
define_unit_dimension! {
    dimension Temperature {
        base_unit: Kelvin = 1.0,
        units: {
            Kelvin = 1.0,
        },
        symbols: {
            Kelvin = "K",
        }
    }
}

// Define Energy units
define_unit_dimension! {
    dimension Energy {
        base_unit: Joule = 1.0,
        units: {
            Joule = 1.0,
            Erg = JOULES_PER_ERG,
            ElectronVolt = JOULES_PER_EV,
        },
        symbols: {
            Joule = "J",
            Erg = "erg",
            ElectronVolt = "eV",
        }
    }
}

// Define Power units
define_unit_dimension! {
    dimension Power {
        base_unit: Watt = 1.0,
        units: {
            Watt = 1.0,
            SolarLuminosity = WATTS_PER_SOLAR_LUMINOSITY,
        },
        symbols: {
            Watt = "W",
            SolarLuminosity = "L☉",
        }
    }
}

// Define Angle units (dimensionless but physically important)
define_unit_dimension! {
    dimension Angle {
        base_unit: Radian = 1.0,
        units: {
            Radian = 1.0,
            Degree = RADIANS_PER_DEGREE,
        },
        symbols: {
            Radian = "rad",
            Degree = "°",
        }
    }
}

// Define AngularVelocity units (angle/time)
define_unit_dimension! {
    dimension AngularVelocity {
        base_unit: RadianPerSecond = 1.0,
        units: {
            RadianPerSecond = 1.0,
            DegreePerSecond = RADIANS_PER_DEGREE,
        },
        symbols: {
            RadianPerSecond = "rad/s",
            DegreePerSecond = "°/s",
        }
    }
}

// Define AngularAcceleration units (angle/time²)
define_unit_dimension! {
    dimension AngularAcceleration {
        base_unit: RadianPerSecondSquared = 1.0,
        units: {
            RadianPerSecondSquared = 1.0,
            DegreePerSecondSquared = RADIANS_PER_DEGREE,
        },
        symbols: {
            RadianPerSecondSquared = "rad/s²",
            DegreePerSecondSquared = "°/s²",
        }
    }
}

// Define Area units (Length²)
define_unit_dimension! {
    dimension Area {
        base_unit: SquareMeter = 1.0,
        units: {
            SquareMeter = 1.0,
            SquareKilometer = 1_000_000.0,
        },
        symbols: {
            SquareMeter = "m²",
            SquareKilometer = "km²",
        }
    }
}

// Define Volume units (Length³)
define_unit_dimension! {
    dimension Volume {
        base_unit: CubicMeter = 1.0,
        units: {
            CubicMeter = 1.0,
            Liter = 0.001,
        },
        symbols: {
            CubicMeter = "m³",
            Liter = "L",
        }
    }
}

// Define Velocity units (Length/Time)
define_unit_dimension! {
    dimension Velocity {
        base_unit: MeterPerSecond = 1.0,
        units: {
            MeterPerSecond = 1.0,
            KilometerPerHour = 1000.0 / 3600.0,
        },
        symbols: {
            MeterPerSecond = "m/s",
            KilometerPerHour = "km/h",
        }
    }
}

// Define Acceleration units (Length/Time²)
define_unit_dimension! {
    dimension Acceleration {
        base_unit: MeterPerSecondSquared = 1.0,
        units: {
            MeterPerSecondSquared = 1.0,
            StandardGravity = 9.80665,
        },
        symbols: {
            MeterPerSecondSquared = "m/s²",
            StandardGravity = "g₀",
        }
    }
}

// Define Force units (Mass×Length/Time²)
define_unit_dimension! {
    dimension Force {
        base_unit: Newton = 1.0,
        units: {
            Newton = 1.0,
        },
        symbols: {
            Newton = "N",
        }
    }
}

// Define Pressure units (Mass/(Length×Time²))
define_unit_dimension! {
    dimension Pressure {
        base_unit: Pascal = 1.0,
        units: {
            Pascal = 1.0,
            Bar = 100_000.0,
        },
        symbols: {
            Pascal = "Pa",
            Bar = "bar",
        }
    }
}

// Define Density units (Mass/Length³)
define_unit_dimension! {
    dimension Density {
        base_unit: KilogramPerCubicMeter = 1.0,
        units: {
            KilogramPerCubicMeter = 1.0,
            GramPerCubicCentimeter = 1000.0,
        },
        symbols: {
            KilogramPerCubicMeter = "kg/m³",
            GramPerCubicCentimeter = "g/cm³",
        }
    }
}

// Define Frequency units (1/Time)
define_unit_dimension! {
    dimension Frequency {
        base_unit: Hertz = 1.0,
        units: {
            Hertz = 1.0,
        },
        symbols: {
            Hertz = "Hz",
        }
    }
}

// Define Momentum units (Mass×Length/Time)
define_unit_dimension! {
    dimension Momentum {
        base_unit: KilogramMeterPerSecond = 1.0,
        units: {
            KilogramMeterPerSecond = 1.0,
        },
        symbols: {
            KilogramMeterPerSecond = "kg⋅m/s",
        }
    }
}

// Convenience type aliases for common combinations
pub type Newton_OLD = Force<Kilogram>; // Actually Force in SI base units  
pub type Pascal_OLD = Pressure<Kilogram>; // Actually Pressure in SI base units

// Common unit operations using helper functions
// Distance / Time = Velocity (simplified - returns value in SI units)
pub fn calculate_velocity(distance: Distance<Meter>, time: Time<Second>) -> f64 {
    divide_quantities(distance, time)
}
