mod physics;
mod stellar_objects;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use physics::astrophysics::orbit::elements::OrbitalElements;
use physics::units::Distance;
use physics::constants::KILOPARSEC_IN_METERS;

use stellar_objects::cosmic_environment::builder::CosmicEnvironmentBuilder;
use stellar_objects::moons::builder::{Moon, MoonBuilder};
use stellar_objects::planets::builder::{Planet, PlanetBuilder};
use stellar_objects::stars::builder::StellarBuilder;
use stellar_objects::stars::properties::StellarProperties;
use stellar_objects::stellar_systems::builder::StarSystemBuilder;
use stellar_objects::stellar_systems::properties::StarSystem;
use stellar_objects::trojans_asteroid::builder::TrojanBuilder;
use stellar_objects::trojans_asteroid::objects::TrojanObject;
use stellar_objects::universe::UniverseBuilder;
use stellar_objects::galaxy::properties::GalacticPosition;

#[derive(Serialize, Deserialize)]
struct PlanetEntry {
    planet: Planet,
    moons: Vec<Moon>,
    trojans: Vec<TrojanObject>,
}

#[derive(Serialize, Deserialize)]
struct SystemData {
    star: StellarProperties,
    system: StarSystem,
    planets: Vec<PlanetEntry>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    // Build a full universe context
    let universe_seed = rng.r#gen::<u64>();
    let universe = UniverseBuilder::new().with_seed(universe_seed).build();

    // Generate a cosmic environment within that universe
    let env_seed = rng.r#gen::<u64>();
    let environment = CosmicEnvironmentBuilder::new()
        .with_seed(env_seed)
        .with_unit_system(universe.cosmic_time.years_since_big_bang.system)
        .build();

    // Generate the stellar system
    let system_seed = rng.r#gen::<u64>();
    let system = StarSystemBuilder::new().with_seed(system_seed).build();

    // Create a primary star
    let star_seed = rng.r#gen::<u64>();
    let star = StellarBuilder::new().with_seed(star_seed).build();

    // Evaluate the galactic environment
    let stability = environment.dynamics.environmental_stability();
    let distance_kpc = environment
        .region
        .distance_from_center()
        .in_meters()
        / KILOPARSEC_IN_METERS;
    let position = GalacticPosition {
        distance_from_center_kpc: distance_kpc,
        azimuth: 0.0,
    };
    let habitability = universe.evaluate_habitability(&position);

    // Example orbital calculation using physics modules
    let earth_orbit = OrbitalElements::new(
        Distance::au(1.0),
        0.0167,
        0.0,
        0.0,
        0.0,
        0.0,
    );
    let period = earth_orbit.orbital_period(&star.mass);

    // Generate planets with moons and trojans
    let planet_count = rng.gen_range(1..=3);
    let mut planets = Vec::new();
    for _ in 0..planet_count {
        let planet_seed = rng.r#gen::<u64>();
        let planet = PlanetBuilder::new().with_seed(planet_seed).build();

        let moon_count = rng.gen_range(0..=2);
        let mut moons = Vec::new();
        for _ in 0..moon_count {
            let moon_seed = rng.r#gen::<u64>();
            let moon = MoonBuilder::new().with_seed(moon_seed).build();
            moons.push(moon);
        }

        // Attach a trojan object
        let trojan_seed = rng.r#gen::<u64>();
        let trojan = TrojanBuilder::new().with_seed(trojan_seed).build();

        planets.push(PlanetEntry {
            planet,
            moons,
            trojans: vec![trojan],
        });
    }

    let data = SystemData {
        star,
        system,
        planets,
    };

    // Serialize to RON
    let ron_string = ron::ser::to_string_pretty(&data, ron::ser::PrettyConfig::default())?;
    let mut file = File::create("complete_system.ron")?;
    file.write_all(ron_string.as_bytes())?;

    println!(
        "Generated system with seed {}. Earth-like period: {:.2} years. Env stability {:.2}, habitability {:.2}",
        system_seed,
        period.in_years(),
        stability,
        habitability
    );

    Ok(())
}
