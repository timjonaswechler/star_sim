use bevy::prelude::*;

mod composition;
mod menu;

#[cfg(not(feature = "automation-control"))]
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(menu::MenuPlugin);
    app.run();
}

#[cfg(feature = "automation-control")]
fn main() {
    let mode =
        composition::ControlledMode::parse(std::env::args_os().skip(1)).unwrap_or_else(|error| {
            eprintln!("app: {error}");
            std::process::exit(2);
        });
    let mut app = App::new();
    composition::controlled(&mut app, mode).add_plugins(menu::MenuPlugin);
    app.run();
}
