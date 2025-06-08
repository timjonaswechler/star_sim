use crate::celestial_objects::stellar_properties::StellarProperties;
use serde::{Deserialize, Serialize};

/// Typ des Sternsystems (aus original main.rs erweitert)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    Single(StellarProperties),
    Binary {
        primary: StellarProperties,
        secondary: StellarProperties,
        orbital_properties: BinaryOrbit,
    },
    Multiple {
        components: Vec<StellarProperties>,
        hierarchy: SystemHierarchy,
    },
}
