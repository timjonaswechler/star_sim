use super::properties::StarSystem;
use crate::physics::units::UnitSystem;

/// Einfacher Builder für [`StarSystem`]
pub struct StarSystemBuilder {
    seed: u64,
    unit_system: UnitSystem,
}

impl StarSystemBuilder {
    pub fn new() -> Self {
        Self {
            seed: 0,
            unit_system: UnitSystem::Astronomical,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_unit_system(mut self, unit_system: UnitSystem) -> Self {
        self.unit_system = unit_system;
        self
    }

    pub fn build(self) -> StarSystem {
        StarSystem::generate_from_seed_with_units(self.seed, self.unit_system)
    }
}

