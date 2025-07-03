//! Example categories demonstrating different phonetic approaches.
//!
//! This module contains example implementations showing the three main approaches
//! to creating distinct sound profiles:
//! 1. Symbol-only approach: Custom symbol maps define the sound
//! 2. Rules-only approach: Standard symbols + phonetic rules filter the sound
//! 3. Hybrid approach: Custom symbols + additional rules for maximum control

use super::super::{
    NameCategory,
    phonetic_rules::{PhoneticRules, profiles},
    symbol_types::{SymbolMapDefinition, create_symbol_map},
    symbols::{BRIGHT_SYMBOL_MAP, DARK_SYMBOL_MAP, EXOTIC_SYMBOL_MAP, SYMBOL_MAP},
};
use std::collections::HashMap;

// ============================================================================
// APPROACH 1: Symbol-only (Pre-filtered sound sets)
// ============================================================================

/// Dark star names using only dark symbol map
///
/// This approach relies entirely on a curated symbol map containing only dark sounds.
/// No phonetic rules needed - the sound profile is built into the symbol selection.
pub struct DarkStarSymbolOnly;

impl Default for DarkStarSymbolOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for DarkStarSymbolOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c>" // Simple pattern: Capitalized syllable + vowel + consonant
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &DARK_SYMBOL_MAP // Use only dark sounds
    }

    // No phonetic rules needed - symbol map handles the sound profile
}

/// Bright star names using only bright symbol map
pub struct BrightStarSymbolOnly;

impl Default for BrightStarSymbolOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for BrightStarSymbolOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><v><s>" // Longer pattern: Cap syllable + vowel + consonant + vowel + syllable
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &BRIGHT_SYMBOL_MAP // Use only bright sounds
    }
}

/// Exotic alien names using only exotic symbol map
pub struct ExoticAlienSymbolOnly;

impl Default for ExoticAlienSymbolOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for ExoticAlienSymbolOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><V><c><s>" // Complex vowels for alien feel
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &EXOTIC_SYMBOL_MAP // Use only exotic sounds
    }
}

// ============================================================================
// APPROACH 2: Rules-only (Standard symbols + phonetic filtering)
// ============================================================================

/// Dark star names using standard symbols + dark phonetic rules
///
/// This approach uses the full standard symbol map but applies phonetic rules
/// to dynamically favor dark sounds and avoid bright combinations.
pub struct DarkStarRulesOnly;

impl Default for DarkStarRulesOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for DarkStarRulesOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<s><v><c><s>" // Standard pattern with all symbols available
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &SYMBOL_MAP // Use standard symbol map (all sounds available)
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::DARK_RULES) // Apply dark phonetic rules to filter
    }
}

/// Bright star names using standard symbols + bright phonetic rules
pub struct BrightStarRulesOnly;

impl Default for BrightStarRulesOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for BrightStarRulesOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<s><v><c><v><s>" // Pattern allowing for flowing combinations
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &SYMBOL_MAP // Use standard symbol map
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::BRIGHT_RULES) // Apply bright phonetic rules
    }
}

/// Exotic alien names using standard symbols + exotic phonetic rules
pub struct ExoticAlienRulesOnly;

impl Default for ExoticAlienRulesOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for ExoticAlienRulesOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<s><V><c><V><s>" // Complex vowels with unusual combinations
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &SYMBOL_MAP // Use standard symbol map
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::EXOTIC_RULES) // Apply exotic phonetic rules
    }
}

// ============================================================================
// APPROACH 3: Hybrid (Custom symbols + additional rules)
// ============================================================================

/// Ultra-dark star names using dark symbols + extra-strict dark rules
///
/// This approach combines a curated dark symbol map with additional phonetic rules
/// for maximum control over the sound profile. Useful when you want to ensure
/// the darkest possible sound while still having fine-grained control.
pub struct UltraDarkStarHybrid;

impl Default for UltraDarkStarHybrid {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for UltraDarkStarHybrid {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><s>" // Dark syllable pattern
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &DARK_SYMBOL_MAP // Start with dark symbols
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::DARK_RULES) // Add additional dark rules for extra filtering
    }
}

/// Ultra-bright star names using bright symbols + extra-flowing bright rules
pub struct UltraBrightStarHybrid;

impl Default for UltraBrightStarHybrid {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for UltraBrightStarHybrid {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><v><c><v><s>" // Long flowing pattern
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &BRIGHT_SYMBOL_MAP // Start with bright symbols
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::BRIGHT_RULES) // Add additional bright rules
    }
}

/// Ultra-exotic alien names using exotic symbols + alien rules
pub struct UltraExoticAlienHybrid;

impl Default for UltraExoticAlienHybrid {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for UltraExoticAlienHybrid {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><V><c><V><c><V>" // Maximum exotic complexity
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &EXOTIC_SYMBOL_MAP // Start with exotic symbols
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::EXOTIC_RULES) // Add additional exotic rules
    }
}

// ============================================================================
// COMPARISON CATEGORY (for testing)
// ============================================================================

/// Standard star names (no customization)
///
/// This category uses the default implementation for comparison purposes.
/// It shows what names look like without any phonetic customization.
pub struct StandardStar;

impl Default for StandardStar {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for StandardStar {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<s><v><c>" // Simple standard pattern
    }

    // Uses default symbol map and no phonetic rules
}

// ============================================================================
// ADVANCED: Custom type-safe symbol definitions
// ============================================================================

/// Example of creating a custom type-safe symbol map for "Draconic" names
///
/// This demonstrates how to create your own symbol definitions that guarantee
/// all required symbols (s, v, V, c, B, C) are defined.
pub struct DraconicSymbols;

impl SymbolMapDefinition for DraconicSymbols {
    fn syllables() -> Vec<&'static str> {
        vec![
            "aer", "ash", "baham", "drak", "faf", "grav", "khar", "lich", "morg", "nym", "orth",
            "pyr", "quin", "rath", "shar", "thar", "urth", "vash", "wyv", "xer", "yth", "zar",
            "zeph", "anc", "arv", "fel", "gor", "hex", "itz", "jor",
        ]
    }

    fn simple_vowels() -> Vec<&'static str> {
        vec!["a", "e", "o", "u", "y"] // No "i" for deeper sound
    }

    fn complex_vowels() -> Vec<&'static str> {
        vec![
            "a", "e", "o", "u", "y", "ae", "au", "ay", "ea", "ou", "ya", "yo",
        ]
    }

    fn simple_consonants() -> Vec<&'static str> {
        vec![
            "k", "g", "r", "th", "v", "z", "x", "h", "n", "m", "s", "t", "f",
        ]
    }

    fn beginning_clusters() -> Vec<&'static str> {
        vec!["dr", "gr", "kr", "th", "shr", "zr", "vr", "kh", "gh"]
    }

    fn ending_clusters() -> Vec<&'static str> {
        vec!["rk", "th", "gh", "ng", "x", "z", "ck", "rn", "rm", "rv"]
    }
}

// Create the symbol map for Draconic names
create_symbol_map!(DRACONIC_SYMBOL_MAP, DraconicSymbols);

/// Draconic star names using custom type-safe symbol definition
pub struct DraconicStar;

impl Default for DraconicStar {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for DraconicStar {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><s>" // Pattern optimized for draconic sounds
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &DRACONIC_SYMBOL_MAP // Use our custom type-safe symbol map
    }

    // Could also add phonetic rules here for even more control
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::name_generator::Name;
    use rand::{SeedableRng, thread_rng};
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_different_approaches_produce_different_sounds() {
        let mut rng = ChaCha8Rng::seed_from_u64(42); // Deterministic for testing

        // Generate multiple names from each category
        let dark_symbol_names: Vec<String> = (0..10)
            .map(|_| Name::<DarkStarSymbolOnly>::new().generate(&mut rng))
            .collect();

        let bright_symbol_names: Vec<String> = (0..10)
            .map(|_| Name::<BrightStarSymbolOnly>::new().generate(&mut rng))
            .collect();

        let dark_rules_names: Vec<String> = (0..10)
            .map(|_| Name::<DarkStarRulesOnly>::new().generate(&mut rng))
            .collect();

        // Print results for manual inspection
        println!("Dark Symbol-Only Names: {:?}", dark_symbol_names);
        println!("Bright Symbol-Only Names: {:?}", bright_symbol_names);
        println!("Dark Rules-Only Names: {:?}", dark_rules_names);

        // Basic checks
        assert!(!dark_symbol_names.is_empty());
        assert!(!bright_symbol_names.is_empty());
        assert!(!dark_rules_names.is_empty());

        // Names should be different (very unlikely to be identical with good randomness)
        assert_ne!(dark_symbol_names, bright_symbol_names);
    }

    #[test]
    fn test_hybrid_approach() {
        let mut rng = thread_rng();

        // Test hybrid approaches generate valid names
        let ultra_dark_name = Name::<UltraDarkStarHybrid>::new().generate(&mut rng);
        let ultra_bright_name = Name::<UltraBrightStarHybrid>::new().generate(&mut rng);
        let ultra_exotic_name = Name::<UltraExoticAlienHybrid>::new().generate(&mut rng);

        assert!(!ultra_dark_name.is_empty());
        assert!(!ultra_bright_name.is_empty());
        assert!(!ultra_exotic_name.is_empty());

        println!("Ultra Dark: {}", ultra_dark_name);
        println!("Ultra Bright: {}", ultra_bright_name);
        println!("Ultra Exotic: {}", ultra_exotic_name);
    }

    #[test]
    fn test_standard_comparison() {
        let mut rng = thread_rng();

        // Test standard category still works
        let standard_name = Name::<StandardStar>::new().generate(&mut rng);
        assert!(!standard_name.is_empty());

        println!("Standard: {}", standard_name);
    }

    #[test]
    fn test_custom_type_safe_symbols() {
        let mut rng = thread_rng();

        // Test custom draconic symbol definition
        let draconic_name = Name::<DraconicStar>::new().generate(&mut rng);
        assert!(!draconic_name.is_empty());

        println!("Draconic: {}", draconic_name);

        // Test that the symbol map has all required symbols
        assert!(DRACONIC_SYMBOL_MAP.contains_key("s"));
        assert!(DRACONIC_SYMBOL_MAP.contains_key("v"));
        assert!(DRACONIC_SYMBOL_MAP.contains_key("V"));
        assert!(DRACONIC_SYMBOL_MAP.contains_key("c"));
        assert!(DRACONIC_SYMBOL_MAP.contains_key("B"));
        assert!(DRACONIC_SYMBOL_MAP.contains_key("C"));

        // Test that symbols are not empty
        assert!(!DRACONIC_SYMBOL_MAP["s"].is_empty());
        assert!(!DRACONIC_SYMBOL_MAP["v"].is_empty());
        assert!(!DRACONIC_SYMBOL_MAP["V"].is_empty());
        assert!(!DRACONIC_SYMBOL_MAP["c"].is_empty());
        assert!(!DRACONIC_SYMBOL_MAP["B"].is_empty());
        assert!(!DRACONIC_SYMBOL_MAP["C"].is_empty());
    }
}
