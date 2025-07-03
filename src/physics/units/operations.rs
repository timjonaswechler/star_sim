//! Extended unit-aware mathematical operations.
//!
//! This module provides advanced mathematical operations that respect dimensional analysis
//! and return appropriately typed results. Includes automatic dimensional analysis for
//! creating proper composite units with prefix support.

use crate::physics::units::constants::*;

// ================================================================================================
// ORBITAL MECHANICS OPERATIONS
// ================================================================================================

/// Calculate orbital velocity from gravitational parameter and distance (returns m/s).
/// v = √(GM/r) for circular orbits
pub fn orbital_velocity_si(gm_m3_s2: f64, radius_m: f64) -> f64 {
    (gm_m3_s2 / radius_m).sqrt()
}

/// Calculate orbital period from gravitational parameter and semi-major axis (returns seconds).
/// T = 2π√(a³/GM) (Kepler's third law)
pub fn orbital_period_si(gm_m3_s2: f64, semi_major_axis_m: f64) -> f64 {
    2.0 * std::f64::consts::PI * (semi_major_axis_m.powi(3) / gm_m3_s2).sqrt()
}

/// Calculate escape velocity from gravitational parameter and radius (returns m/s).
/// v_esc = √(2GM/r)
pub fn escape_velocity_si(gm_m3_s2: f64, radius_m: f64) -> f64 {
    (2.0 * gm_m3_s2 / radius_m).sqrt()
}

/// Calculate Hill sphere radius (returns meters).
/// r_Hill = a * (m/(3M))^(1/3)
pub fn hill_sphere_radius_si(
    semi_major_axis_m: f64,
    satellite_mass_kg: f64,
    primary_mass_kg: f64,
) -> f64 {
    semi_major_axis_m * (satellite_mass_kg / (3.0 * primary_mass_kg)).powf(1.0 / 3.0)
}

// ================================================================================================
// STELLAR PHYSICS OPERATIONS
// ================================================================================================

/// Calculate stellar luminosity using Stefan-Boltzmann law (returns watts).
/// L = 4πR²σT⁴
pub fn stellar_luminosity_si(radius_m: f64, temperature_k: f64) -> f64 {
    const STEFAN_BOLTZMANN: f64 = 5.670374419e-8; // W/(m²⋅K⁴)
    4.0 * std::f64::consts::PI * radius_m.powi(2) * STEFAN_BOLTZMANN * temperature_k.powi(4)
}

/// Calculate effective temperature from luminosity and radius (returns Kelvin).
/// T_eff = (L/(4πR²σ))^(1/4)
pub fn effective_temperature_si(luminosity_w: f64, radius_m: f64) -> f64 {
    const STEFAN_BOLTZMANN: f64 = 5.670374419e-8; // W/(m²⋅K⁴)
    (luminosity_w / (4.0 * std::f64::consts::PI * radius_m.powi(2) * STEFAN_BOLTZMANN)).powf(0.25)
}

/// Calculate surface gravity from mass and radius (returns m/s²).
/// g = GM/R²
pub fn surface_gravity_si(mass_kg: f64, radius_m: f64) -> f64 {
    GRAVITATIONAL_CONSTANT * mass_kg / radius_m.powi(2)
}

/// Calculate stellar density from mass and radius (returns kg/m³).
/// ρ = M / (4/3 * π * R³)
pub fn stellar_density_si(mass_kg: f64, radius_m: f64) -> f64 {
    let volume = (4.0 / 3.0) * std::f64::consts::PI * radius_m.powi(3);
    mass_kg / volume
}

// ================================================================================================
// SPECTRAL OPERATIONS
// ================================================================================================

/// Calculate Wien displacement law - peak wavelength from temperature (returns meters).
/// λ_max = b / T, where b is Wien's displacement constant
pub fn wien_peak_wavelength_si(temperature_k: f64) -> f64 {
    const WIEN_DISPLACEMENT_CONSTANT: f64 = 2.897771955e-3; // m⋅K
    WIEN_DISPLACEMENT_CONSTANT / temperature_k
}

/// Calculate frequency from wavelength (returns Hz).
/// f = c / λ
pub fn wavelength_to_frequency_si(wavelength_m: f64) -> f64 {
    SPEED_OF_LIGHT / wavelength_m
}

/// Calculate wavelength from frequency (returns meters).
/// λ = c / f
pub fn frequency_to_wavelength_si(frequency_hz: f64) -> f64 {
    SPEED_OF_LIGHT / frequency_hz
}

/// Calculate photon energy from wavelength (returns Joules).
/// E = hc/λ
pub fn photon_energy_from_wavelength_si(wavelength_m: f64) -> f64 {
    PLANCK_CONSTANT * SPEED_OF_LIGHT / wavelength_m
}

/// Calculate photon energy from frequency (returns Joules).
/// E = hf
pub fn photon_energy_from_frequency_si(frequency_hz: f64) -> f64 {
    PLANCK_CONSTANT * frequency_hz
}

// ================================================================================================
// THERMODYNAMIC OPERATIONS
// ================================================================================================

/// Calculate thermal energy per particle (returns Joules).
/// E_thermal = (3/2) * k_B * T for monatomic gas
pub fn thermal_energy_per_particle_si(temperature_k: f64) -> f64 {
    const BOLTZMANN_CONSTANT: f64 = 1.380649e-23; // J/K
    1.5 * BOLTZMANN_CONSTANT * temperature_k
}

/// Calculate sound speed in ideal gas (returns m/s).
/// c_s = √(γ * k_B * T / m)
pub fn sound_speed_si(temperature_k: f64, gamma: f64, particle_mass_kg: f64) -> f64 {
    const BOLTZMANN_CONSTANT: f64 = 1.380649e-23; // J/K
    (gamma * BOLTZMANN_CONSTANT * temperature_k / particle_mass_kg).sqrt()
}

/// Calculate pressure from density and temperature for ideal gas (returns Pa).
/// P = (ρ/m) * k_B * T
pub fn ideal_gas_pressure_si(density_kg_m3: f64, temperature_k: f64, particle_mass_kg: f64) -> f64 {
    const BOLTZMANN_CONSTANT: f64 = 1.380649e-23; // J/K
    (density_kg_m3 / particle_mass_kg) * BOLTZMANN_CONSTANT * temperature_k
}

// ================================================================================================
// RELATIVISTIC OPERATIONS
// ================================================================================================

/// Calculate Lorentz factor γ from velocity (dimensionless).
/// γ = 1/√(1 - v²/c²)
pub fn lorentz_factor_si(velocity_ms: f64) -> f64 {
    let beta = velocity_ms / SPEED_OF_LIGHT;
    1.0 / (1.0 - beta.powi(2)).sqrt()
}

/// Calculate relativistic kinetic energy (returns Joules).
/// K = (γ - 1) * m * c²
pub fn relativistic_kinetic_energy_si(mass_kg: f64, velocity_ms: f64) -> f64 {
    let gamma = lorentz_factor_si(velocity_ms);
    (gamma - 1.0) * mass_kg * SPEED_OF_LIGHT.powi(2)
}

// ================================================================================================
// BLACKBODY RADIATION
// ================================================================================================

/// Calculate Planck function for blackbody radiation (returns W/(m²⋅sr⋅m)).
/// B(λ,T) = (2hc²/λ⁵) * 1/(exp(hc/λkT) - 1)
pub fn planck_function_si(wavelength_m: f64, temperature_k: f64) -> f64 {
    const BOLTZMANN_CONSTANT: f64 = 1.380649e-23; // J/K

    let coeff = 2.0 * PLANCK_CONSTANT * SPEED_OF_LIGHT.powi(2) / wavelength_m.powi(5);
    let exponent =
        PLANCK_CONSTANT * SPEED_OF_LIGHT / (wavelength_m * BOLTZMANN_CONSTANT * temperature_k);

    coeff / (exponent.exp() - 1.0)
}

/// Calculate Stefan-Boltzmann total radiated power (returns W/m²).
/// j = σT⁴
pub fn stefan_boltzmann_flux_si(temperature_k: f64) -> f64 {
    const STEFAN_BOLTZMANN: f64 = 5.670374419e-8; // W/(m²⋅K⁴)
    STEFAN_BOLTZMANN * temperature_k.powi(4)
}
