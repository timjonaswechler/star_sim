/// Astronomische Einheit in Metern
pub const AU_IN_METERS: f64 = 1.4959787e11;

/// Sonnenmasse in Kilogramm
pub const SOLAR_MASS_IN_KG: f64 = 1.98847e30;

/// Sonnenradius in Metern
pub const SOLAR_RADIUS_IN_METERS: f64 = 6.957e8;
/// Alias für einfacheren Zugriff
pub const SOLAR_RADIUS: f64 = SOLAR_RADIUS_IN_METERS;

/// Sonnenleuchtkraft in Watt
pub const SOLAR_LUMINOSITY_IN_WATTS: f64 = 3.828e26;
pub const SOLAR_LUMINOSITY: f64 = SOLAR_LUMINOSITY_IN_WATTS;

/// Effektivtemperatur der Sonne in Kelvin
pub const SOLAR_TEMPERATURE_IN_KELVIN: f64 = 5778.0;
pub const SOLAR_TEMPERATURE: f64 = SOLAR_TEMPERATURE_IN_KELVIN;

/// Parsec in Metern
pub const PARSEC_IN_METERS: f64 = 3.0857e16;

/// Kiloparsec in Metern
pub const KILOPARSEC_IN_METERS: f64 = PARSEC_IN_METERS * 1000.0;

/// Minimum Massenverhältnis für stabile L4/L5 Lagrange-Punkte
/// Aus dem Artikel: Stern muss mindestens 24.96 mal schwerer sein als Planet
pub const MIN_LAGRANGE_MASS_RATIO: f64 = 24.96;

/// Standard Epoch für astronomische Berechnungen (J2000.0)
/// 12:00:00 TT on 1 January 2000 = JD 2451545.0
pub const J2000_EPOCH: f64 = 2451545.0;

/// Standard Gravitationsparameter für die Sonne (GM☉) in m³/s²
pub const SOLAR_MU: f64 = super::common::G * super::SOLAR_MASS_IN_KG;

pub const AU_TO_M: f64 = 1.4959787e11;
pub const M_TO_AU: f64 = 1.0 / AU_TO_M;

pub const SOLAR_MASS_TO_KG: f64 = 1.98847e30;
pub const KG_TO_SOLAR_MASS: f64 = 1.0 / SOLAR_MASS_TO_KG;

pub const YEARS_PER_KILOYEAR: f64 = 1_000.0;
pub const KILOYEARS_PER_YEAR: f64 = 1.0 / YEARS_PER_KILOYEAR;

pub const KILOYEARS_PER_MEGAYEAR: f64 = 1_000.0;
pub const MEGAYEARS_PER_KILOYEAR: f64 = 1.0 / KILOYEARS_PER_MEGAYEAR;

pub const MEGAYEARS_PER_GIGAYEAR: f64 = 1_000.0;
pub const GIGAYEARS_PER_MEGAYEAR: f64 = 1.0 / MEGAYEARS_PER_GIGAYEAR;

/// Erdmassen in Kilogramm
pub const EARTH_MASS_IN_KG: f64 = 5.9722e24;
/// Erdradius in Metern
pub const EARTH_RADIUS_IN_METERS: f64 = 6.371e6;

pub const KG_TO_EARTH_MASS: f64 = 1.0 / EARTH_MASS_IN_KG;
pub const EARTH_MASS_TO_KG: f64 = EARTH_MASS_IN_KG;

pub const METERS_TO_EARTH_RADIUS: f64 = 1.0 / EARTH_RADIUS_IN_METERS;
pub const EARTH_RADIUS_TO_METERS: f64 = EARTH_RADIUS_IN_METERS;
