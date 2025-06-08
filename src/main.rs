mod physics;
mod stellar_objects;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use stellar_objects::moons::builder::{Moon, MoonBuilder};
use stellar_objects::planets::builder::{Planet, PlanetBuilder};
use stellar_objects::stellar_systems::builder::StarSystemBuilder;
use stellar_objects::stellar_systems::properties::StarSystem;

#[derive(Serialize, Deserialize)]
struct PlanetEntry {
    planet: Planet,
    moons: Vec<Moon>,
}

#[derive(Serialize, Deserialize)]
struct SystemData {
    system: StarSystem,
    planets: Vec<PlanetEntry>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a random seed for reproducibility of the created system
    // `gen` became a reserved keyword in Rust 2024, so use the raw identifier
    let seed = rand::thread_rng().r#gen::<u64>();
    let system = StarSystemBuilder::new().with_seed(seed).build();

    // Generate a small planetary system with random planets and moons
    let mut rng = rand::thread_rng();
    let planet_count = rng.gen_range(1..=5);
    let mut planets = Vec::new();
    for _ in 0..planet_count {
        let planet_seed = rng.r#gen::<u64>();
        let planet = PlanetBuilder::new().with_seed(planet_seed).build();

        let moon_count = rng.gen_range(0..=3);
        let mut moons = Vec::new();
        for _ in 0..moon_count {
            let moon_seed = rng.r#gen::<u64>();
            let moon = MoonBuilder::new().with_seed(moon_seed).build();
            moons.push(moon);
        }
        planets.push(PlanetEntry { planet, moons });
    }

    let data = SystemData { system, planets };

    // Serialize the system with planets and moons to RON format
    let ron_string = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())?;

    // Write the RON data to a file
    let mut file = File::create("star_system.ron")?;
    file.write_all(ron_string.as_bytes())?;

    println!(
        "Generated star system with seed {} and saved to star_system.ron",
        seed
    );

    Ok(())
}
