use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "automation-control")]
    if std::env::args().any(|argument| argument == "--automation") {
        app.add_plugins(automation_control::AutomationControlPlugin::default());
    }

    app.run();
}
