//! Mass units for stellar system calculations.
//!
//! This module provides mass units spanning from laboratory scales to stellar masses,
//! with a focus on astronomical applications and stellar system modeling.
//!
//! # Available Units
//!
//! ## Standard Units
//! - **Gram** (`g`) - Base unit (1/1000 of SI kilogram for cleaner prefix handling)
//! - **Kilogram** (`kg`) - SI base unit via prefix system
//!
//! ## Astronomical Mass Units
//! - **EarthMass** (`M⊕`) - Mass of Earth (≈5.972 × 10²⁴ kg)
//! - **SolarMass** (`M☉`) - Mass of the Sun (≈1.989 × 10³⁰ kg)
//!
//! # Design Note
//!
//! This system uses **Gram** as the base unit instead of Kilogram to maintain
//! consistency with the prefix system. Kilogram is available as `Prefixed<Kilo, Gram>`.
//!
//! # Examples
//!
//! ```rust,no_run
//! use star_sim::physics::units::*;
//!
//! // Stellar masses
//! let sun_mass = Mass::<SolarMass>::new(1.0);
//! let red_dwarf = Mass::<SolarMass>::new(0.3);
//! let massive_star = Mass::<SolarMass>::new(25.0);
//!
//! // Planetary masses
//! let earth_mass = Mass::<EarthMass>::new(1.0);
//! let mars_mass = Mass::<EarthMass>::new(0.107); // Mars ≈ 0.107 Earth masses
//! let jupiter_mass = Mass::<EarthMass>::new(317.8); // Jupiter ≈ 317.8 Earth masses
//!
//! // Convert between astronomical and SI units
//! let sun_kg = sun_mass.convert_to::<Kilogram>();
//! let earth_kg = earth_mass.convert_to::<Kilogram>();
//!
//! println!("Sun: {}", sun_mass);     // "1 M☉"
//! println!("Earth: {}", earth_mass); // "1 M⊕"
//! ```
//!
//! # Conversion Hierarchy
//!
//! All conversions use kilograms (via gram × 1000) as the hub unit:
//! - Other units → Kilograms → Target unit
//! - Maintains astronomical precision while supporting SI compatibility

use crate::physics::units::{constants::*, core::*, prefix::*};
use crate::{define_quantity, define_unit_dimension};

/// Physical quantity representing mass with dimensional type safety.
///
/// Dimensional analysis: `[Mass¹]` - pure mass dimension
define_quantity!(Mass, 0, 1, 0, 0, 0, 0, 0);

// Define Mass units with astronomical focus
// Note: Using Gram as base unit to avoid confusion with prefix system
// Kilogram will be available as Prefixed<Kilo, Gram>
define_unit_dimension! {
    dimension Mass {
        base_unit: Gram = KG_PER_GRAM,
        units: {
            Gram = KG_PER_GRAM,
            EarthMass = KG_PER_EARTH_MASS,
            SolarMass = KG_PER_SOLAR_MASS,
        },
        symbols: {
            Gram = "g",
            EarthMass = "M⊕",
            SolarMass = "M☉",
        }
    }
}

// ================================================================================================
// CONVENIENCE TYPE ALIASES FOR COMMON PREFIXED UNITS
// ================================================================================================

// Mass prefixes (Gram is now the base unit, so Kilogram is a proper prefix)
pub type Kilogram = Prefixed<Kilo, Gram>;
