use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use star_sim::stellar_objects::generate_teacup_system;

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
    match to_roman(8) {
        Ok(roman) => println!("Römische Zahl: {}", roman),
        Err(e) => eprintln!("Fehler bei der Umwandlung in römische Zahlen: {}", e),
    }
    println!("--- Anwendungsfall 1: Nach und nach die Symbole von 1 bis 26 ausgeben ---");
    // Wir zählen bis 26, um auch den Fehlerfall zu zeigen.
    for i in 1..=26 {
        match to_greek_symbol(i) {
            Ok(symbol) => println!("Index {}: {}", i, symbol),
            Err(e) => eprintln!("Fehler bei der Umwandlung in griechische Symbole: {}", e),
        }
    }
}
fn to_roman(mut num: u32) -> Result<String, &'static str> {
    // Römische Zahlen haben keine 0 und dieses Schema funktioniert üblicherweise nur bis 3999.
    if num == 0 {
        return Err("Römische Zahlen kennen keine Null.");
    }
    if num >= 4000 {
        return Err("Diese Funktion unterstützt nur Zahlen kleiner als 4000.");
    }

    // Eine Zuordnung von Werten zu ihren römischen Symbolen.
    // Wichtig: Die Liste muss absteigend sortiert sein, damit der Algorithmus funktioniert.
    // Sie enthält auch die subtraktiven Fälle (z.B. 900 für "CM", 4 für "IV").
    let mapping = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();

    // Wir gehen die Zuordnungen von der größten zur kleinsten durch.
    for &(value, symbol) in &mapping {
        // Solange die Zahl größer oder gleich dem aktuellen Wert ist...
        while num >= value {
            // ...fügen wir das entsprechende Symbol zum Ergebnis hinzu...
            result.push_str(symbol);
            // ...und ziehen den Wert von unserer Zahl ab.
            num -= value;
        }
    }

    Ok(result)
}

fn to_greek_symbol(index: usize) -> Result<String, &'static str> {
    // Statische Liste der Symbole.
    const GREEK_ALPHABET_SYMBOLS: [&'static str; 24] = [
        "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ", "ν", "ξ", "ο", "π", "ρ", "σ",
        "τ", "υ", "φ", "χ", "ψ", "ω",
    ];

    // 1. Gültigkeitsprüfung
    // `GREEK_ALPHABET_SYMBOLS.len()` holt die Größe des Arrays (24) dynamisch.
    if index > 0 && index <= GREEK_ALPHABET_SYMBOLS.len() {
        Ok(GREEK_ALPHABET_SYMBOLS[index - 1].to_string())
    } else {
        // 3. Fehlerfall: Der Index ist ungültig.
        Err("Ungültiger Index. Der Index muss zwischen 1 und 24 liegen.")
    }
}
