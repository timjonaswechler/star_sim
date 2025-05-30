// units.rs - Einheitensystem für astronomische und SI-Einheiten

use crate::constants::conversion::*;
use serde::{Deserialize, Serialize};

/// Einheitensystem für Berechnungen
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnitSystem {
    /// Astronomische Einheiten (AU, Jahre, Sonnenmassen)
    Astronomical,
    /// SI-Einheiten (m, s, kg)
    SI,
}

/// Struktur für Einheiten-bewusste Werte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Units<T> {
    pub value: T,
    pub system: UnitSystem,
}

impl<T> Units<T> {
    pub fn new(value: T, system: UnitSystem) -> Self {
        Self { value, system }
    }

    pub fn astronomical(value: T) -> Self {
        Self::new(value, UnitSystem::Astronomical)
    }

    pub fn si(value: T) -> Self {
        Self::new(value, UnitSystem::SI)
    }
}

/// Trait für Einheitenkonvertierung
pub trait UnitConversion {
    /// Konvertiert von astronomischen zu SI-Einheiten
    fn to_si(&self) -> Self;

    /// Konvertiert von SI zu astronomischen Einheiten
    fn to_astronomical(&self) -> Self;

    /// Konvertiert zu einem spezifischen Einheitensystem
    fn to_system(&self, target: UnitSystem) -> Self;
}

/// Distanz mit Einheiten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    pub value: f64,
    pub system: UnitSystem,
}

impl Distance {
    pub fn new(value: f64, system: UnitSystem) -> Self {
        Self { value, system }
    }
    pub fn au(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::Astronomical,
        }
    }

    pub fn meters(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::SI,
        }
    }

    pub fn kilometers(value: f64) -> Self {
        Self {
            value: value * KM_TO_M,
            system: UnitSystem::SI,
        }
    }

    /// Gibt den Wert in Metern zurück
    pub fn in_meters(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value,
            UnitSystem::Astronomical => self.value * AU_TO_M,
        }
    }

    /// Gibt den Wert in AU zurück
    pub fn in_au(&self) -> f64 {
        match self.system {
            UnitSystem::Astronomical => self.value,
            UnitSystem::SI => self.value / AU_TO_M,
        }
    }
}

impl UnitConversion for Distance {
    fn to_si(&self) -> Self {
        match self.system {
            UnitSystem::SI => self.clone(),
            UnitSystem::Astronomical => Self::meters(self.value * AU_TO_M),
        }
    }

    fn to_astronomical(&self) -> Self {
        match self.system {
            UnitSystem::Astronomical => self.clone(),
            UnitSystem::SI => Self::au(self.value / AU_TO_M),
        }
    }

    fn to_system(&self, target: UnitSystem) -> Self {
        match target {
            UnitSystem::SI => self.to_si(),
            UnitSystem::Astronomical => self.to_astronomical(),
        }
    }
}

/// Zeit mit Einheiten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub value: f64,
    pub system: UnitSystem,
}

impl Time {
    pub fn new(value: f64, system: UnitSystem) -> Self {
        Self { value, system }
    }
    pub fn years(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::Astronomical,
        }
    }

    pub fn seconds(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::SI,
        }
    }

    pub fn days(value: f64) -> Self {
        Self {
            value: value * 24.0 * 3600.0,
            system: UnitSystem::SI,
        }
    }

    /// Gibt den Wert in Sekunden zurück
    pub fn in_seconds(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value,
            UnitSystem::Astronomical => self.value * YEARS_TO_SECONDS,
        }
    }

    /// Gibt den Wert in Jahren zurück
    pub fn in_years(&self) -> f64 {
        match self.system {
            UnitSystem::Astronomical => self.value,
            UnitSystem::SI => self.value / YEARS_TO_SECONDS,
        }
    }
}

impl UnitConversion for Time {
    fn to_si(&self) -> Self {
        match self.system {
            UnitSystem::SI => self.clone(),
            UnitSystem::Astronomical => Self::seconds(self.value * YEARS_TO_SECONDS),
        }
    }

    fn to_astronomical(&self) -> Self {
        match self.system {
            UnitSystem::Astronomical => self.clone(),
            UnitSystem::SI => Self::years(self.value / YEARS_TO_SECONDS),
        }
    }

    fn to_system(&self, target: UnitSystem) -> Self {
        match target {
            UnitSystem::SI => self.to_si(),
            UnitSystem::Astronomical => self.to_astronomical(),
        }
    }
}

/// Masse mit Einheiten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mass {
    pub value: f64,
    pub system: UnitSystem,
}

impl Mass {
    pub fn new(value: f64, system: UnitSystem) -> Self {
        Self { value, system }
    }
    pub fn solar_masses(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::Astronomical,
        }
    }

    pub fn kilograms(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::SI,
        }
    }

    /// Gibt den Wert in Kilogramm zurück
    pub fn in_kg(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value,
            UnitSystem::Astronomical => self.value * MSUN_TO_KG,
        }
    }

    /// Gibt den Wert in Sonnenmassen zurück
    pub fn in_solar_masses(&self) -> f64 {
        match self.system {
            UnitSystem::Astronomical => self.value,
            UnitSystem::SI => self.value / MSUN_TO_KG,
        }
    }
}

impl UnitConversion for Mass {
    fn to_si(&self) -> Self {
        match self.system {
            UnitSystem::SI => self.clone(),
            UnitSystem::Astronomical => Self::kilograms(self.value * MSUN_TO_KG),
        }
    }

    fn to_astronomical(&self) -> Self {
        match self.system {
            UnitSystem::Astronomical => self.clone(),
            UnitSystem::SI => Self::solar_masses(self.value / MSUN_TO_KG),
        }
    }

    fn to_system(&self, target: UnitSystem) -> Self {
        match target {
            UnitSystem::SI => self.to_si(),
            UnitSystem::Astronomical => self.to_astronomical(),
        }
    }
}

/// Geschwindigkeit mit Einheiten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Velocity {
    pub value: f64,
    pub system: UnitSystem,
}

impl Velocity {
    /// AU pro Jahr
    pub fn au_per_year(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::Astronomical,
        }
    }

    /// Meter pro Sekunde
    pub fn meters_per_second(value: f64) -> Self {
        Self {
            value,
            system: UnitSystem::SI,
        }
    }

    /// Kilometer pro Sekunde
    pub fn km_per_second(value: f64) -> Self {
        Self {
            value: value * KMS_TO_MS,
            system: UnitSystem::SI,
        }
    }

    /// Gibt den Wert in m/s zurück
    pub fn in_ms(&self) -> f64 {
        match self.system {
            UnitSystem::SI => self.value,
            UnitSystem::Astronomical => {
                // AU/Jahr zu m/s: (AU/Jahr) * (m/AU) / (s/Jahr)
                self.value * AU_TO_M / YEARS_TO_SECONDS
            }
        }
    }

    /// Gibt den Wert in km/s zurück
    pub fn in_kms(&self) -> f64 {
        self.in_ms() / KMS_TO_MS
    }

    /// Gibt den Wert in AU/Jahr zurück
    pub fn in_au_per_year(&self) -> f64 {
        match self.system {
            UnitSystem::Astronomical => self.value,
            UnitSystem::SI => {
                // m/s zu AU/Jahr: (m/s) * (s/Jahr) / (m/AU)
                self.value * YEARS_TO_SECONDS / AU_TO_M
            }
        }
    }
}

impl UnitConversion for Velocity {
    fn to_si(&self) -> Self {
        match self.system {
            UnitSystem::SI => self.clone(),
            UnitSystem::Astronomical => Self::meters_per_second(self.in_ms()),
        }
    }

    fn to_astronomical(&self) -> Self {
        match self.system {
            UnitSystem::Astronomical => self.clone(),
            UnitSystem::SI => Self::au_per_year(self.in_au_per_year()),
        }
    }

    fn to_system(&self, target: UnitSystem) -> Self {
        match target {
            UnitSystem::SI => self.to_si(),
            UnitSystem::Astronomical => self.to_astronomical(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::PI;
    use crate::constants::{AU_TO_METERS, SECONDS_PER_YEAR, SOLAR_MASS};

    #[test]
    fn test_distance_conversion() {
        let dist_au = Distance::au(1.0);
        let dist_m = dist_au.to_si();
        assert!((dist_m.value - AU_TO_METERS).abs() < 1e-6 * AU_TO_METERS); // Relative Toleranz

        let back_to_au = dist_m.to_astronomical();
        assert!((back_to_au.value - 1.0).abs() < 1e-10);

        let dist_km = Distance::kilometers(1000.0); // kilometers verwenden
        assert!((dist_km.value - 1_000_000.0).abs() < 1e-6); // 1000 km = 1e6 m
        assert_eq!(dist_km.system, UnitSystem::SI);
    }

    #[test]
    fn test_time_conversion() {
        let time_years = Time::years(1.0);
        let time_seconds = time_years.to_si();
        assert!((time_seconds.value - SECONDS_PER_YEAR).abs() < 1.0);

        let back_to_years = time_seconds.to_astronomical();
        assert!((back_to_years.value - 1.0).abs() < 1e-10);

        let time_days = Time::days(365.25); // days verwenden
        assert!((time_days.in_seconds() - SECONDS_PER_YEAR).abs() < 1.0); // in_seconds verwenden
        assert_eq!(time_days.system, UnitSystem::SI);
    }

    #[test]
    fn test_mass_conversion() {
        let mass_solar = Mass::solar_masses(1.0);
        let mass_kg = mass_solar.to_si();

        assert!((mass_kg.value - SOLAR_MASS).abs() < 1e25); // Within reasonable precision

        let back_to_solar = mass_kg.to_astronomical();
        assert!((back_to_solar.value - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_velocity_conversion() {
        // Earth's orbital velocity is ~30 km/s
        let earth_orbital_kms = Velocity::km_per_second(29.78); // km_per_second verwenden
        assert_eq!(earth_orbital_kms.system, UnitSystem::SI);
        assert!((earth_orbital_kms.in_kms() - 29.78).abs() < 1e-6); // in_kms verwenden
        assert!((earth_orbital_kms.in_ms() - 29780.0).abs() < 1.0);

        let au_per_year = earth_orbital_kms.to_astronomical();
        // Sollte ungefähr 2π AU/Jahr sein (eigentlich 1 AU / (1/2π) Jahr, also ca. 6.28)
        // 1 Jahr = P, a = 1 AU. v = 2πa/P = 2π * 1AU / 1 Jahr = 2π AU/Jahr
        assert!(
            (au_per_year.value - 2.0 * PI).abs() < 0.1,
            "AU/yr: {}",
            au_per_year.value
        );
    }
}
