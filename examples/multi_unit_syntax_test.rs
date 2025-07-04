use star_sim::physics::units::*;

fn main() {
    println!("=== Multi-Unit Syntax Test ===\n");
    
    // Test the new dual-unit syntax
    println!("1. Testing dual-unit syntax:");
    
    // Velocity: Distance/Time = m/s
    let velocity = VelocityNew::<Meter, Second>::new(10.0);
    println!("   Velocity: {} (should be 10 m/s)", velocity);
    
    // Acceleration: Distance/Time² = m/s²
    let acceleration = AccelerationNew::<Meter, Second>::new(9.81);
    println!("   Acceleration: {} (should be 9.81 m/s²)", acceleration);
    
    println!("\n2. Testing with different units:");
    
    // TODO: Add Force implementation later
    // let force = ForceNew::<Kilogram, Meter, Second>::new(98.1);
    // println!("   Force: {} (should be 98.1 kg⋅m/s²)", force);
    
    println!("\n3. Testing with different units:");
    
    // Test automatic conversions and compatibility
    let distance = Distance::<Meter>::new(10.0);
    let time = Time::<Second>::new(2.0);
    
    println!("   Distance: {}", distance);
    println!("   Time: {}", time);
    
    // The old arithmetic system should still work
    let calculated_velocity = distance / time;
    println!("   Calculated velocity (old system): {}", calculated_velocity);
    
    println!("\n=== Testing new multi-unit syntax completed! ===");
}