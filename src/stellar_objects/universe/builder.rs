use super::CosmicTime;
use super::Universe;
use crate::stellar_objects::galaxy::properties::*;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::SeedableRng;

pub struct UniverseBuilder {
    cosmic_time: Option<CosmicTime>,
    galaxy: Option<Galaxy>,
    seed: u64,
}

impl UniverseBuilder {
    pub fn new() -> Self {
        Self {
            cosmic_time: None,
            galaxy: None,
            seed: 42,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_cosmic_time(mut self, time: CosmicTime) -> Self {
        self.cosmic_time = Some(time);
        self
    }

    pub fn with_galaxy(mut self, galaxy: Galaxy) -> Self {
        self.galaxy = Some(galaxy);
        self
    }

    pub fn build(self) -> Universe {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seed);

        let cosmic_time = self.cosmic_time.unwrap_or_else(|| {
            // Zufällige Zeit im Modern Era
            let time_years = rng.gen_range(10e9..100e9);
            CosmicTime::new(time_years)
        });

        let galaxy = self
            .galaxy
            .unwrap_or_else(|| Self::generate_random_galaxy(&mut rng, &cosmic_time));

        Universe {
            cosmic_time,
            galaxy,
            radiation_history: Vec::new(),
            seed: self.seed,
        }
    }

    fn generate_random_galaxy(rng: &mut ChaCha8Rng, cosmic_time: &CosmicTime) -> Galaxy {
        let galaxy_type = match rng.gen_range(0..4) {
            0 => GalaxyType::Spiral,
            1 => GalaxyType::Elliptical,
            2 => GalaxyType::Irregular,
            _ => GalaxyType::Dwarf,
        };

        let age_gyr =
            rng.gen_range(1.0..cosmic_time.years_since_big_bang.value_in_system_base() / 1e9);

        let mass_solar_masses = match galaxy_type {
            GalaxyType::Spiral => rng.gen_range(1e10..1e12),
            GalaxyType::Elliptical => rng.gen_range(1e11..1e13),
            GalaxyType::Irregular => rng.gen_range(1e8..1e10),
            GalaxyType::Dwarf => rng.gen_range(1e6..1e9),
        };

        // Metallizität steigt mit kosmischer Zeit
        let time_factor =
            (cosmic_time.years_since_big_bang.value_in_system_base() / 13.8e9).min(1.0);
        let metallicity = rng.gen_range(0.1..2.0) * time_factor;

        let star_formation_rate = match galaxy_type {
            GalaxyType::Spiral => rng.gen_range(0.1..10.0),
            GalaxyType::Elliptical => rng.gen_range(0.01..1.0),
            GalaxyType::Irregular => rng.gen_range(0.05..2.0),
            GalaxyType::Dwarf => rng.gen_range(0.001..0.1),
        };

        let has_active_nucleus = rng.gen_bool(0.1); // 10% Chance für AGN

        Galaxy {
            galaxy_type,
            age_gyr,
            mass_solar_masses,
            metallicity,
            star_formation_rate,
            has_active_nucleus,
        }
    }
}
