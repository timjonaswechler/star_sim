use crate::physics::units::{Distance, Mass};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// Einfache Mondstruktur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moon {
    pub mass: Mass,
    pub semi_major_axis: Distance,
    pub seed: u64,
}

/// Builder für [`Moon`]
pub struct MoonBuilder {
    seed: u64,
    mass: Option<Mass>,
    semi_major_axis: Option<Distance>,
}

impl MoonBuilder {
    pub fn new() -> Self {
        Self {
            seed: 0,
            mass: None,
            semi_major_axis: None,
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

    pub fn with_semi_major_axis(mut self, axis: Distance) -> Self {
        self.semi_major_axis = Some(axis);
        self
    }

    pub fn build(self) -> Moon {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let mass = self
            .mass
            .unwrap_or_else(|| Mass::earth_masses(rng.gen_range(0.001..0.1)));
        let axis = self
            .semi_major_axis
            .unwrap_or_else(|| Distance::earth_radii(rng.gen_range(30.0..1000.0)));
        Moon {
            mass,
            semi_major_axis: axis,
            seed: self.seed,
        }
    }
}
