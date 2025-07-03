//! Macros for generating name systems with minimal boilerplate.
//!
//! This module provides three specialized macros that work together to create
//! a flexible and maintainable name generation system:
//!
//! 1. `define_symbol_profile!` - Creates symbol profiles for different sound sets
//! 2. `define_phonetic_rules!` - Creates phonetic rules for sound compatibility
//! 3. `define_name_category!` - Creates name categories using profiles and rules

// Imports are handled within the macro expansion to avoid unused import warnings

/// Creates a symbol profile with a specific sound set.
///
/// This macro generates a symbol map definition and the corresponding static HashMap
/// that can be used by name categories. It follows the same pattern as the units system
/// for consistency and type safety.
///
/// # Parameters
///
/// - `$name`: The name of the symbol profile (e.g., `Dark`, `Bright`)
/// - `syllables`: Array of syllable strings
/// - `simple_vowels`: Array of single vowel strings
/// - `complex_vowels`: Array of vowel combinations
/// - `simple_consonants`: Array of single consonant strings
/// - `beginning_clusters`: Array of consonant clusters for word beginnings
/// - `ending_clusters`: Array of consonant clusters for word endings
///
/// # Examples
///
/// ```rust
/// use star_sim::define_symbol_profile;
///
/// define_symbol_profile! {
///     Dark {
///         syllables: ["aer", "ash", "drak", "grav", "morg", "nym"],
///         simple_vowels: ["a", "e", "o", "u"],
///         complex_vowels: ["a", "e", "o", "u", "au", "ou"],
///         simple_consonants: ["k", "g", "r", "th", "v", "z"],
///         beginning_clusters: ["dr", "gr", "kr", "th"],
///         ending_clusters: ["rk", "th", "gh", "ng"],
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_symbol_profile {
    (
        $name:ident {
            syllables: [$($syllable:expr),* $(,)?],
            simple_vowels: [$($simple_vowel:expr),* $(,)?],
            complex_vowels: [$($complex_vowel:expr),* $(,)?],
            simple_consonants: [$($simple_consonant:expr),* $(,)?],
            beginning_clusters: [$($beginning_cluster:expr),* $(,)?],
            ending_clusters: [$($ending_cluster:expr),* $(,)?],
        }
    ) => {
        // Define the symbol map definition struct
        pub struct $name;

        impl $crate::utilities::name_generator::symbol_types::SymbolMapDefinition for $name {
            fn syllables() -> Vec<&'static str> {
                vec![$($syllable),*]
            }

            fn simple_vowels() -> Vec<&'static str> {
                vec![$($simple_vowel),*]
            }

            fn complex_vowels() -> Vec<&'static str> {
                vec![$($complex_vowel),*]
            }

            fn simple_consonants() -> Vec<&'static str> {
                vec![$($simple_consonant),*]
            }

            fn beginning_clusters() -> Vec<&'static str> {
                vec![$($beginning_cluster),*]
            }

            fn ending_clusters() -> Vec<&'static str> {
                vec![$($ending_cluster),*]
            }
        }

        // Generate the symbol map using the existing create_symbol_map! macro
        paste::paste! {
            $crate::utilities::name_generator::symbol_types::create_symbol_map!([<$name:upper _SYMBOL_MAP>], $name);
        }
    };
}

/// Creates a phonetic rules set for sound compatibility.
///
/// This macro generates a static PhoneticRules instance that can be used
/// by name categories to filter and weight sound combinations.
///
/// # Parameters
///
/// - `$name`: The name of the phonetic rules set (e.g., `DarkRules`, `BrightRules`)
/// - `forbidden_sequences`: Array of sound sequences that should never appear
/// - `preferred_sequences`: Array of sound sequences that should be favored
/// - `vowel_consonant_compatibility`: Map of vowels to compatible consonants
/// - `consonant_vowel_compatibility`: Map of consonants to compatible vowels
/// - `max_consecutive_vowels`: Maximum consecutive vowels allowed
/// - `max_consecutive_consonants`: Maximum consecutive consonants allowed
/// - `preferred_weight_multiplier`: Weight multiplier for preferred combinations
/// - `compatible_weight_multiplier`: Weight multiplier for compatible combinations
///
/// # Examples
///
/// ```rust
/// use star_sim::define_phonetic_rules;
///
/// define_phonetic_rules! {
///     DarkRules {
///         forbidden_sequences: ["ii", "ee", "ll"],
///         preferred_sequences: ["th", "kr", "gh"],
///         vowel_consonant_compatibility: {
///             'a' => ['r', 'k', 'g'],
///             'o' => ['r', 'k', 'th'],
///         },
///         consonant_vowel_compatibility: {
///             'r' => ['a', 'o', 'u'],
///             'k' => ['a', 'o'],
///         },
///         max_consecutive_vowels: 2,
///         max_consecutive_consonants: 2,
///         preferred_weight_multiplier: 2.0,
///         compatible_weight_multiplier: 1.5,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_phonetic_rules {
    (
        $name:ident {
            forbidden_sequences: [$($forbidden:expr),* $(,)?],
            preferred_sequences: [$($preferred:expr),* $(,)?],
            vowel_consonant_compatibility: {
                $($vowel:expr => [$($v_consonant:expr),* $(,)?]),* $(,)?
            },
            consonant_vowel_compatibility: {
                $($consonant:expr => [$($c_vowel:expr),* $(,)?]),* $(,)?
            },
            max_consecutive_vowels: $max_vowels:expr,
            max_consecutive_consonants: $max_consonants:expr,
            preferred_weight_multiplier: $preferred_weight:expr,
            compatible_weight_multiplier: $compatible_weight:expr,
        }
    ) => {
        paste::paste! {
            lazy_static::lazy_static! {
                pub static ref [<$name:upper>]: $crate::utilities::name_generator::phonetic_rules::PhoneticRules = $crate::utilities::name_generator::phonetic_rules::PhoneticRules {
                    name: stringify!($name),
                    forbidden_sequences: vec![$($forbidden),*],
                    preferred_sequences: vec![$($preferred),*],
                    vowel_consonant_compatibility: {
                        let mut map = std::collections::HashMap::new();
                        $(
                            map.insert($vowel, vec![$($v_consonant),*]);
                        )*
                        map
                    },
                    consonant_vowel_compatibility: {
                        let mut map = std::collections::HashMap::new();
                        $(
                            map.insert($consonant, vec![$($c_vowel),*]);
                        )*
                        map
                    },
                    max_consecutive_vowels: $max_vowels,
                    max_consecutive_consonants: $max_consonants,
                    preferred_weight_multiplier: $preferred_weight,
                    compatible_weight_multiplier: $compatible_weight,
                };
            }
        }
    };
}

/// Creates a name category with pattern, symbol profile, and phonetic rules.
///
/// This macro creates a category marker type and implements the NameCategory trait
/// with the specified configuration. It's designed to work with symbol profiles
/// and phonetic rules created by the other macros.
///
/// # Parameters
///
/// - `$name`: The name of the category (e.g., `DarkStar`, `BrightPlanet`)
/// - `pattern`: The pattern string for name generation (required)
/// - `symbol_profile`: The symbol profile to use (optional, defaults to standard)
/// - `phonetic_rules`: The phonetic rules to apply (optional, defaults to none)
///
/// # Examples
///
/// ```rust
/// use star_sim::define_name_category;
///
/// // Category with all options
/// define_name_category! {
///     DarkStar {
///         pattern: "<!s><v><c>",
///         symbol_profile: Dark,
///         phonetic_rules: DarkRules,
///     }
/// }
///
/// // Category with just pattern (uses defaults)
/// define_name_category! {
///     StandardStar {
///         pattern: "<!s><v><c>",
///     }
/// }
///
/// // Category with pattern and symbol profile
/// define_name_category! {
///     BrightStar {
///         pattern: "<!s><v><c><v><s>",
///         symbol_profile: Bright,
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_name_category {
    // Full specification: pattern + symbol_profile + phonetic_rules
    (
        $name:ident {
            pattern: $pattern:expr,
            symbol_profile: $symbol_profile:ident,
            phonetic_rules: $phonetic_rules:ident,
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name;

        impl $crate::utilities::name_generator::NameCategory for $name {
            type Variant = Self;

            fn pattern(&self) -> &'static str {
                $pattern
            }

            fn symbol_map(&self) -> &std::collections::HashMap<&'static str, Vec<&'static str>> {
                paste::paste! {
                    &[<$symbol_profile:upper _SYMBOL_MAP>]
                }
            }

            fn phonetic_rules(&self) -> Option<&$crate::utilities::name_generator::phonetic_rules::PhoneticRules> {
                paste::paste! {
                    Some(&*[<$phonetic_rules:upper>])
                }
            }
        }
    };

    // Pattern + symbol_profile (no phonetic rules)
    (
        $name:ident {
            pattern: $pattern:expr,
            symbol_profile: $symbol_profile:ident,
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name;

        impl $crate::utilities::name_generator::NameCategory for $name {
            type Variant = Self;

            fn pattern(&self) -> &'static str {
                $pattern
            }

            fn symbol_map(&self) -> &std::collections::HashMap<&'static str, Vec<&'static str>> {
                paste::paste! {
                    &[<$symbol_profile:upper _SYMBOL_MAP>]
                }
            }
        }
    };

    // Pattern + phonetic_rules (no symbol profile)
    (
        $name:ident {
            pattern: $pattern:expr,
            phonetic_rules: $phonetic_rules:ident,
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name;

        impl $crate::utilities::name_generator::NameCategory for $name {
            type Variant = Self;

            fn pattern(&self) -> &'static str {
                $pattern
            }

            fn phonetic_rules(&self) -> Option<&$crate::utilities::name_generator::phonetic_rules::PhoneticRules> {
                paste::paste! {
                    Some(&*[<$phonetic_rules:upper>])
                }
            }
        }
    };

    // Pattern only (uses defaults)
    (
        $name:ident {
            pattern: $pattern:expr,
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub struct $name;

        impl $crate::utilities::name_generator::NameCategory for $name {
            type Variant = Self;

            fn pattern(&self) -> &'static str {
                $pattern
            }
        }
    };
}