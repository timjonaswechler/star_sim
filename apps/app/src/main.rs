use bevy::prelude::*;

mod menu;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(menu::MenuPlugin);

    app.run();
}
