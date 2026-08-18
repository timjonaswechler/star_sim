pub mod components;

use bevy::prelude::*;
use components::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {}
}
