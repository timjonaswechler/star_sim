use bevy::prelude::*;

mod composition;
mod menu;

/// Gibt ein Raum in dem alle Plugins und Systeme oder andere Dinge für die App initialisiert werden können ohne das man doppelten Code schreibt.
fn init(app: &mut App) -> &mut App {
    app.add_plugins(menu::MenuPlugin)
}

fn main() {
    let mut app = App::new();
    #[cfg(feature = "automation-control")]
    composition::controlled(&mut app);
    init(&mut app);
    app.run();
}
