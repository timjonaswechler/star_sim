use super::properties::StarSystem;

/// Einfacher Builder für [`StarSystem`]
pub struct StarSystemBuilder {
    seed: u64,
}

impl StarSystemBuilder {
    pub fn new() -> Self {
        Self { seed: 0 }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn build(self) -> StarSystem {
        StarSystem::generate_from_seed(self.seed)
    }
}
