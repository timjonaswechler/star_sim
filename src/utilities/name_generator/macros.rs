//! Macros for generating name systems with minimal boilerplate.
//!
//! This module provides macros that automatically generate naming categories and patterns,
//! following the same design pattern as the physics units system.

/// Generates a complete naming system for a specific category.
///
/// This macro creates category marker types and implements naming traits with minimal boilerplate.
/// It follows the same pattern as the units system for consistency.
///
/// # Parameters
///
/// - `categories`: A list of category types and their naming patterns
///
/// # Generated Code
///
/// For each category specified, this macro generates:
/// - A category marker struct (e.g., `struct Star;`)
/// - `NameCategory` implementation with the provided pattern
/// - Integration with the `Name<T>` system
///
/// # Examples
///
/// ```rust
/// use star_sim::utilities::name_generator::*;
/// use star_sim::define_name_categories;
///
/// define_name_categories! {
///     categories: {
///         Star = "<!s><v><c>",
///         RockyBody = "<!s><v><c>",
///         GaseousBody = "<!s><v><c>",
///         IcyBody = "<!s><v><c>",
///     }
/// }
/// ```
///
/// # Usage
///
/// Once defined, you can use these categories with the `Name<T>` API:
///
/// ```rust
/// let star_name = Name::<Star>::new().generate(&mut rng);
/// let planet_name = Name::<RockyBody>::new().generate(&mut rng);
/// ```
#[macro_export]
macro_rules! define_name_categories {
    (
        categories: {
            $($category:ident = $pattern:expr),+ $(,)*
        }
    ) => {
        // Define category marker structs
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
            pub struct $category;
        )+

        // Implement NameCategory trait for each category
        $(
            impl NameCategory for $category {
                type Variant = Self;

                fn pattern(&self) -> &'static str {
                    $pattern
                }
            }
        )+
    };
}

/// Creates a name type for a specific category.
///
/// This macro provides a simple way to define name types for different categories,
/// similar to how `define_quantity!` works for units.
///
/// # Parameters
///
/// - `$name`: The name of the new name type
/// - `$category`: The category this name type represents
///
/// # Examples
///
/// ```rust
/// use star_sim::utilities::name_generator::*;
/// use star_sim::define_name_type;
///
/// // Define name types for different categories
/// define_name_type!(StellarName, Star);
/// define_name_type!(PlanetName, RockyBody);
/// ```
#[macro_export]
macro_rules! define_name_type {
    ($name:ident, $category:ident) => {
        pub type $name = Name<$category>;
    };
}