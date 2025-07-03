//! Name generation categories for celestial objects.
//!
//! This module defines naming categories using macros, following the same pattern
//! as the physics units system. Each category provides specific naming patterns
//! for different types of celestial objects.

use super::NameCategory;

// Define the core celestial object categories using the macro
crate::define_name_categories! {
    categories: {
        Star = "<!s><v><c>",
        RockyBody = "<!s><v><c>",
        GaseousBody = "<!s><v><c>",
        IcyBody = "<!s><v><c>",
        Dragon = "",
    }
}
