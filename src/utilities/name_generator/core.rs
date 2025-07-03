//! Core traits and types for the name generator system.
//!
//! This module provides the fundamental building blocks for the macro-based name generation system,
//! following the same pattern as the physics units system.

use super::{Pattern, symbols::SYMBOL_MAP};
use rand::Rng;

/// Base trait for name pattern categories
/// 
/// All name generator categories must implement this trait to provide
/// their specific naming patterns and generation logic.
pub trait NameCategory: Default {
    type Variant;

    /// Get the pattern string for this category variant
    fn pattern(&self) -> &'static str;

    /// Get a parsed Pattern for this category variant
    fn get_pattern(&self) -> Pattern {
        Pattern::parse(self.pattern(), &SYMBOL_MAP, false).expect("Invalid pattern in category")
    }

    /// Generate a name using this category's pattern
    fn generate(&self, rng: &mut impl Rng) -> String {
        self.get_pattern().generate(rng)
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