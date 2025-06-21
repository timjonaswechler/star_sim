//! Physical constants for unit conversions.
//!
//! This module centralizes all physical constants used for converting between different
//! units and SI base units. All values are exact conversion factors to SI base units.
//!
//! # Design Philosophy
//!
//! By centralizing constants in one location, we:
//! - Ensure consistency across all unit conversions
//! - Make it easy to update values when more precise measurements become available
//! - Provide a single source of truth for physical constants
//! - Enable easy auditing of the constants used in the system
//!
//! # Precision
//!
//! Constants are taken from authoritative sources and represent the best available
//! precision as of the implementation date. Key sources include:
//! - IAU (International Astronomical Union) for astronomical constants
//! - NIST (National Institute of Standards and Technology) for physical constants
//! - SI (International System of Units) definitions

// ================================================================================================
// DISTANCE CONVERSIONS (to meters)
// ================================================================================================

/// Astronomical Unit (AU) to meters.
///
/// Exact value as defined by the IAU in 2012. The AU is the average distance
/// from Earth to the Sun and is fundamental for astronomical distance measurements.
pub const METERS_PER_AU: f64 = 149_597_870_700.0;

/// Earth radius to meters.
///
/// Volumetric mean radius as defined by the IAU. This is commonly used for
/// expressing the size of terrestrial exoplanets.
pub const METERS_PER_EARTH_RADIUS: f64 = 6_371_000.0;

/// Solar radius to meters.
///
/// Standard solar radius as defined by the IAU. Essential for expressing stellar
/// radii in solar units, the most common way to describe star sizes.
pub const METERS_PER_SUN_RADIUS: f64 = 696_340_000.0;

/// Light year to meters.
///
/// Distance light travels in one Julian year (365.25 days). Used for
/// interstellar distance measurements.
pub const METERS_PER_LIGHT_YEAR: f64 = 9_460_730_472_580_800.0;

/// Parsec to meters.
///
/// The distance at which one AU subtends one arcsecond. Fundamental unit
/// for astronomical distance measurements, especially in galactic astronomy.
pub const METERS_PER_PARSEC: f64 = 3.085677581491367e16;

// ================================================================================================
// MASS CONVERSIONS (to kilograms)
// ================================================================================================

/// Gram to kilograms.
///
/// Basic metric conversion factor.
pub const KG_PER_GRAM: f64 = 0.001;

/// Earth mass to kilograms.
///
/// Standard Earth mass as defined by the IAU. Used for expressing the mass of
/// terrestrial exoplanets and rocky bodies.
pub const KG_PER_EARTH_MASS: f64 = 5.972e24;

/// Solar mass to kilograms.
///
/// Standard solar mass as defined by the IAU. The fundamental unit for expressing
/// stellar masses and is used throughout astrophysics.
pub const KG_PER_SOLAR_MASS: f64 = 1.989e30;

// ================================================================================================
// TIME CONVERSIONS (to seconds)
// ================================================================================================

/// Minute to seconds.
pub const SECONDS_PER_MINUTE: f64 = 60.0;

/// Hour to seconds.
pub const SECONDS_PER_HOUR: f64 = 3600.0;

/// Day to seconds.
pub const SECONDS_PER_DAY: f64 = 86400.0;

/// Julian year to seconds.
///
/// Exactly 365.25 days. This is the standard year used in astronomy for
/// consistency across calculations involving orbital periods and stellar evolution.
pub const SECONDS_PER_YEAR: f64 = 31_557_600.0;

/// Gigayear (billion years) to seconds.
///
/// Used for expressing long astronomical timescales like stellar evolution,
/// galactic dynamics, and cosmological processes.
pub const SECONDS_PER_GIGAYEAR: f64 = SECONDS_PER_YEAR * 1e9;

// ================================================================================================
// TEMPERATURE CONVERSIONS
// ================================================================================================

/// Celsius to Kelvin offset.
///
/// Note: This is for additive conversions (°C = K - 273.15).
/// Multiplicative temperature conversions use scale factors of 1.0.
pub const CELSIUS_OFFSET: f64 = 273.15;

// ================================================================================================
// ENERGY CONVERSIONS (to Joules)
// ================================================================================================

/// Erg to Joules.
///
/// CGS unit of energy, still commonly used in astrophysics.
pub const JOULES_PER_ERG: f64 = 1e-7;

/// Electron volt to Joules.
///
/// Fundamental energy unit in atomic and particle physics. The 2019 exact value
/// following the redefinition of SI base units.
pub const JOULES_PER_EV: f64 = 1.602176634e-19;

// ================================================================================================
// POWER CONVERSIONS (to Watts)
// ================================================================================================

/// Solar luminosity to Watts.
///
/// Standard solar luminosity as defined by the IAU. Used for expressing the
/// power output of stars and is fundamental to stellar astrophysics.
pub const WATTS_PER_SOLAR_LUMINOSITY: f64 = 3.828e26;

// ================================================================================================
// ANGLE CONVERSIONS (to radians - dimensionless but important)
// ================================================================================================

/// Degrees to radians.
///
/// Fundamental angular conversion. π radians = 180 degrees.
pub const RADIANS_PER_DEGREE: f64 = std::f64::consts::PI / 180.0;

// ================================================================================================
// ADDITIONAL TIME CONVERSIONS
// ================================================================================================

/// Megayear (million years) to seconds.
///
/// Used for intermediate astronomical timescales, particularly in stellar evolution
/// and galactic processes.
pub const SECONDS_PER_MEGAYEAR: f64 = SECONDS_PER_YEAR * 1e6;

// ================================================================================================
// ADDITIONAL DISTANCE CONVERSIONS
// ================================================================================================

/// Kiloparsec to meters.
///
/// 1000 parsecs. Used for galactic-scale distance measurements, particularly
/// in describing the structure and size of galaxies.
pub const METERS_PER_KILOPARSEC: f64 = METERS_PER_PARSEC * 1000.0;

// ================================================================================================
// TEMPERATURE CONVERSIONS
// ================================================================================================

/// Fahrenheit to Celsius conversion factor.
///
/// °C = (°F - 32) × 5/9, so this is the multiplicative factor 5/9.
pub const CELSIUS_PER_FAHRENHEIT: f64 = 5.0 / 9.0;

/// Fahrenheit to Celsius offset.
///
/// The additive offset: °C = (°F - 32) × 5/9.
pub const FAHRENHEIT_OFFSET: f64 = 32.0;
