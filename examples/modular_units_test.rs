//! Test and demonstration of the new modular unit composition system.
//!
//! This example shows the new intuitive syntax for composing units:
//! - Area::<Meter, Squared>
//! - Velocity::<Meter, Per<Second>>
//! - Acceleration::<Meter, Per<Second, Squared>>
//! - Complex compositions with prefixes

use star_sim::physics::units::*;

fn main() {
    println!("=== Modular Unit Composition System Test ===\n");

    // ================================================================================================
    // SIMPLE MODULAR SYNTAX
    // ================================================================================================

    println!("1. Simple modular syntax:");

    // Area using Squared marker
    let area1 = AreaModular::<Meter, Squared>::new(25.0);
    println!("   Area: {} (25 m²)", area1.value());

    // Volume using Cubed marker
    let volume1 = VolumeModular::<Meter, Cubed>::new(125.0);
    println!("   Volume: {} (125 m³)", volume1.value());

    // ================================================================================================
    // PER NOTATION
    // ================================================================================================

    println!("\n2. Per notation for fractions:");

    // Velocity: m/s
    let velocity1 = VelocityModular::<Meter, Per<Second>>::new(10.0);
    println!("   Velocity: {} (10 m/s)", velocity1.value());

    // Acceleration: m/s²
    let accel1 = AccelerationModular::<Meter, Per<Second, Squared>>::new(9.81);
    println!("   Acceleration: {} (9.81 m/s²)", accel1.value());

    // ================================================================================================
    // PREFIXED UNITS WITH MODULAR SYNTAX
    // ================================================================================================

    println!("\n3. Prefixed units with modular syntax:");

    // Area in square kilometers
    let area_km = AreaModular::<Prefixed<Kilo, Meter>, Squared>::new(2.5);
    println!("   Area: {} (2.5 km²)", area_km.value());

    // Velocity in km/h (using modular types)
    // Note: This would need Hour unit to be fully functional
    let velocity_kmh = VelocityModular::<Prefixed<Kilo, Meter>, Per<Second>>::new(16.67);
    println!(
        "   Velocity: {} (≈60 km/h converted to km/s)",
        velocity_kmh.value()
    );

    // ================================================================================================
    // COMPLEX COMPOSITIONS
    // ================================================================================================

    println!("\n4. Complex unit compositions:");

    // Density: kg/m³
    let density1 = DensityModular::<Kilogram, Per<Meter, Cubed>>::new(1000.0);
    println!("   Water density: {} (1000 kg/m³)", density1.value());

    // Alternative acceleration syntax using Exponent directly
    let accel2 = AccelerationModular::<Meter, Per<Exponent<Second, 2>>>::new(9.81);
    println!(
        "   Acceleration (alt syntax): {} (9.81 m/s²)",
        accel2.value()
    );

    // ================================================================================================
    // COMPARISON WITH OLD SYNTAX
    // ================================================================================================

    println!("\n5. Comparison with traditional syntax:");

    // Traditional way
    let distance_old = Distance::<Meter>::new(100.0);
    let area_old = distance_old * distance_old;
    println!("   Traditional multiplication: {:?}", area_old);

    // Modular way - more explicit and readable
    let area_new = AreaModular::<Meter, Squared>::new(100.0 * 100.0);
    println!("   Modular declaration: {} (10000 m²)", area_new.value());

    // ================================================================================================
    // UNIT CONVERSIONS
    // ================================================================================================

    println!("\n6. Unit conversions with modular syntax:");

    // Convert between different area units (conceptually)
    let area_m2 = AreaModular::<Meter, Squared>::new(10000.0); // 10000 m²
    let area_km2_value = area_m2.value() / (1000.0 * 1000.0); // Convert to km²
    let area_km2 = AreaModular::<Prefixed<Kilo, Meter>, Squared>::new(area_km2_value);

    println!("   {} m² = {} km²", area_m2.value(), area_km2.value());

    // ================================================================================================
    // PHYSICS CALCULATIONS
    // ================================================================================================

    println!("\n7. Physics calculations with modular units:");

    // Force calculation conceptually
    let mass = Mass::<Kilogram>::new(10.0);
    let acceleration = AccelerationModular::<Meter, Per<Second, Squared>>::new(9.81);

    println!("   Mass: {} kg", mass.value());
    println!("   Acceleration: {} m/s²", acceleration.value());

    // Conceptual force calculation (would need proper implementation)
    let force_value = mass.value() * acceleration.value();
    println!("   Force: {} N (calculated)", force_value);

    // ================================================================================================
    // ASTRONOMICAL EXAMPLES
    // ================================================================================================

    println!("\n8. Astronomical examples:");

    // Solar surface area (conceptual)
    let solar_radius = Distance::<SunRadius>::new(1.0);
    let solar_radius_m = solar_radius.convert_to::<Meter>();
    let solar_area_value = 4.0 * std::f64::consts::PI * solar_radius_m.value().powi(2);
    let solar_area = AreaModular::<Meter, Squared>::new(solar_area_value);

    println!("   Solar surface area: {:.2e} m²", solar_area.value());

    // Orbital velocity
    let orbital_distance = Distance::<AstronomicalUnit>::new(1.0);
    let orbital_period = Time::<Year>::new(1.0);

    let orbital_velocity_value = orbital_distance.to_si() / orbital_period.to_si();
    let orbital_velocity = VelocityModular::<Meter, Per<Second>>::new(orbital_velocity_value);

    println!(
        "   Earth orbital velocity: {:.0} m/s",
        orbital_velocity.value()
    );

    println!("\n=== Test Complete ===");
    println!("\nThe modular syntax provides:");
    println!("✓ Clear intent: Area::<Meter, Squared> vs complex dimensional types");
    println!("✓ Natural notation: Per<Second, Squared> reads like physics");
    println!("✓ Flexible composition: Mix prefixes and exponents easily");
    println!("✓ Type safety: Still prevents unit mixing errors");
}
