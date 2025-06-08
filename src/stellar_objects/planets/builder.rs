use super::properties::PlanetComposition;
use crate::stellar_objects::bodies::builder::{PhysicalPropertiesBuilder};
use crate::stellar_objects::bodies::properties::PhysicalProperties;
use crate::physics::units::{Mass, Time};

/// Einfacher Planet mit physikalischen Eigenschaften
#[derive(Debug, Clone)]
pub struct Planet {
    pub properties: PhysicalProperties,
}

/// Builder zur Erstellung eines [`Planet`]
pub struct PlanetBuilder {
    inner: PhysicalPropertiesBuilder,
}

impl PlanetBuilder {
    /// Neuer Builder
    pub fn new() -> Self {
        Self {
            inner: PhysicalPropertiesBuilder::new(),
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.inner = self.inner.with_seed(seed);
        self
    }

    pub fn with_mass(mut self, mass: Mass) -> Self {
        self.inner = self.inner.with_mass(mass);
        self
    }

    pub fn with_composition(mut self, comp: PlanetComposition) -> Self {
        self.inner = self.inner.with_composition(comp);
        self
    }

    pub fn with_age(mut self, age: Time) -> Self {
        self.inner = self.inner.with_age(age);
        self
    }

    /// Baut den Planet
    pub fn build(self) -> Planet {
        Planet {
            properties: self.inner.build(),
        }
    }
}

