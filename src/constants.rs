// constants.rs - Physikalische und astronomische Konstanten

/// Standard Epoch für astronomische Berechnungen (J2000.0)
/// 12:00:00 TT on 1 January 2000 = JD 2451545.0
pub const J2000_EPOCH: f64 = 2451545.0;

/// Version des Sternsystem Generators
pub const VERSION: &str = "0.1.0";

/// Gravitationskonstante (m³ kg⁻¹ s⁻²)
pub const G: f64 = 6.67430e-11;

/// Astronomische Einheit in Metern
pub const AU_TO_METERS: f64 = 1.4959787e11;

/// Sonnenmasse in Kilogramm
pub const SOLAR_MASS: f64 = 1.98847e30;

/// Sonnenradius in Metern
pub const SOLAR_RADIUS: f64 = 6.957e8;

/// Sonnenleuchtkraft in Watt
pub const SOLAR_LUMINOSITY: f64 = 3.828e26;

/// Effektivtemperatur der Sonne in Kelvin
pub const SOLAR_TEMPERATURE: f64 = 5778.0;

/// Lichtgeschwindigkeit in m/s
pub const SPEED_OF_LIGHT: f64 = 2.99792458e8;

/// Stefan-Boltzmann Konstante (W m⁻² K⁻⁴)
pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;

/// Planck-Konstante (J⋅s)
pub const PLANCK_CONSTANT: f64 = 6.62607015e-34;

/// Boltzmann-Konstante (J/K)
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23;

/// Sekunden pro Jahr (Julianisches Jahr)
pub const SECONDS_PER_YEAR: f64 = 31557600.0;

/// Tage pro Jahr (Julianisches Jahr)
pub const DAYS_PER_YEAR: f64 = 365.25;

/// Parsec in Metern
pub const PARSEC_TO_METERS: f64 = 3.0857e16;

/// Kiloparsec in Metern
pub const KILOPARSEC_TO_METERS: f64 = PARSEC_TO_METERS * 1000.0;

/// Minimum Massenverhältnis für stabile L4/L5 Lagrange-Punkte
/// Aus dem Artikel: Stern muss mindestens 24.96 mal schwerer sein als Planet
pub const MIN_LAGRANGE_MASS_RATIO: f64 = 24.96;

/// Mathematische Konstanten
pub const PI: f64 = std::f64::consts::PI;
pub const TAU: f64 = 2.0 * PI;

/// Umwandlungsfaktoren
pub mod conversion {
    use super::*;

    /// Umwandlung von Grad zu Radiant
    pub const DEG_TO_RAD: f64 = PI / 180.0;

    /// Umwandlung von Radiant zu Grad
    pub const RAD_TO_DEG: f64 = 180.0 / PI;

    /// Jahre zu Sekunden
    pub const YEARS_TO_SECONDS: f64 = SECONDS_PER_YEAR;

    /// AU zu Metern
    pub const AU_TO_M: f64 = AU_TO_METERS;

    /// Sonnenmassen zu Kilogramm
    pub const MSUN_TO_KG: f64 = SOLAR_MASS;

    /// Kilometer zu Metern
    pub const KM_TO_M: f64 = 1000.0;

    /// Kilometer pro Sekunde zu Meter pro Sekunde
    pub const KMS_TO_MS: f64 = 1000.0;
}

/// Astronomische Standardwerte für Skalierung
pub mod standards {
    /// Standard Gravitationsparameter für die Sonne (GM☉) in m³/s²
    pub const SOLAR_MU: f64 = super::G * super::SOLAR_MASS;

    /// Charakteristische Geschwindigkeit für 1 AU Umlaufbahn um die Sonne
    /// v = √(GM☉/1AU) ≈ 29.8 km/s
    pub const SOLAR_ORBITAL_VELOCITY_1AU: f64 = 29784.0; // m/s

    /// Escape Velocity von der Sonnenoberfläche
    pub const SOLAR_ESCAPE_VELOCITY: f64 = 617500.0; // m/s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_constants() {
        // Gravitationskonstante sollte korrekt sein
        assert!((G - 6.67430e-11).abs() < 1e-16);

        // 1 AU sollte etwa 150 Million km sein
        assert!((AU_TO_METERS / 1e11 - 1.496).abs() < 0.001);

        // Lichtgeschwindigkeit exakt definiert
        assert_eq!(SPEED_OF_LIGHT, 2.99792458e8);
    }

    #[test]
    fn test_conversions() {
        use conversion::*;

        // 180° = π Radiant
        assert!((180.0 * DEG_TO_RAD - PI).abs() < 1e-10);

        // π Radiant = 180°
        assert!((PI * RAD_TO_DEG - 180.0).abs() < 1e-10);
    }

    #[test]
    fn test_lagrange_mass_ratio() {
        // Aus dem Artikel: Mindestens 24.96:1 für stabile L4/L5
        assert_eq!(MIN_LAGRANGE_MASS_RATIO, 24.96);
    }
}
