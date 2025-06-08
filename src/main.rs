mod physics;
mod stellar_objects;
use stellar_objects::galaxy::properties::GalacticPosition;
use stellar_objects::universe::UniverseBuilder;
use stellar_objects::universe::{CosmicTime, Universe};
fn main() {
    println!("=== Star System Generator - Teil 1: Cosmic Time & Place ===\n");

    // Beispiel 1: Universum zur heutigen Zeit
    let modern_universe = UniverseBuilder::new()
        .with_cosmic_time(CosmicTime::new(13.8e9))
        .with_seed(12345)
        .build();

    println!("Modernes Universum:");
    println!(
        "Kosmische Zeit: {:.2e} Jahre",
        modern_universe
            .cosmic_time
            .years_since_big_bang
            .as_gigayears()
    );
    println!("Kosmische Epoche: {:?}", modern_universe.cosmic_time.era());
    println!(
        "CMB Temperatur: {:.2} K",
        modern_universe.cosmic_time.cmb_temperature()
    );
    println!("Galaxie: {:?}", modern_universe.galaxy.galaxy_type);
    println!(
        "Metallizität: {:.2} Z_sun",
        modern_universe.galaxy.metallicity
    );
    println!();

    // Beispiel 2: Bewohnbarkeit verschiedener Positionen
    let positions = [
        (
            "Galaktisches Zentrum",
            GalacticPosition {
                distance_from_center_kpc: 1.0,
                azimuth: 0.0,
            },
        ),
        (
            "Sonnen-Position",
            GalacticPosition {
                distance_from_center_kpc: 8.0,
                azimuth: 1.5,
            },
        ),
        (
            "Äußerer Rand",
            GalacticPosition {
                distance_from_center_kpc: 15.0,
                azimuth: 3.0,
            },
        ),
    ];

    println!("Bewohnbarkeits-Analyse:");
    for (name, position) in &positions {
        let habitability = modern_universe.evaluate_habitability(position);
        println!("{}: {:.2}% bewohnbar", name, habitability * 100.0);
    }
    println!();

    // Beispiel 3: Frühe Universums-Epoche
    let early_universe = Universe::builder()
        .with_cosmic_time(CosmicTime::new(500e6)) // 500 Millionen Jahre nach Big Bang
        .with_seed(54321)
        .build();

    println!("Frühes Universum (500 Mio Jahre nach Big Bang):");
    println!("Kosmische Epoche: {:?}", early_universe.cosmic_time.era());
    println!(
        "CMB Temperatur: {:.2} K",
        early_universe.cosmic_time.cmb_temperature()
    );
    println!(
        "CMB bewohnbare Epoche: {}",
        early_universe.cosmic_time.is_cmb_habitable_epoch()
    );

    let early_habitability = early_universe.evaluate_habitability(&positions[1].1);
    println!(
        "Bewohnbarkeit (Sonnen-Position): {:.2}%",
        early_habitability * 100.0
    );
}
