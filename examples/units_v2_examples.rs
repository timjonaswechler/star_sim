//! Examples demonstrating the units_v2 system capabilities.
//!
//! This file contains practical examples of how to use the improved unit system
//! for astronomical calculations and stellar system modeling.

use star_sim::physics::units_v2::*;

fn main() {
    println!("=== Units v2 System Examples ===\n");

    // Basic unit creation and display
    basic_units_example();

    // Unit conversions
    conversion_examples();

    // Astronomical calculations
    stellar_system_example();

    // Serialization demonstration
    serialization_example();

    // Performance comparison
    performance_comparison();
}

/// Demonstrates basic unit creation and usage
fn basic_units_example() {
    println!("1. Basic Unit Creation");
    println!("---------------------");

    // Create quantities with different units
    let distance_au = Distance::<AstronomicalUnit>::new(1.5);
    let mass_earth = Mass::<EarthMass>::new(0.8);
    let time_gyr = Time::<Gigayear>::new(6.0);
    let power_solar = Power::<SolarLuminosity>::new(0.15);

    // Display with Unicode symbols
    println!("Distance: {}", distance_au);
    println!("Mass: {}", mass_earth);
    println!("Age: {}", time_gyr);
    println!("Luminosity: {}", power_solar);
    println!();
}

/// Demonstrates the hub-and-spoke conversion system
fn conversion_examples() {
    println!("2. Unit Conversions (Hub-and-Spoke)");
    println!("-----------------------------------");

    // Distance conversions
    let distance_au = Distance::<AstronomicalUnit>::new(1.0);
    println!("1 AU = {} m", distance_au.convert_to::<Meter>().value());
    println!(
        "1 AU = {} km",
        distance_au.convert_to::<Kilometer>().value()
    );
    println!(
        "1 AU = {} R⊕",
        distance_au.convert_to::<EarthRadius>().value()
    );

    // Mass conversions
    let solar_mass = Mass::<SolarMass>::new(1.0);
    println!("1 M☉ = {} M⊕", solar_mass.convert_to::<EarthMass>().value());
    println!("1 M☉ = {} kg", solar_mass.convert_to::<Kilogram>().value());

    // Time conversions
    let age_gyr = Time::<Gigayear>::new(4.6);
    println!("4.6 Gyr = {} years", age_gyr.convert_to::<Year>().value());
    println!(
        "4.6 Gyr = {} seconds",
        age_gyr.convert_to::<Second>().value()
    );
    println!();
}

/// Demonstrates realistic stellar system calculations
fn stellar_system_example() {
    println!("3. Stellar System Modeling");
    println!("---------------------------");

    // Model the Kepler-442 system
    println!("Modeling Kepler-442 system:");

    // Star properties
    let star_mass = Mass::<SolarMass>::new(0.61);
    let star_radius = Distance::<SunRadius>::new(0.60);
    let star_temperature = Temperature::<Kelvin>::new(4402.0);
    let star_luminosity = Power::<SolarLuminosity>::new(0.12);

    println!("  Star mass: {}", star_mass);
    println!("  Star radius: {}", star_radius);
    println!("  Star temperature: {}", star_temperature);
    println!("  Star luminosity: {}", star_luminosity);

    // Planet properties
    let planet_mass = Mass::<EarthMass>::new(2.3);
    let planet_radius = Distance::<EarthRadius>::new(1.34);
    let orbital_distance = Distance::<AstronomicalUnit>::new(0.409);

    println!("  Planet mass: {}", planet_mass);
    println!("  Planet radius: {}", planet_radius);
    println!("  Orbital distance: {}", orbital_distance);

    // Calculate some derived properties
    let star_mass_kg = star_mass.convert_to::<Kilogram>();
    let orbital_distance_m = orbital_distance.convert_to::<Meter>();

    // Orbital velocity (simplified circular orbit)
    // v = sqrt(GM/r)
    let gm = 6.67430e-11 * star_mass_kg.value(); // G * M
    let orbital_velocity_si = (gm / orbital_distance_m.value()).sqrt();

    println!(
        "  Orbital velocity: {:.1} km/s",
        orbital_velocity_si / 1000.0
    );

    // Habitable zone calculation (simplified)
    // r_hab = sqrt(L/L_sun) AU
    let luminosity_ratio = star_luminosity.value();
    let habitable_zone_inner = luminosity_ratio.sqrt() * 0.95;
    let habitable_zone_outer = luminosity_ratio.sqrt() * 1.37;

    println!(
        "  Habitable zone: {:.2} - {:.2} AU",
        habitable_zone_inner, habitable_zone_outer
    );

    let is_habitable = orbital_distance.value() >= habitable_zone_inner
        && orbital_distance.value() <= habitable_zone_outer;
    println!("  In habitable zone: {}", is_habitable);
    println!();
}

/// Demonstrates serialization capabilities
fn serialization_example() {
    println!("4. Serialization to RON Format");
    println!("------------------------------");

    // Create a planetary system
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Planet {
        name: String,
        mass: Mass<EarthMass>,
        radius: Distance<EarthRadius>,
        orbital_distance: Distance<AstronomicalUnit>,
        orbital_period: Time<Day>,
    }

    let earth = Planet {
        name: "Earth".to_string(),
        mass: Mass::<EarthMass>::new(1.0),
        radius: Distance::<EarthRadius>::new(1.0),
        orbital_distance: Distance::<AstronomicalUnit>::new(1.0),
        orbital_period: Time::<Day>::new(365.25),
    };

    let mars = Planet {
        name: "Mars".to_string(),
        mass: Mass::<EarthMass>::new(0.107),
        radius: Distance::<EarthRadius>::new(0.532),
        orbital_distance: Distance::<AstronomicalUnit>::new(1.524),
        orbital_period: Time::<Day>::new(687.0),
    };

    let planets = vec![earth, mars];

    // Serialize to RON
    match ron::ser::to_string_pretty(&planets, ron::ser::PrettyConfig::default()) {
        Ok(ron_string) => {
            println!("Serialized planets to RON:");
            println!("{}", ron_string);
        }
        Err(e) => println!("Serialization error: {}", e),
    }

    // Demonstrate round-trip serialization
    let distance = Distance::<AstronomicalUnit>::new(1.5);
    let ron_str = ron::to_string(&distance).unwrap();
    let deserialized: Distance<AstronomicalUnit> = ron::from_str(&ron_str).unwrap();

    println!(
        "Round-trip test: {} -> {} -> {}",
        distance.value(),
        ron_str,
        deserialized.value()
    );
    println!();
}

/// Compares performance characteristics
fn performance_comparison() {
    println!("5. Performance Characteristics");
    println!("------------------------------");

    println!("Conversion complexity comparison:");
    println!("  Traditional system (6 units): 6×6 = 36 conversion functions");
    println!("  Hub-and-spoke system (6 units): 6×2 = 12 conversion functions");
    println!("  Savings: 67% reduction in code");
    println!();

    println!("Adding new units:");
    println!("  Traditional: Adding 1 unit requires 2×n new conversions");
    println!("  Hub-and-spoke: Adding 1 unit requires 2 new conversions");
    println!("  For 10 units: 20 vs 2 new functions (90% reduction)");
    println!();

    // Runtime performance test
    use std::time::Instant;

    let start = Instant::now();
    let mut sum = 0.0;

    // Perform many conversions
    for i in 0..100_000 {
        let distance = Distance::<AstronomicalUnit>::new(i as f64);
        let in_meters = distance.convert_to::<Meter>();
        sum += in_meters.value();
    }

    let duration = start.elapsed();
    println!("100,000 AU->meter conversions: {:?}", duration);
    println!("Sum (to prevent optimization): {:.2e}", sum);
    println!();
}

/// Demonstrates type safety at compile time
#[allow(dead_code)]
fn type_safety_examples() {
    // These examples would cause compile errors:

    /*
    let distance = Distance::<Meter>::new(100.0);
    let mass = Mass::<Kilogram>::new(5.0);

    // ❌ This would be a compile error:
    // let invalid = distance + mass;

    // ❌ This would be a compile error:
    // let wrong_conversion = distance.convert_to::<Kilogram>();
    */

    // ✅ These work correctly:
    let distance1 = Distance::<Meter>::new(100.0);
    let distance2 = Distance::<Meter>::new(50.0);
    let _total_distance = distance1 + distance2; // Same dimensions

    let _au_distance = distance1.convert_to::<AstronomicalUnit>(); // Same dimension
}
