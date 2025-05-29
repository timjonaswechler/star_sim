// lib.rs - Hauptmodulstruktur für den wissenschaftlichen Sternsystem Generator

pub mod constants;
pub mod cosmic_environment;
pub mod habitability;
pub mod lagrange_points;
pub mod orbital_mechanics;
pub mod stellar_properties;
pub mod system_hierarchy;
pub mod units;

// Re-exports für einfache Verwendung
pub use constants::*;
pub use cosmic_environment::{CosmicEpoch, CosmicRadiationEnvironment, GalacticRegion};
pub use habitability::{HabitabilityAssessment, HabitableZone};
pub use lagrange_points::{LagrangeSystem, TrojanObject};
pub use orbital_mechanics::{EscapeVelocity, OrbitalElements, OrbitalPosition};
pub use stellar_properties::{EvolutionaryStage, LuminosityClass, SpectralType, StellarProperties};
pub use system_hierarchy::{StarSystem, SystemHierarchy, SystemType};
pub use units::{UnitSystem, Units};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j2000_epoch() {
        assert_eq!(J2000_EPOCH, 2451545.0);
    }
}
