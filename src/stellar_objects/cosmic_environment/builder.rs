use super::dynamic::GalacticDynamics;
use super::elemental_abundance::ElementalAbundance;
use super::epoch::CosmicEpoch;
use super::region::{CosmicRadiationEnvironment, GalacticRegion};
use crate::physics::units::{UnitSystem, Distance};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Komplettes kosmisches Umfeld mit Epoche und galaktischer Position
#[derive(Debug, Clone)]
pub struct CosmicEnvironment {
    pub epoch: CosmicEpoch,
    pub region: GalacticRegion,
    pub dynamics: GalacticDynamics,
    pub radiation: CosmicRadiationEnvironment,
    pub elements: ElementalAbundance,
    pub seed: u64,
}

/// Builder für [`CosmicEnvironment`]
pub struct CosmicEnvironmentBuilder {
    seed: u64,
    epoch: Option<CosmicEpoch>,
    region: Option<GalacticRegion>,
    units: UnitSystem,
}

impl CosmicEnvironmentBuilder {
    /// Erstellt einen neuen Builder
    pub fn new() -> Self {
        Self {
            seed: 0,
            epoch: None,
            region: None,
            units: UnitSystem::Astronomical,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_epoch(mut self, epoch: CosmicEpoch) -> Self {
        self.epoch = Some(epoch);
        self
    }

    pub fn with_region(mut self, region: GalacticRegion) -> Self {
        self.region = Some(region);
        self
    }

    pub fn with_units(mut self, units: UnitSystem) -> Self {
        self.units = units;
        self
    }

    /// Baut das kosmische Umfeld zusammen
    pub fn build(self) -> CosmicEnvironment {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);
        let epoch = self
            .epoch
            .unwrap_or_else(|| CosmicEpoch::from_age(rng.gen_range(3.0..13.8)));
        let region = self
            .region
            .unwrap_or_else(|| GalacticRegion::generate_random(&mut rng, self.units));
        let dynamics = GalacticDynamics::calculate_for_position(&region, epoch.age_universe, &mut rng);
        let radiation = CosmicRadiationEnvironment::from_region_and_epoch(&region, &epoch, &mut rng);
        let elements = ElementalAbundance::from_epoch(&epoch);
        CosmicEnvironment {
            epoch,
            region,
            dynamics,
            radiation,
            elements,
            seed: self.seed,
        }
    }
}

