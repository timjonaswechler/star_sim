use bevy::{prelude::*, window::Window};

/// Adds a rendered Player Run or, with `automation`, a rendered Controlled Session.
pub fn add_rendered_run_plugins(app: &mut App, window: Window) -> &mut App {
    #[cfg(feature = "automation")]
    {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .build()
                .disable::<bevy::input::InputPlugin>()
                .disable::<bevy::gilrs::GilrsPlugin>(),
        )
        .add_plugins((
            automation_control::AutomationControlPlugin::rendered_stdio(),
            automation_control::screenshot::Plugin::default(),
        ))
    }

    #[cfg(not(feature = "automation"))]
    {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
    }
}
