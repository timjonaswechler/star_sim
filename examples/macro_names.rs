//! Demonstration of the new macro-based name generation system.
//!
//! This example shows how to use the three new macros:
//! - `define_symbol_profile!` - Custom sound sets
//! - `define_phonetic_rules!` - Sound compatibility rules  
//! - `define_name_category!` - Complete name categories
//!
//! Run with: cargo run --example macro_names

use rand::{SeedableRng, thread_rng};
use rand_chacha::ChaCha8Rng;
use star_sim::utilities::name_generator::symbol_types::SymbolMapDefinition;
use star_sim::utilities::name_generator::{Name, NameCategory};
use star_sim::{define_name_category, define_phonetic_rules, define_symbol_profile};

// Define custom symbol profiles for different star types
define_symbol_profile! {
    Draconic {
        syllables: ["aer", "ash", "baham", "drak", "faf", "grav", "khar", "lich", "morg", "nym", "orth", "shar", "thar", "urth", "vash", "wyv", "zar"],
        simple_vowels: ["a", "e", "o", "u"],
        complex_vowels: ["a", "e", "o", "u", "ae", "au", "ou"],
        simple_consonants: ["k", "g", "r", "th", "v", "z", "x", "h"],
        beginning_clusters: ["dr", "gr", "kr", "th", "shr", "zr"],
        ending_clusters: ["rk", "th", "gh", "ng", "x", "z"],
    }
}

define_symbol_profile! {
    Celestial {
        syllables: ["astra", "luna", "sol", "stella", "nova", "cosm", "gala", "orb", "lum", "radi", "bril", "shin"],
        simple_vowels: ["a", "e", "i", "o", "u"],
        complex_vowels: ["a", "e", "i", "o", "u", "ia", "io", "ea", "au", "ei"],
        simple_consonants: ["l", "n", "r", "s", "t", "m"],
        beginning_clusters: ["st", "br", "gl", "cr", "pr", "tr"],
        ending_clusters: ["st", "nd", "nt", "ll", "ss", "ra"],
    }
}

// Define phonetic rules for different sound profiles
define_phonetic_rules! {
    DarkRules {
        forbidden_sequences: ["ii", "ee", "ll"],
        preferred_sequences: ["th", "kr", "gh", "rk"],
        vowel_consonant_compatibility: {
            'a' => ['r', 'k', 'g'],
            'o' => ['r', 'k', 'g'],
            'u' => ['r', 'g', 'z'],
        },
        consonant_vowel_compatibility: {
            'r' => ['a', 'o', 'u'],
            'k' => ['a', 'o'],
            'g' => ['a', 'o', 'u'],
        },
        max_consecutive_vowels: 2,
        max_consecutive_consonants: 2,
        preferred_weight_multiplier: 2.0,
        compatible_weight_multiplier: 1.5,
    }
}

define_phonetic_rules! {
    BrightRules {
        forbidden_sequences: ["kk", "gg", "xx"],
        preferred_sequences: ["st", "br", "cr", "ll"],
        vowel_consonant_compatibility: {
            'a' => ['l', 'n', 'r', 's'],
            'e' => ['l', 'n', 'r'],
            'i' => ['l', 'n', 's'],
            'o' => ['l', 'n', 'r'],
        },
        consonant_vowel_compatibility: {
            'l' => ['a', 'e', 'i', 'o', 'u'],
            'n' => ['a', 'e', 'i', 'o'],
            'r' => ['a', 'e', 'i', 'o'],
            's' => ['a', 'e', 'i', 'o'],
        },
        max_consecutive_vowels: 3,
        max_consecutive_consonants: 2,
        preferred_weight_multiplier: 1.8,
        compatible_weight_multiplier: 1.3,
    }
}

// Define name categories using the profiles and rules
define_name_category! {
    DraconicStar {
        pattern: "<!s><v><c><s>",
        symbol_profile: Draconic,
        phonetic_rules: DarkRules,
    }
}

define_name_category! {
    CelestialStar {
        pattern: "<!s><v><c><v><s>",
        symbol_profile: Celestial,
        phonetic_rules: BrightRules,
    }
}

define_name_category! {
    SimpleStar {
        pattern: "<!s><v><c>",
    }
}

define_name_category! {
    CelestialOnlyStar {
        pattern: "<!s><v><c><v><s>",
        symbol_profile: Celestial,
    }
}

fn main() {
    println!("🌟 Star Name Generator - New Macro System Demo");
    println!("================================================\n");

    let mut rng = ChaCha8Rng::seed_from_u64(42); // Deterministic for consistent demo

    // Generate names using different categories
    println!("🐉 Draconic Stars (Dark Symbol Profile + Dark Rules):");
    for i in 1..=8 {
        let name = Name::<DraconicStar>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\n✨ Celestial Stars (Bright Symbol Profile + Bright Rules):");
    for i in 1..=8 {
        let name = Name::<CelestialStar>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\n🌟 Simple Stars (Default Symbols + No Rules):");
    for i in 1..=5 {
        let name = Name::<SimpleStar>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\n🌌 Celestial Only Stars (Bright Symbols, No Rules):");
    for i in 1..=5 {
        let name = Name::<CelestialOnlyStar>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\n📊 Macro System Advantages:");
    println!("  ✅ Define symbol profiles once, reuse everywhere");
    println!("  ✅ Define phonetic rules once, mix and match");
    println!("  ✅ Create categories with just a few lines");
    println!("  ✅ Type-safe compilation guarantees");
    println!("  ✅ Consistent with units system design");

    println!("\n🔧 Usage Summary:");
    println!("  define_symbol_profile!   - Creates custom sound sets");
    println!("  define_phonetic_rules!   - Creates sound compatibility rules");
    println!("  define_name_category!    - Creates complete name categories");
    println!("  Name::<Category>::new()  - Generate names");
}
