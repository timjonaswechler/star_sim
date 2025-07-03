//! Phonetic rules for context-aware name generation.
//!
//! This module provides phonetic rules that can dynamically adjust the probability
//! of sound combinations based on linguistic compatibility and desired sound profiles.

use std::collections::HashMap;

/// Phonetic rules that define how sounds should be weighted based on context
///
/// These rules allow for dynamic adjustment of sound probabilities during name generation,
/// enabling different sound profiles (dark, bright, exotic, etc.) without requiring
/// separate symbol maps.
#[derive(Debug, Clone)]
pub struct PhoneticRules {
    /// Name identifier for this rule set
    pub name: &'static str,

    /// Sequences that should never appear in generated names
    /// Example: ["kk", "gg", "pp"] to prevent double consonants
    pub forbidden_sequences: Vec<&'static str>,

    /// Sequences that should be preferred and get higher weight
    /// Example: ["st", "ch", "th"] for more natural combinations
    pub preferred_sequences: Vec<&'static str>,

    /// Mapping from vowels to consonants that sound good after them
    /// Example: 'a' -> ['r', 'k', 'g'] for darker sounds
    pub vowel_consonant_compatibility: HashMap<char, Vec<char>>,

    /// Mapping from consonants to vowels that sound good after them
    /// Example: 'r' -> ['a', 'o', 'u'] for darker vowels
    pub consonant_vowel_compatibility: HashMap<char, Vec<char>>,

    /// Maximum number of consecutive vowels allowed
    pub max_consecutive_vowels: usize,

    /// Maximum number of consecutive consonants allowed
    pub max_consecutive_consonants: usize,

    /// Weight multiplier for preferred sound combinations
    pub preferred_weight_multiplier: f32,

    /// Weight multiplier for compatible sound combinations
    pub compatible_weight_multiplier: f32,
}

impl PhoneticRules {
    /// Calculate the weight for a potential next sound based on current context
    ///
    /// Returns a weight between 0.0 and higher values:
    /// - 0.0: Forbidden combination
    /// - 1.0: Neutral combination
    /// - >1.0: Preferred combination
    pub fn calculate_weight(&self, context: &str, next_option: &str) -> f32 {
        if context.is_empty() {
            return 1.0; // No context to judge by
        }

        let mut weight = 1.0;
        let test_sequence = format!("{}{}", context, next_option);

        // Check for forbidden sequences
        for forbidden in &self.forbidden_sequences {
            if test_sequence.contains(forbidden) {
                return 0.0; // Completely forbidden
            }
        }

        // Check for preferred sequences
        for preferred in &self.preferred_sequences {
            if test_sequence.contains(preferred) {
                weight *= self.preferred_weight_multiplier;
            }
        }

        // Check phonetic compatibility
        if let Some(last_char) = context.chars().last() {
            if let Some(first_char) = next_option.chars().next() {
                let compatibility_score = self.get_compatibility_score(last_char, first_char);
                weight *= compatibility_score;
            }
        }

        // Check consecutive vowel/consonant limits
        weight *= self.check_consecutive_limits(context, next_option);

        weight
    }

    /// Get compatibility score between two characters
    fn get_compatibility_score(&self, last_char: char, next_char: char) -> f32 {
        let last_is_vowel = is_vowel(last_char);
        let next_is_vowel = is_vowel(next_char);

        match (last_is_vowel, next_is_vowel) {
            (true, false) => {
                // Vowel followed by consonant
                if let Some(compatible_consonants) =
                    self.vowel_consonant_compatibility.get(&last_char)
                {
                    if compatible_consonants.contains(&next_char) {
                        return self.compatible_weight_multiplier;
                    }
                }
            }
            (false, true) => {
                // Consonant followed by vowel
                if let Some(compatible_vowels) = self.consonant_vowel_compatibility.get(&last_char)
                {
                    if compatible_vowels.contains(&next_char) {
                        return self.compatible_weight_multiplier;
                    }
                }
            }
            _ => {}
        }

        1.0 // Neutral compatibility
    }

    /// Check if the addition would violate consecutive limits
    fn check_consecutive_limits(&self, context: &str, next_option: &str) -> f32 {
        let context_chars: Vec<char> = context.chars().collect();
        let next_chars: Vec<char> = next_option.chars().collect();

        if context_chars.is_empty() || next_chars.is_empty() {
            return 1.0;
        }

        let last_char = context_chars[context_chars.len() - 1];
        let next_char = next_chars[0];

        let last_is_vowel = is_vowel(last_char);
        let next_is_vowel = is_vowel(next_char);

        // Count consecutive vowels/consonants at the end of context
        let mut consecutive_count = 1;
        for &ch in context_chars.iter().rev().skip(1) {
            if is_vowel(ch) == last_is_vowel {
                consecutive_count += 1;
            } else {
                break;
            }
        }

        // Check if adding next_char would exceed limits
        if last_is_vowel == next_is_vowel {
            let max_allowed = if last_is_vowel {
                self.max_consecutive_vowels
            } else {
                self.max_consecutive_consonants
            };

            if consecutive_count >= max_allowed {
                return 0.0; // Would exceed limit
            }
        }

        1.0 // Within limits
    }
}

/// Helper function to determine if a character is a vowel
fn is_vowel(ch: char) -> bool {
    matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}

/// Predefined phonetic rules for different sound profiles
pub mod profiles {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        /// Dark/mysterious sound profile
        /// Emphasizes deep vowels (a, o, u) and hard consonants (k, g, r, th)
        pub static ref DARK_RULES: PhoneticRules = PhoneticRules {
            name: "dark",
            forbidden_sequences: vec!["ee", "ii", "ay", "ey", "ly", "el"], // Bright sounds
            preferred_sequences: vec!["rk", "th", "sh", "ck", "ng", "gh", "or", "ar"], // Dark combinations
            vowel_consonant_compatibility: {
                let mut map = HashMap::new();
                map.insert('a', vec!['r', 'k', 'g']);
                map.insert('o', vec!['r', 'k', 'g']);
                map.insert('u', vec!['r', 'k', 'g', 'n']);
                map
            },
            consonant_vowel_compatibility: {
                let mut map = HashMap::new();
                map.insert('r', vec!['a', 'o', 'u']);
                map.insert('k', vec!['a', 'o']);
                map.insert('g', vec!['a', 'o', 'u']);
                map
            },
            max_consecutive_vowels: 1,
            max_consecutive_consonants: 2,
            preferred_weight_multiplier: 2.0,
            compatible_weight_multiplier: 1.5,
        };

        /// Bright/cheerful sound profile
        /// Emphasizes light vowels (e, i, a) and soft consonants (l, r, n, s)
        pub static ref BRIGHT_RULES: PhoneticRules = PhoneticRules {
            name: "bright",
            forbidden_sequences: vec!["rkh", "ggh", "kk", "gg", "orth", "ark"], // Dark sounds
            preferred_sequences: vec!["ly", "el", "ar", "al", "er", "en", "in", "an"], // Bright combinations
            vowel_consonant_compatibility: {
                let mut map = HashMap::new();
                map.insert('e', vec!['l', 'r', 'n', 's', 't']);
                map.insert('i', vec!['l', 'n', 's', 'r', 't']);
                map.insert('a', vec!['l', 'r', 'n', 's']);
                map.insert('y', vec!['l', 'r', 'n', 's']);
                map
            },
            consonant_vowel_compatibility: {
                let mut map = HashMap::new();
                map.insert('l', vec!['e', 'i', 'a', 'y']);
                map.insert('r', vec!['e', 'i', 'a', 'y']);
                map.insert('n', vec!['e', 'i', 'a', 'y']);
                map.insert('s', vec!['e', 'i', 'a', 'y']);
                map.insert('t', vec!['e', 'i', 'a', 'y']);
                map
            },
            max_consecutive_vowels: 2, // Allow diphthongs
            max_consecutive_consonants: 1,
            preferred_weight_multiplier: 2.0,
            compatible_weight_multiplier: 1.5,
        };

        /// Exotic/alien sound profile
        /// Allows unusual combinations and longer sequences
        pub static ref EXOTIC_RULES: PhoneticRules = PhoneticRules {
            name: "exotic",
            forbidden_sequences: vec!["aaa", "eee", "iii", "ooo", "uuu"], // Only prevent triple vowels
            preferred_sequences: vec!["zz", "xx", "qy", "yq", "zk", "xr"], // Unusual combinations
            vowel_consonant_compatibility: {
                let mut map = HashMap::new();
                map.insert('a', vec!['z', 'x', 'q', 'v', 'w']);
                map.insert('e', vec!['z', 'x', 'q', 'v', 'w']);
                map.insert('i', vec!['z', 'x', 'q', 'v', 'w']);
                map.insert('o', vec!['z', 'x', 'q', 'v', 'w']);
                map.insert('u', vec!['z', 'x', 'q', 'v', 'w']);
                map
            },
            consonant_vowel_compatibility: {
                let mut map = HashMap::new();
                map.insert('z', vec!['a', 'e', 'i', 'o', 'u']);
                map.insert('x', vec!['a', 'e', 'i', 'o', 'u']);
                map.insert('q', vec!['a', 'e', 'i', 'o', 'u']);
                map.insert('v', vec!['a', 'e', 'i', 'o', 'u']);
                map.insert('w', vec!['a', 'e', 'i', 'o', 'u']);
                map
            },
            max_consecutive_vowels: 3,
            max_consecutive_consonants: 3,
            preferred_weight_multiplier: 1.5,
            compatible_weight_multiplier: 1.2,
        };
    }
}
