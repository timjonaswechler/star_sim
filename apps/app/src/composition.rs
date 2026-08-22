#![cfg(feature = "automation-control")]
use bevy::prelude::*;

pub(crate) fn controlled(app: &mut App) -> &mut App {
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Star Sim Controlled Session".into(),
                    ..default()
                }),
                ..default()
            })
            .build()
            .disable::<bevy::input::InputPlugin>()
            .disable::<bevy::gilrs::GilrsPlugin>(),
        bug_hunter::AutomationControlPlugin::rendered_stdio(),
        bug_hunter::screenshot::Plugin::default(),
    ))
}
