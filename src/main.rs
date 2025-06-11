use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
mod lib;
mod stellar_objects;
use stellar_objects::generate_teacup_system;
mod physics;

// Dieser Code würde in einer Bevy-App laufen.
// Der Einfachheit halber hier nur der Aufruf der Setup-Funktion.
fn main() {
    let teacup_system = generate_teacup_system();

    let pretty_config = ron::ser::PrettyConfig::new()
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    let ron_string = ron::ser::to_string_pretty(&teacup_system, pretty_config)
        .expect("Fehler bei der Serialisierung zu RON.");

    let file_path = "teacup_system_typed.ron";
    match File::create(file_path) {
        Ok(mut file) => {
            file.write_all(ron_string.as_bytes())
                .expect("Fehler beim Schreiben in die Datei.");
            println!("Sternensystem erfolgreich in '{}' gespeichert.", file_path);
            println!("\n--- RON-Vorschau ---");
            println!("{}", ron_string);
        }
        Err(e) => {
            eprintln!("Konnte Datei '{}' nicht erstellen: {}", file_path, e);
        }
    }
}
