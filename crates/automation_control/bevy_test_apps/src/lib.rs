use bevy::{prelude::*, window::Window};

/// Adds the Player Run plugins or, with `automation`, the Controlled Session plugins.
pub fn add_run_plugins(app: &mut App, controlled_window: Window) -> &mut App {
    #[cfg(feature = "automation")]
    {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(controlled_window),
                    ..default()
                })
                .build()
                .disable::<bevy::input::InputPlugin>()
                .disable::<bevy::gilrs::GilrsPlugin>(),
        )
        .add_plugins((
            automation_control::AutomationControlPlugin::stdio(),
            automation_control::screenshot::Plugin::default(),
        ))
    }

    #[cfg(not(feature = "automation"))]
    {
        let _ = controlled_window;
        app.add_plugins(DefaultPlugins)
    }
}
