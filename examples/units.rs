//! Comprehensive demonstration of the Star Sim unit system capabilities.
//!
//! This example showcases all major features of the type-safe unit system including:
//! - Type-safe unit creation and conversion
//! - Dimensional analysis and compile-time safety
//! - Astronomical unit support with Unicode symbols
//! - Mathematical operations with proper units
//! - Serialization and deserialization
//! - Performance characteristics

use serde_json;
use star_sim::physics::units::*;
use std::time::Instant;

fn main() {
    println!("🌟 Star Sim Unit System - Comprehensive Example\n");

    // ================================================================================================
    // 1. Basic Unit Creation and Display
    // ================================================================================================

    println!("1️⃣ Basic Unit Creation and Display");
    println!("──────────────────────────────────");

    // Create quantities with different units
    let distance_au = Distance::<AstronomicalUnit>::new(1.5);
    let distance_meters = Distance::<Meter>::new(100_000.0);
    let distance_parsecs = Distance::<Parsec>::new(0.5);

    let mass_solar = Mass::<SolarMass>::new(2.1);
    let mass_earth = Mass::<EarthMass>::new(5.7);
    let mass_kg = Mass::<Kilogram>::new(1000.0);

    let time_gyr = Time::<Prefixed<Giga, Year>>::new(4.6);
    let time_years = Time::<Year>::new(365.25);
    let time_seconds = Time::<Second>::new(3600.0);

    let power_solar = Power::<SolarLuminosity>::new(0.8);
    let energy_joules = Energy::<Joule>::new(1e15);

    println!("Distances:");
    println!("  {}", distance_au);
    println!("  {}", distance_meters);
    println!("  {}", distance_parsecs);

    println!("\nMasses:");
    println!("  {}", mass_solar);
    println!("  {}", mass_earth);
    println!("  {}", mass_kg);

    println!("\nTime periods:");
    println!("  {}", time_gyr);
    println!("  {}", time_years);
    println!("  {}", time_seconds);

    println!("\nEnergy and Power:");
    println!("  {}", power_solar);
    println!("  {}", energy_joules);

    // ================================================================================================
    // 2. Hub-and-Spoke Unit Conversions
    // ================================================================================================

    println!("\n2️⃣ Hub-and-Spoke Unit Conversions");
    println!("─────────────────────────────────");

    // Convert the same distance between different units
    let dist_au = Distance::<AstronomicalUnit>::new(1.0);
    let dist_meters = dist_au.convert_to::<Meter>();
    let dist_ly = dist_au.convert_to::<LightYear>();
    let dist_parsecs = dist_au.convert_to::<Parsec>();

    println!("1 AU in different units:");
    println!("  {} = {}", dist_au, dist_meters);
    println!("  {} = {}", dist_au, dist_ly);
    println!("  {} = {}", dist_au, dist_parsecs);

    // Mass conversions
    let mass_solar_1 = Mass::<SolarMass>::new(1.0);
    let mass_earth_equiv = mass_solar_1.convert_to::<EarthMass>();
    let mass_kg_equiv = mass_solar_1.convert_to::<Kilogram>();

    println!("\n1 Solar Mass in different units:");
    println!("  {} = {}", mass_solar_1, mass_earth_equiv);
    println!("  {} = {:.2e} kg", mass_solar_1, mass_kg_equiv.value());

    // Time conversions
    let age_universe = Time::<Prefixed<Giga, Year>>::new(13.8);
    let age_years = age_universe.convert_to::<Year>();
    let age_seconds = age_universe.convert_to::<Second>();

    println!("\nAge of the Universe:");
    println!("  {} = {:.1e} years", age_universe, age_years.value());
    println!("  {} = {:.2e} seconds", age_universe, age_seconds.value());

    // ================================================================================================
    // 3. Type-Safe Arithmetic Operations
    // ================================================================================================

    println!("\n3️⃣ Type-Safe Arithmetic Operations");
    println!("──────────────────────────────────");

    // Same-unit arithmetic
    let dist1 = Distance::<AstronomicalUnit>::new(1.0);
    let dist2 = Distance::<AstronomicalUnit>::new(0.5);
    let total_distance = dist1 + dist2;
    let distance_diff = dist1 - dist2;

    println!("Distance arithmetic (same units):");
    println!("  {} + {} = {}", dist1, dist2, total_distance);
    println!("  {} - {} = {}", dist1, dist2, distance_diff);

    // Scalar operations
    let doubled = dist1 * 2.0;
    let halved = dist1 / 2.0;
    let negated = -dist1;

    println!("\nScalar operations:");
    println!("  {} × 2 = {}", dist1, doubled);
    println!("  {} ÷ 2 = {}", dist1, halved);
    println!("  -{} = {}", dist1, negated);

    // Mass operations
    let star_mass = Mass::<SolarMass>::new(1.5);
    let companion_mass = Mass::<SolarMass>::new(0.8);
    let binary_system_mass = star_mass + companion_mass;

    println!("\nBinary star system:");
    println!("  Primary: {}", star_mass);
    println!("  Secondary: {}", companion_mass);
    println!("  Total: {}", binary_system_mass);

    // ================================================================================================
    // 4. Dimensional Analysis with Helper Functions
    // ================================================================================================

    println!("\n4️⃣ Dimensional Analysis with Helper Functions");
    println!("─────────────────────────────────────────────");

    // Calculate velocity: distance / time
    let journey_distance = Distance::<Meter>::new(299_792_458.0); // Speed of light in m
    let journey_time = Time::<Second>::new(1.0);
    let speed = calculate_velocity(journey_distance, journey_time);

    println!("Physics calculations:");
    println!("  Distance: {} meters", journey_distance.value());
    println!("  Time: {} seconds", journey_time.value());
    println!("  Velocity: {:.0} m/s (speed of light)", speed);

    // Astronomical calculations
    let stellar_distance = Distance::<AstronomicalUnit>::new(5.2); // Jupiter's distance
    let orbital_period = Time::<Year>::new(11.86); // Jupiter's orbital period
    let stellar_distance_m = stellar_distance.convert_to::<Meter>();
    let orbital_period_s = orbital_period.convert_to::<Second>();
    let orbital_velocity = calculate_velocity(stellar_distance_m, orbital_period_s);

    println!("\nJupiter's orbital characteristics:");
    println!("  Distance: {}", stellar_distance);
    println!("  Period: {}", orbital_period);
    println!("  Average orbital velocity: {:.0} m/s", orbital_velocity);

    // ================================================================================================
    // 5. Advanced Astronomical Examples
    // ================================================================================================

    println!("\n5️⃣ Advanced Astronomical Examples");
    println!("─────────────────────────────────");

    // Stellar parameters
    let sirius_mass = Mass::<SolarMass>::new(2.063);
    let sirius_radius = Distance::<SunRadius>::new(1.711);
    let sirius_luminosity = Power::<SolarLuminosity>::new(25.4);
    let sirius_distance = Distance::<Parsec>::new(2.64);

    println!("Sirius A properties:");
    println!("  Mass: {}", sirius_mass);
    println!("  Radius: {}", sirius_radius);
    println!("  Luminosity: {}", sirius_luminosity);
    println!("  Distance: {}", sirius_distance);

    // Convert to different units for comparison
    let sirius_radius_earth = sirius_radius.convert_to::<EarthRadius>();
    let sirius_distance_ly = sirius_distance.convert_to::<LightYear>();

    println!(
        "  Radius in Earth radii: {:.1}",
        sirius_radius_earth.value()
    );
    println!(
        "  Distance in light-years: {:.1}",
        sirius_distance_ly.value()
    );

    // Exoplanet example
    let kepler_452b_mass = Mass::<EarthMass>::new(5.0); // Estimated
    let kepler_452b_radius = Distance::<EarthRadius>::new(1.6);
    let kepler_452b_distance = Distance::<LightYear>::new(1402.0);

    println!("\nKepler-452b (Earth's cousin):");
    println!("  Mass: {}", kepler_452b_mass);
    println!("  Radius: {}", kepler_452b_radius);
    println!("  Distance: {}", kepler_452b_distance);

    // ================================================================================================
    // 6. Angular Measurements
    // ================================================================================================

    println!("\n6️⃣ Angular Measurements");
    println!("──────────────────────");

    let full_circle = Angle::<Degree>::new(360.0);
    let half_circle = Angle::<Degree>::new(180.0);
    let right_angle = Angle::<Degree>::new(90.0);

    let full_circle_rad = full_circle.convert_to::<Radian>();
    let half_circle_rad = half_circle.convert_to::<Radian>();
    let right_angle_rad = right_angle.convert_to::<Radian>();

    println!("Angular conversions:");
    println!("  {} = {:.4} radians", full_circle, full_circle_rad.value());
    println!("  {} = {:.4} radians", half_circle, half_circle_rad.value());
    println!("  {} = {:.4} radians", right_angle, right_angle_rad.value());

    // Angular velocity
    let earth_rotation = AngularVelocity::<DegreePerSecond>::new(360.0 / (24.0 * 3600.0));
    let earth_rotation_rad = earth_rotation.convert_to::<RadianPerSecond>();

    println!("\nEarth's rotation:");
    println!("  {:.6} °/s", earth_rotation.value());
    println!("  {:.8} rad/s", earth_rotation_rad.value());

    // ================================================================================================
    // 7. Derived Quantities
    // ================================================================================================

    println!("\n7️⃣ Derived Quantities");
    println!("────────────────────");

    // Area and volume
    let earth_surface_area = Area::<SquareKilometer>::new(510_072_000.0);
    let earth_volume = Volume::<CubicMeter>::new(1.08321e21);

    println!("Earth dimensions:");
    println!("  Surface area: {}", earth_surface_area);
    println!("  Volume: {:.2e} cubic meters", earth_volume.value());

    // Density calculation (conceptual)
    let earth_mass_full = Mass::<Kilogram>::new(5.972e24);
    let earth_volume_m3 = earth_volume.value();
    let earth_density = earth_mass_full.value() / earth_volume_m3;

    println!("  Mean density: {:.0} kg/m³", earth_density);

    // Pressure and force examples
    let atmospheric_pressure = Pressure::<Bar>::new(1.01325);
    let atmospheric_pressure_pa = atmospheric_pressure.convert_to::<Pascal>();

    println!("\nAtmospheric conditions:");
    println!("  Sea level pressure: {}", atmospheric_pressure);
    println!("  In Pascals: {:.0} Pa", atmospheric_pressure_pa.value());

    // ================================================================================================
    // 8. Serialization and Deserialization
    // ================================================================================================

    println!("\n8️⃣ Serialization and Deserialization");
    println!("───────────────────────────────────");

    // Create a complex astronomical object
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct Star {
        name: String,
        mass: Mass<SolarMass>,
        radius: Distance<SunRadius>,
        luminosity: Power<SolarLuminosity>,
        distance: Distance<Parsec>,
        age: Time<Prefixed<Giga, Year>>,
    }

    let vega = Star {
        name: "Vega".to_string(),
        mass: Mass::<SolarMass>::new(2.135),
        radius: Distance::<SunRadius>::new(2.362),
        luminosity: Power::<SolarLuminosity>::new(40.12),
        distance: Distance::<Parsec>::new(7.68),
        age: Time::<Prefixed<Giga, Year>>::new(0.455),
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&vega).unwrap();
    println!("Serialized star data:");
    println!("{}", json);

    // Deserialize back
    let vega_deserialized: Star = serde_json::from_str(&json).unwrap();
    println!("\nDeserialized star:");
    println!("  Name: {}", vega_deserialized.name);
    println!("  Mass: {}", vega_deserialized.mass);
    println!("  Radius: {}", vega_deserialized.radius);
    println!("  Luminosity: {}", vega_deserialized.luminosity);
    println!("  Distance: {}", vega_deserialized.distance);
    println!("  Age: {}", vega_deserialized.age);

    // ================================================================================================
    // 9. Performance Characteristics
    // ================================================================================================

    println!("\n9️⃣ Performance Characteristics");
    println!("─────────────────────────────");

    let iterations = 1_000_000;

    // Measure unit creation performance
    let start = Instant::now();
    for i in 0..iterations {
        let _distance = Distance::<AstronomicalUnit>::new(i as f64);
    }
    let creation_time = start.elapsed();

    // Measure conversion performance
    let base_distance = Distance::<AstronomicalUnit>::new(1.0);
    let start = Instant::now();
    for _ in 0..iterations {
        let _converted = base_distance.convert_to::<Meter>();
    }
    let conversion_time = start.elapsed();

    // Measure arithmetic performance
    let dist1 = Distance::<AstronomicalUnit>::new(1.0);
    let dist2 = Distance::<AstronomicalUnit>::new(2.0);
    let start = Instant::now();
    for _ in 0..iterations {
        let _sum = dist1 + dist2;
    }
    let arithmetic_time = start.elapsed();

    println!("Performance ({} iterations):", iterations);
    println!(
        "  Unit creation: {:?} ({:.0} ns/op)",
        creation_time,
        creation_time.as_nanos() as f64 / iterations as f64
    );
    println!(
        "  Unit conversion: {:?} ({:.0} ns/op)",
        conversion_time,
        conversion_time.as_nanos() as f64 / iterations as f64
    );
    println!(
        "  Arithmetic: {:?} ({:.0} ns/op)",
        arithmetic_time,
        arithmetic_time.as_nanos() as f64 / iterations as f64
    );

    // ================================================================================================
    // 10. Compile-Time Safety Examples
    // ================================================================================================

    println!("\n🔟 Compile-Time Safety Examples");
    println!("──────────────────────────────");

    println!("✅ Valid operations:");
    let distance = Distance::<Meter>::new(100.0);
    let same_distance = Distance::<Meter>::new(50.0);
    let _valid_sum = distance + same_distance;
    println!("  Distance + Distance = Valid");

    let mass = Mass::<Kilogram>::new(10.0);
    let _valid_scaled = mass * 2.0;
    println!("  Mass × Scalar = Valid");

    println!("\n❌ The following would cause compile errors:");
    println!("  Distance + Mass = Compile Error!");
    println!("  Temperature - Velocity = Compile Error!");
    println!("  Mass × Time = Compile Error! (without helper functions)");

    // Demonstrate dimensional safety by commenting out these lines:
    // let _invalid = distance + mass;  // ❌ Compile error
    // let _invalid2 = Temperature::<Kelvin>::new(300.0) - Velocity::<MeterPerSecond>::new(10.0);  // ❌ Compile error

    // ================================================================================================
    // 11. Metric Prefix System
    // ================================================================================================

    println!("\n1️⃣1️⃣ Metric Prefix System");
    println!("─────────────────────────");

    println!("Metric prefixes enable scalable unit combinations:");

    // ================================================================================================
    // Real Prefix Examples with Type-Safe Conversions
    // ================================================================================================

    println!("\n📏 Distance Prefixes (with real conversions):");

    // Create prefixed distance unitsHuman hair
    let paper_thickness = Distance::<Prefixed<Milli, Meter>>::new(0.1);

    // Convert all to meters for comparison
    let paper_thickness_m = paper_thickness.convert_to::<Meter>();
    println!(
        "  Paper thickness: {} = {:.4} m",
        paper_thickness,
        paper_thickness_m.value()
    );

    // Demonstrate prefix system benefits
    println!("\n🎯 Prefix System Benefits:");
    println!("✓ Avoids combinatorial explosion (n×m vs n+m units)");
    println!("✓ Automatic symbol generation (k + m = km)");
    println!("✓ Maintains type safety and dimensional analysis");
    println!("✓ Consistent hub-and-spoke conversions");
    println!("✓ Supports all standard SI prefixes (yocto to yotta)");
    println!("✓ Real working implementation with proper conversions");

    // ================================================================================================
    // 12. Scientific Notation and Scale Awareness
    // ================================================================================================

    println!("\n1️⃣2️⃣ Scientific Notation and Scale Awareness");
    println!("────────────────────────────────────────────");

    // Demonstrate the vast range of scales the unit system handles
    println!("The unit system handles an enormous range of scales:");

    let planck_length = Distance::<Meter>::new(1.616e-35);
    let observable_universe = Distance::<Meter>::new(8.8e26);
    let scale_ratio = observable_universe.value() / planck_length.value();

    println!("\n🔬 Smallest to Largest:");
    println!("  Planck length: {:.2e} m", planck_length.value());
    println!(
        "  Observable universe: {:.1e} m",
        observable_universe.value()
    );
    println!(
        "  Scale ratio: {:.1e} (61 orders of magnitude!)",
        scale_ratio
    );

    let planck_time = Time::<Second>::new(5.391e-44);
    let age_universe = Time::<Prefixed<Giga, Year>>::new(13.8);
    let age_universe_s = age_universe.convert_to::<Second>();
    let time_ratio = age_universe_s.value() / planck_time.value();

    println!("\n⏱️ Shortest to Longest Times:");
    println!("  Planck time: {:.2e} s", planck_time.value());
    println!("  Age of universe: {:.1e} s", age_universe_s.value());
    println!(
        "  Time ratio: {:.1e} (60+ orders of magnitude!)",
        time_ratio
    );

    let electron_mass_example = Mass::<Kilogram>::new(9.109e-31);
    let observable_universe_mass = Mass::<Kilogram>::new(1.5e53);
    let mass_ratio = observable_universe_mass.value() / electron_mass_example.value();

    println!("\n⚖️ Lightest to Heaviest:");
    println!("  Electron mass: {:.2e} kg", electron_mass_example.value());
    println!(
        "  Observable universe mass: {:.1e} kg",
        observable_universe_mass.value()
    );
    println!("  Mass ratio: {:.1e} (83 orders of magnitude!)", mass_ratio);

    // ================================================================================================
    // Summary
    // ================================================================================================

    println!("\n🎯 Summary of Unit System Features");
    println!("─────────────────────────────────");
    println!("✓ Type-safe unit creation and conversion");
    println!("✓ Hub-and-spoke conversion system (O(n) complexity)");
    println!("✓ Compile-time dimensional analysis");
    println!("✓ Astronomical units with Unicode symbols");
    println!("✓ Mathematical operations with proper units");
    println!("✓ Full serialization/deserialization support");
    println!("✓ Zero-cost abstractions for high performance");
    println!("✓ Extensive coverage of physical dimensions");
    println!("✓ Metric prefix system (scalable unit combinations)");
    println!("✓ Scientific notation support (60+ orders of magnitude)");
    println!("✓ Astronomy-focused design for stellar simulation");

    println!("\n🌟 Perfect for stellar system generation and astrophysics calculations!");
}
