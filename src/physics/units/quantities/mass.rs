use crate::physics::units::{constants::*, core::*, prefix::*};
use crate::{define_quantity, define_unit_dimension};

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
