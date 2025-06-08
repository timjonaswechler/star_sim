/// Gravitationskonstante (m³ kg⁻¹ s⁻²)
pub const G: f64 = 6.67430e-11;

/// Lichtgeschwindigkeit in m/s
pub const SPEED_OF_LIGHT: f64 = 2.99792458e8;

/// Stefan-Boltzmann Konstante (W m⁻² K⁻⁴)
pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;

/// Planck-Konstante (J⋅s)
pub const PLANCK_CONSTANT: f64 = 6.62607015e-34;

/// Boltzmann-Konstante (J/K)
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23;

/// Mathematische Konstanten
pub const PI: f64 = std::f64::consts::PI;
pub const TAU: f64 = 2.0 * PI;

/// Umwandlung von Grad zu Radiant
pub const DEG_TO_RAD: f64 = PI / 180.0;

/// Umwandlung von Radiant zu Grad
pub const RAD_TO_DEG: f64 = 180.0 / PI;

pub const SECONDS_PER_MINUTE: f64 = 60.0;
pub const MINUTES_PER_SECOND: f64 = 1.0 / SECONDS_PER_MINUTE;

pub const MINUTES_PER_HOUR: f64 = 60.0;
pub const HOURS_PER_MINUTE: f64 = 1.0 / MINUTES_PER_HOUR;

pub const HOURS_PER_DAY: f64 = 24.0;
pub const DAYS_PER_HOUR: f64 = 1.0 / HOURS_PER_DAY;

pub const DAYS_PER_YEAR_ASTRO: f64 = 365.25; // Julianisches Jahr
pub const SECONDS_PER_HOUR: f64 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR; // 3600.0 Sekunden pro Stunde
pub const SECONDS_PER_DAY: f64 = SECONDS_PER_HOUR * HOURS_PER_DAY; // 86400.0 Sekunden pro Tag
pub const SECONDS_PER_YEAR: f64 = SECONDS_PER_DAY * DAYS_PER_YEAR_ASTRO; // 31_557_600.0 Sekunden pro Jahr

pub const YEARS_PER_SECONDS: f64 = 1.0 / SECONDS_PER_YEAR; // 31_557_600.0 Sekunden pro Jahr    
pub const DAYS_PER_SECONDS: f64 = 1.0 / SECONDS_PER_DAY; // 86_400.0 Sekunden pro Tag
pub const HOURS_PER_SECONDS: f64 = 1.0 / SECONDS_PER_HOUR; // 3600.0 Sekunden pro Stunde
pub const MINUTES_PER_SECONDS: f64 = 1.0 / SECONDS_PER_MINUTE; // 60.0 Sekunden pro Minute

pub const KM_TO_M: f64 = 1000.0; // Kilometer zu Meter
pub const M_TO_KM: f64 = 0.001; // Meter zu Kilometer

pub const G_TO_KG: f64 = 0.001; // Gramm zu Kilogramm
pub const KG_TO_G: f64 = 1000.0; // Kilogramm zu Gramm
