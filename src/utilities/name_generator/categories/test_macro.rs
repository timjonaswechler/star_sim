//! Test file for the new macro system
//!
//! This file demonstrates how to use the new three-macro system and serves as a test
//! to ensure the macros work correctly.

use crate::utilities::name_generator::{NameCategory, Name};
use crate::utilities::name_generator::symbol_types::SymbolMapDefinition;
use crate::{define_symbol_profile, define_phonetic_rules, define_name_category};

// Test 1: Define a simple symbol profile
define_symbol_profile! {
    TestDark {
        syllables: ["aer", "ash", "drak", "grav", "morg", "nym"],
        simple_vowels: ["a", "e", "o", "u"],
        complex_vowels: ["a", "e", "o", "u", "au", "ou"],
        simple_consonants: ["k", "g", "r", "th", "v", "z"],
        beginning_clusters: ["dr", "gr", "kr", "th"],
        ending_clusters: ["rk", "th", "gh", "ng"],
    }
}

// Test 2: Define phonetic rules
define_phonetic_rules! {
    TestDarkRules {
        forbidden_sequences: ["ii", "ee", "ll"],
        preferred_sequences: ["th", "kr", "gh"],
        vowel_consonant_compatibility: {
            'a' => ['r', 'k', 'g'],
            'o' => ['r', 'k'],
        },
        consonant_vowel_compatibility: {
            'r' => ['a', 'o', 'u'],
            'k' => ['a', 'o'],
        },
        max_consecutive_vowels: 2,
        max_consecutive_consonants: 2,
        preferred_weight_multiplier: 2.0,
        compatible_weight_multiplier: 1.5,
    }
}

// Test 3: Define categories using the new macros
define_name_category! {
    TestDarkStar {
        pattern: "<!s><v><c>",
        symbol_profile: TestDark,
        phonetic_rules: TestDarkRules,
    }
}

define_name_category! {
    TestSimpleStar {
        pattern: "<!s><v><c>",
    }
}

define_name_category! {
    TestSymbolOnlyStar {
        pattern: "<!s><v><c>",
        symbol_profile: TestDark,
    }
}

define_name_category! {
    TestRulesOnlyStar {
        pattern: "<!s><v><c>",
        phonetic_rules: TestDarkRules,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_macro_generated_categories() {
        let mut rng = thread_rng();

        // Test all category variants
        let dark_name = Name::<TestDarkStar>::new().generate(&mut rng);
        let simple_name = Name::<TestSimpleStar>::new().generate(&mut rng);
        let symbol_only_name = Name::<TestSymbolOnlyStar>::new().generate(&mut rng);
        let rules_only_name = Name::<TestRulesOnlyStar>::new().generate(&mut rng);

        // All should generate non-empty names
        assert!(!dark_name.is_empty());
        assert!(!simple_name.is_empty());
        assert!(!symbol_only_name.is_empty());
        assert!(!rules_only_name.is_empty());

        println!("Dark Star: {}", dark_name);
        println!("Simple Star: {}", simple_name);
        println!("Symbol Only Star: {}", symbol_only_name);
        println!("Rules Only Star: {}", rules_only_name);
    }

    #[test]
    fn test_symbol_map_generation() {
        // Test that the symbol map was generated correctly
        assert!(TESTDARK_SYMBOL_MAP.contains_key("s"));
        assert!(TESTDARK_SYMBOL_MAP.contains_key("v"));
        assert!(TESTDARK_SYMBOL_MAP.contains_key("V"));
        assert!(TESTDARK_SYMBOL_MAP.contains_key("c"));
        assert!(TESTDARK_SYMBOL_MAP.contains_key("B"));
        assert!(TESTDARK_SYMBOL_MAP.contains_key("C"));

        // Test that syllables are included
        assert!(TESTDARK_SYMBOL_MAP["s"].contains(&"aer"));
        assert!(TESTDARK_SYMBOL_MAP["s"].contains(&"drak"));
        assert!(TESTDARK_SYMBOL_MAP["s"].contains(&"morg"));
    }

    #[test]
    fn test_phonetic_rules_generation() {
        // Test that phonetic rules were generated correctly
        assert_eq!(TESTDARKRULES.name, "TestDarkRules");
        assert!(TESTDARKRULES.forbidden_sequences.contains(&"ii"));
        assert!(TESTDARKRULES.preferred_sequences.contains(&"th"));
        
        // Test compatibility maps
        assert!(TESTDARKRULES.vowel_consonant_compatibility.contains_key(&'a'));
        assert!(TESTDARKRULES.consonant_vowel_compatibility.contains_key(&'r'));
    }
}