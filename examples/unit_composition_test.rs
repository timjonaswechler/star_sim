//! Test and demonstration of the advanced unit composition system.
//!
//! This example shows how to use the new unit composition features:
//! - Power operations (squared, cubed)
//! - Automatic dimensional analysis
//! - Prefixed power units
//! - Type-safe unit arithmetic

use star_sim::physics::units::*;

fn main() {
    println!("=== Unit Composition System Test ===\n");

    // ================================================================================================
    // BASIC UNIT OPERATIONS
    // ================================================================================================

    println!("1. Basic unit operations:");

    let distance1 = Distance::<Meter>::new(100.0);
    let distance2 = Distance::<Meter>::new(50.0);

    println!("   Distance 1: {}", distance1);
    println!("   Distance 2: {}", distance2);

    // Addition/Subtraction (same dimensions)
    let sum = distance1 + distance2;
    let diff = distance1 - distance2;

    println!("   Sum: {}", sum);
    println!("   Difference: {}", diff);

    // ================================================================================================
    // POWER OPERATIONS
    // ================================================================================================

    println!("\n2. Power operations:");

    let radius = Distance::<Meter>::new(5.0);
    println!("   Radius: {}", radius);

    // Note: These operations might not work yet due to missing ToSI/FromSI implementations
    // let area = radius.squared();
    // let volume = radius.cubed();
    // println!("   Area: {}", area);
    // println!("   Volume: {}", volume);

    // ================================================================================================
    // AUTOMATIC DIMENSIONAL ANALYSIS
    // ================================================================================================

    println!("\n3. Automatic dimensional analysis:");

    // Try multiplication and division
    let dist1 = Distance::<Meter>::new(10.0);
    let dist2 = Distance::<Meter>::new(5.0);

    println!("   Distance 1: {}", dist1);
    println!("   Distance 2: {}", dist2);

    // This should create an area-like quantity
    let area_result = dist1 * dist2;
    println!("   Multiplication result: {:?}", area_result);

    // This should create a dimensionless ratio
    let ratio_result = dist1 / dist2;
    println!("   Division result: {:?}", ratio_result);

    // ================================================================================================
    // PREFIXED UNITS
    // ================================================================================================

    println!("\n4. Prefixed units:");

    let km_distance = Distance::<Prefixed<Kilo, Meter>>::new(2.0);
    println!("   Distance in km: {}", km_distance);

    let m_distance = km_distance.convert_to::<Meter>();
    println!("   Same distance in meters: {}", m_distance);

    // ================================================================================================
    // PHYSICS CALCULATIONS
    // ================================================================================================

    println!("\n5. Physics calculations:");

    // Velocity calculation: Distance / Time
    let distance = Distance::<Meter>::new(100.0);
    let time = Time::<Second>::new(10.0);

    println!("   Distance: {}", distance);
    println!("   Time: {}", time);

    let velocity = distance / time;
    println!("   Velocity: {:?}", velocity);

    // Force calculation: Mass × Acceleration
    let mass = Mass::<Kilogram>::new(10.0);
    let acceleration = Acceleration::<MeterPerSecondSquared>::new(9.81);

    println!("   Mass: {}", mass);
    println!("   Acceleration: {}", acceleration);

    // Using helper function since automatic multiplication not fully implemented yet
    let force_si = multiply_quantities(mass, acceleration);
    println!("   Force (SI): {} N", force_si);

    // ================================================================================================
    // COMPLEX UNIT COMPOSITIONS
    // ================================================================================================

    println!("\n6. Complex unit compositions:");

    // Energy = Force × Distance
    let force_n = Force::<Newton>::new(50.0);
    let distance_m = Distance::<Meter>::new(10.0);

    println!("   Force: {}", force_n);
    println!("   Distance: {}", distance_m);

    // Using helper functions since automatic multiplication not fully implemented yet
    let energy_si = multiply_quantities(force_n, distance_m);
    println!("   Energy (SI): {} J", energy_si);

    // Power = Energy / Time
    let time_s = Time::<Second>::new(5.0);
    let power_si = energy_si / time_s.to_si();
    println!("   Power (SI): {} W", power_si);

    // ================================================================================================
    // ASTRONOMICAL CALCULATIONS
    // ================================================================================================

    println!("\n7. Astronomical calculations:");

    // Stellar radius and surface area
    let sun_radius = Distance::<SunRadius>::new(1.0);
    println!("   Sun radius: {}", sun_radius);

    let sun_radius_m = sun_radius.convert_to::<Meter>();
    println!("   Sun radius in meters: {}", sun_radius_m);

    // Area calculation
    let surface_area = sun_radius_m * sun_radius_m;
    println!("   Surface area calculation: {:?}", surface_area);

    // Orbital mechanics
    let orbital_distance = Distance::<AstronomicalUnit>::new(1.0);
    let orbital_time = Time::<Year>::new(1.0);

    println!("   Orbital distance: {}", orbital_distance);
    println!("   Orbital time: {}", orbital_time);

    let orbital_velocity = orbital_distance / orbital_time;
    println!("   Orbital velocity: {:?}", orbital_velocity);

    println!("\n=== Test Complete ===");
}
