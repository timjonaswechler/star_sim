use super::properties::PhysicalProperties;
use crate::physics::unit_system::mass::Mass;
use crate::physics::unit_system::time::Time;
use crate::stellar_objects::planets::properties::PlanetComposition;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Builder zur Erzeugung von [`PhysicalProperties`]
pub struct PhysicalPropertiesBuilder {
    seed: u64,
    mass: Option<Mass>,
    composition: Option<PlanetComposition>,
    age: Option<Time>,
}

impl PhysicalPropertiesBuilder {
    /// Neuer Builder mit Standardwerten
    pub fn new() -> Self {
        Self {
            seed: 0,
            mass: None,
            composition: None,
            age: None,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_mass(mut self, mass: Mass) -> Self {
        self.mass = Some(mass);
        self
    }

    pub fn with_composition(mut self, comp: PlanetComposition) -> Self {
        self.composition = Some(comp);
        self
    }

    pub fn with_age(mut self, age: Time) -> Self {
        self.age = Some(age);
        self
    }

    /// Baut die [`PhysicalProperties`]
    pub fn build(self) -> PhysicalProperties {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let mass = self
            .mass
            .unwrap_or_else(|| Mass::earth_masses(rng.gen_range(0.1..10.0)));
        let comp = self
            .composition
            .unwrap_or_else(|| PlanetComposition::Terrestrial { rock_fraction: 0.7 });
        let age = self
            .age
            .unwrap_or_else(|| Time::years(rng.gen_range(1.0e6..4.5e9)));
        PhysicalProperties::from_mass_and_composition(mass, comp, age, &mut rng)
    }
}
