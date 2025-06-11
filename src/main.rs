use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
mod lib;
mod stellar_objects;
use stellar_objects::generate_teacup_system;

// Dieser Code würde in einer Bevy-App laufen.
// Der Einfachheit halber hier nur der Aufruf der Setup-Funktion.
fn main() {
    // 1. Generiere die Datenstruktur für das Sternensystem.
    let teacup_system = generate_teacup_system();

    // 2. Konfiguriere das RON-Format für gute Lesbarkeit.
    let pretty_config = ron::ser::PrettyConfig::new()
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    // 3. Serialisiere die Struktur in einen RON-String.
    let ron_string = ron::ser::to_string_pretty(&teacup_system, pretty_config)
        .expect("Fehler bei der Serialisierung zu RON.");

    // 4. Schreibe den RON-String in eine Datei.
    let file_path = "teacup_system.ron";
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
