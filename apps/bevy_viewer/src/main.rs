use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "agent-control")]
    if std::env::args().any(|argument| argument == "--agent") {
        app.add_plugins(agent_control::AgentControlPlugin::default());
    }

    app.run();
}
