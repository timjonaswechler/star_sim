use bevy::prelude::*;

mod menu;

#[cfg(feature = "automation-control")]
mod automation;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(menu::MenuPlugin);

    #[cfg(feature = "automation-control")]
    app.add_plugins(automation::AutomationPlugin);

    app.run();
}
