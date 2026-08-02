use bevy::prelude::*;

use crate::math::probabilities::SeedPlugin;

pub mod probabilities;

pub struct MathPlugin;

impl Plugin for MathPlugin {
    /// Builds the plugin by adding necessary resources, events, and systems to the Bevy `App`.
    fn build(&self, app: &mut App) {
        app.add_plugins(SeedPlugin);
    }
}
