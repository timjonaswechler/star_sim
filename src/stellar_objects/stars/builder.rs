use super::properties::StellarProperties;
use crate::physics::units::{Mass, Time};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Builder für [`StellarProperties`]
pub struct StellarBuilder {
    seed: u64,
    mass: Option<Mass>,
    age: Option<Time>,
    metallicity: Option<f64>,
}

impl StellarBuilder {
    pub fn new() -> Self {
        Self {
            seed: 0,
            mass: None,
            age: None,
            metallicity: None,
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

    pub fn with_age(mut self, age: Time) -> Self {
        self.age = Some(age);
        self
    }

    pub fn with_metallicity(mut self, z: f64) -> Self {
        self.metallicity = Some(z);
        self
    }

    pub fn build(self) -> StellarProperties {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let mass = self
            .mass
            .unwrap_or_else(|| Mass::solar_masses(rng.gen_range(0.1..5.0)));
        let age = self
            .age
            .unwrap_or_else(|| Time::years(rng.gen_range(1e6..1e10)));
        let metallicity = self.metallicity.unwrap_or_else(|| rng.gen_range(-1.0..0.5));
        StellarProperties::new(mass, age, metallicity)
    }
}

