//! Core traits and types for the name generator system.
//!
//! This module provides the fundamental building blocks for the macro-based name generation system,
//! following the same pattern as the physics units system.

use super::{Pattern, phonetic_rules::PhoneticRules};
use super::symbols::SYMBOL_MAP;
use std::collections::HashMap;
use rand::Rng;

/// Base trait for name pattern categories
/// 
/// All name generator categories must implement this trait to provide
/// their specific naming patterns and generation logic. Categories can customize
/// their sound profile through custom symbol maps and phonetic rules.
pub trait NameCategory: Default {
    type Variant;

    /// Get the pattern string for this category variant
    fn pattern(&self) -> &'static str;

    /// Get the symbol map for this category
    /// 
    /// Returns a reference to the symbol map that defines available sounds.
    /// Default implementation uses the standard symbol map, but categories
    /// can override this to provide specialized sound sets.
    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &SYMBOL_MAP
    }

    /// Get phonetic rules for this category
    /// 
    /// Returns phonetic rules that will be applied during name generation
    /// to adjust probabilities based on context and sound compatibility.
    /// Default implementation returns None (no special rules).
    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        None
    }

    /// Get a parsed Pattern for this category variant
    fn get_pattern(&self) -> Pattern {
        Pattern::parse(self.pattern(), self.symbol_map(), false).expect("Invalid pattern in category")
    }

    /// Generate a name using this category's pattern and rules
    fn generate(&self, rng: &mut impl Rng) -> String {
        let mut context = String::new();
        self.get_pattern().generate_with_context(rng, &mut context, self.phonetic_rules())
    }
}

/// Main name type that works with any category, similar to how Quantity<Unit> works
///
/// This is the core type that provides a consistent API for name generation
/// across all categories, following the same design pattern as the units system.
///
/// # Examples
///
/// ```rust
/// use star_sim::utilities::name_generator::*;
/// use rand::thread_rng;
///
/// let mut rng = thread_rng();
///
/// // Basic usage
/// let star_name = Name::<Star>::new().generate(&mut rng);
/// let planet_name = Name::<RockyBody>::new().generate(&mut rng);
/// ```
#[derive(Debug, Clone)]
pub struct Name<T: NameCategory> {
    category: T,
}

impl<T: NameCategory> Name<T> {
    /// Create a new name generator for a specific category
    pub fn new() -> Self {
        Self { 
            category: T::default()
        }
    }
    
    /// Create a new name generator with a specific category instance
    pub fn with_category(category: T) -> Self {
        Self { category }
    }

    /// Generate a name using this category's pattern
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        self.category.generate(rng)
    }

    /// Get the category this name generator uses
    pub fn category(&self) -> &T {
        &self.category
    }
}

impl<T: NameCategory> Default for Name<T> {
    fn default() -> Self {
        Self::new()
    }
}

// For backward compatibility, keep the old NameGenerator type as an alias
/// Legacy name generator type - use `Name<T>` instead
#[deprecated(note = "Use Name<T> instead for consistency with units system")]
pub type NameGenerator<T> = Name<T>;