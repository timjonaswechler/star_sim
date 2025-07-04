use physics_units::*;
use physics_units::variadic::{
    Velocity as VelocityVariadic, 
    Acceleration as AccelerationVariadic,
    Force as ForceVariadic, 
    Energy as EnergyVariadic,
    Power as PowerVariadic,
    Area as AreaVariadic,
    Volume as VolumeVariadic
};

fn main() {
    println!("=== Variadic Multi-Unit Syntax Test ===\n");
    
    // ================================================================================================
    // TWO UNITS: Length/Time, Length/Time², Area
    // ================================================================================================
    
    println!("1. Two-unit syntax:");
    
    // Velocity: Distance/Time = m/s
    let velocity = VelocityVariadic::<Meter, Second>::new(10.0);
    println!("   Velocity: {} (10 m/s)", velocity);
    
    // Acceleration: Distance/Time² = m/s²  
    let acceleration = AccelerationVariadic::<Meter, Second>::new(9.81);
    println!("   Acceleration: {} (9.81 m/s²)", acceleration);
    
    // Area: Length² = m²
    let area = AreaVariadic::<Meter, Meter>::new(25.0);
    println!("   Area: {} (25 m²)", area);
    
    // ================================================================================================
    // THREE UNITS: Mass×Length/Time², Volume
    // ================================================================================================
    
    println!("\n2. Three-unit syntax:");
    
    // Force: Mass×Distance/Time² = kg⋅m/s²
    let force = ForceVariadic::<Kilogram, Meter, Second>::new(98.1);
    println!("   Force: {} (98.1 kg⋅m/s²)", force);
    
    // Volume: Length³ = m³
    let volume = VolumeVariadic::<Meter, Meter, Meter>::new(125.0);
    println!("   Volume: {} (125 m³)", volume);
    
    // ================================================================================================
    // THREE UNITS: Smart dimensional inference
    // ================================================================================================
    
    println!("\n3. Three-unit syntax with smart dimensions:");
    
    // Energy: Mass×Distance×Time = kg⋅m²/s² (automatically inferred)
    let energy = EnergyVariadic::<Kilogram, Meter, Second>::new(500.0);
    println!("   Energy: {} (500 kg⋅m²/s²)", energy);
    
    // Power: Mass×Distance×Time = kg⋅m²/s³ (automatically inferred)
    let power = PowerVariadic::<Kilogram, Meter, Second>::new(1000.0);
    println!("   Power: {} (1000 kg⋅m²/s³)", power);
    
    // ================================================================================================
    // COMPATIBILITY TEST
    // ================================================================================================
    
    println!("\n4. Compatibility with old system:");
    
    let old_distance = Distance::<Meter>::new(10.0);
    let old_time = Time::<Second>::new(2.0);
    let old_velocity = old_distance / old_time;
    
    println!("   Old system velocity: {} (5 m/s)", old_velocity);
    println!("   New system velocity: {} (10 m/s)", velocity);
    
    // ================================================================================================
    // PREFIXES TEST (if available)
    // ================================================================================================
    
    println!("\n5. Different units (same dimension):");
    
    // Different velocity units
    let velocity_kmh = VelocityVariadic::<Prefixed<Kilo, Meter>, Hour>::new(36.0); // Should be 10 m/s equivalent
    println!("   Velocity in km/h: {} (36 km/h)", velocity_kmh);
    
    let area_km2 = AreaVariadic::<Prefixed<Kilo, Meter>, Prefixed<Kilo, Meter>>::new(0.000025); // Should be 25 m²
    println!("   Area in km²: {} (0.000025 km²)", area_km2);
    
    println!("\n=== Variadic syntax test completed! ===");
}