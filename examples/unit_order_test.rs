use physics_units::*;
use physics_units::variadic::{
    Energy as EnergyVariadic,
    Force as ForceVariadic
};

fn main() {
    println!("=== Unit Order Test ===\n");
    
    // Test different orders for Energy (should be kg⋅m²/s²)
    println!("Energy with different unit orders:");
    
    // Order 1: Kilogram, Meter, Second
    let energy1 = EnergyVariadic::<Kilogram, Meter, Second>::new(500.0);
    println!("   Energy<Kg, m, s>: {}", energy1);
    
    // Order 2: Meter, Kilogram, Second  
    let energy2 = EnergyVariadic::<Meter, Kilogram, Second>::new(500.0);
    println!("   Energy<m, Kg, s>: {}", energy2);
    
    // Order 3: Second, Meter, Kilogram
    let energy3 = EnergyVariadic::<Second, Meter, Kilogram>::new(500.0);
    println!("   Energy<s, m, Kg>: {}", energy3);
    
    println!("\nForce with different unit orders:");
    
    // Order 1: Kilogram, Meter, Second
    let force1 = ForceVariadic::<Kilogram, Meter, Second>::new(98.1);
    println!("   Force<Kg, m, s>: {}", force1);
    
    // Order 2: Meter, Kilogram, Second
    let force2 = ForceVariadic::<Meter, Kilogram, Second>::new(98.1);
    println!("   Force<m, Kg, s>: {}", force2);
    
    // Order 3: Second, Kilogram, Meter
    let force3 = ForceVariadic::<Second, Kilogram, Meter>::new(98.1);
    println!("   Force<s, Kg, m>: {}", force3);
    
    println!("\n=== Order Test Completed ===");
}