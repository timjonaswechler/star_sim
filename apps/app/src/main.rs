use bevy::prelude::*;

mod menu;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(menu::MenuPlugin);

    #[cfg(feature = "automation-control")]
    if std::env::args().any(|argument| argument == "--automation") {
        app.add_plugins(automation_control::AutomationControlPlugin::default());
    }

    app.run();
}
