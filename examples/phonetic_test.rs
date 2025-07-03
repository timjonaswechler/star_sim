//! Test example for the phonetic name generation system.
//!
//! This example demonstrates the different approaches to creating distinct sound profiles
//! and shows how phonetic rules affect the generated names.

use star_sim::utilities::name_generator::{
    Name,
    categories::examples::*,
};
use rand::{SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;

fn main() {
    println!("=== Phonetic Name Generation Test ===\n");
    
    // Use deterministic RNG for consistent results in testing
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    
    println!("Approach 1: Symbol-only (Pre-filtered sound sets)\n");
    
    println!("Dark Star Names (Dark Symbol Map):");
    for i in 1..=10 {
        let name = Name::<DarkStarSymbolOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nBright Star Names (Bright Symbol Map):");
    for i in 1..=10 {
        let name = Name::<BrightStarSymbolOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nExotic Alien Names (Exotic Symbol Map):");
    for i in 1..=10 {
        let name = Name::<ExoticAlienSymbolOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\n{}", "=".repeat(60));
    println!("Approach 2: Rules-only (Standard symbols + phonetic filtering)\n");
    
    println!("Dark Star Names (Standard Symbols + Dark Rules):");
    for i in 1..=10 {
        let name = Name::<DarkStarRulesOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nBright Star Names (Standard Symbols + Bright Rules):");
    for i in 1..=10 {
        let name = Name::<BrightStarRulesOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nExotic Alien Names (Standard Symbols + Exotic Rules):");
    for i in 1..=10 {
        let name = Name::<ExoticAlienRulesOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\n{}", "=".repeat(60));
    println!("Approach 3: Hybrid (Custom symbols + additional rules)\n");
    
    println!("Ultra-Dark Star Names (Dark Symbols + Dark Rules):");
    for i in 1..=10 {
        let name = Name::<UltraDarkStarHybrid>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nUltra-Bright Star Names (Bright Symbols + Bright Rules):");
    for i in 1..=10 {
        let name = Name::<UltraBrightStarHybrid>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nUltra-Exotic Alien Names (Exotic Symbols + Exotic Rules):");
    for i in 1..=10 {
        let name = Name::<UltraExoticAlienHybrid>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\n{}", "=".repeat(60));
    println!("Comparison: Standard Names (No customization)\n");
    
    println!("Standard Star Names:");
    for i in 1..=10 {
        let name = Name::<StandardStar>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\n{}", "=".repeat(60));
    println!("Random Seed Test (demonstrating variety)\n");
    
    let mut random_rng = thread_rng();
    
    println!("Random Dark Names:");
    for i in 1..=5 {
        let name = Name::<DarkStarSymbolOnly>::new().generate(&mut random_rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nRandom Bright Names:");
    for i in 1..=5 {
        let name = Name::<BrightStarSymbolOnly>::new().generate(&mut random_rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\n{}", "=".repeat(60));
    println!("Advanced: Custom Type-Safe Symbol Definition\n");
    
    println!("Draconic Star Names (Custom Symbol Definition):");
    for i in 1..=10 {
        let name = Name::<DraconicStar>::new().generate(&mut random_rng);
        println!("  {}. {}", i, name);
    }
    
    println!("\nTest completed! Notice the different sound profiles:");
    println!("- Dark names: Deep vowels (a, o, u), hard consonants (k, g, r)");
    println!("- Bright names: Light vowels (e, i), soft consonants (l, n, s)");
    println!("- Exotic names: Unusual combinations with x, z, q");
    println!("- Draconic names: Fantasy sounds with draconic syllables (baham, drak, wyv)");
    println!("- Standard names: Mixed sounds without preferences");
    
    println!("\nType Safety Benefits:");
    println!("- All symbol maps are guaranteed to have s, v, V, c, B, C symbols");
    println!("- Compile-time verification prevents missing symbol definitions");
    println!("- Easy to create new custom sound profiles with guaranteed completeness");
}