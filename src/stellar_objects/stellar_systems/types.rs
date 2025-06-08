use crate::stellar_objects::stars::properties::StellarProperties;
use crate::physics::astrophysics::orbit::two_body::BinaryOrbit;
use crate::stellar_objects::stellar_systems::hierarchy::SystemHierarchy;
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
