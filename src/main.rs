// main.rs - Vereinfachter Sternsystem Generator

// use rand::SeedableRng; // Nicht direkt genutzt in dieser vereinfachten main, aber oft nützlich
// use rand_chacha::ChaCha8Rng; // Wie oben
// use std::collections::HashMap; // Wie oben

// Module imports
mod constants;
mod cosmic_environment;
mod habitability;
mod lagrange_points;
mod orbital_mechanics;
mod stellar_properties;
mod system_hierarchy;
mod units;

// Spezifische imports für die vereinfachte main
use stellar_properties::StellarProperties;
use system_hierarchy::{StarSystem, SystemType};
use units::*; // UnitSystem hier importieren

fn main() {
    println!(
        "🌌 Minimaler Seed-basierter Sternsystem Generator v{}",
        constants::VERSION
    );

    println!("{}", "=".repeat(50));

    println!("\nEinige fundamentale Konstanten:");
    println!(
        "  Lichtgeschwindigkeit (c): {:.4e} m/s",
        constants::SPEED_OF_LIGHT
    );
    println!(
        "  Planck-Konstante (h): {:.4e} J·s",
        constants::PLANCK_CONSTANT
    );
    println!(
        "  Boltzmann-Konstante (k): {:.4e} J/K",
        constants::BOLTZMANN_CONSTANT
    );
    println!("  Tage pro Julianischem Jahr: {}", constants::DAYS_PER_YEAR);
    println!("\nAstronomische Standardwerte für die Sonne:");
    println!(
        "  Gravitationsparameter (GM☉): {:.4e} m³/s²",
        constants::standards::SOLAR_MU
    );
    println!(
        "  Orbitale Geschwindigkeit bei 1 AU: {:.0} m/s",
        constants::standards::SOLAR_ORBITAL_VELOCITY_1AU
    );
    println!(
        "  Fluchtgeschwindigkeit von der Sonnenoberfläche: {:.0} m/s",
        constants::standards::SOLAR_ESCAPE_VELOCITY
    );
    println!("{}", "=".repeat(50));
    println!("{}", "=".repeat(50));
    println!("");

    // Wähle einen Seed für die Generierung
    let seed = 2024_u64; // u64, da generate_from_seed es erwartet

    // Generiere das Sternsystem.
    // StarSystem::generate_from_seed verwendet standardmäßig UnitSystem::Astronomical
    // Wenn du SI-Einheiten willst: StarSystem::generate_from_seed_with_units(seed, UnitSystem::SI)
    let system = StarSystem::generate_from_seed(seed);

    println!("🌱 Generiertes System mit Seed: {}", system.seed);
    println!("   Verwendetes Einheitensystem: {:?}", system.unit_system);
    println!(
        "   Alter des Universums der Simulation: {:.2} Gyr",
        system.cosmic_epoch.age_universe
    );
    println!("     - Ära: {}", system.cosmic_epoch.era); // Auch nützlich
    println!(
        "     - Erlaubt komplexe Chemie: {}",
        system.cosmic_epoch.allows_complex_chemistry()
    );
    println!(
        "     - Erlaubt langlebige Sterne: {}",
        system.cosmic_epoch.allows_long_lived_stars()
    );

    let dist_val = system.galactic_region.distance_from_center().value;
    let dist_unit = get_distance_unit_str(system.galactic_region.distance_from_center().system);
    println!(
        "\n   Galaktische Region Entfernung: {:.2} {}",
        dist_val, dist_unit
    );
    println!(
        "     - Entfernung vom Zentrum: {:?}",
        system.galactic_distance
    );
    println!(
        "     - Habitabilitätsfaktor der Region: {:.2}",
        system.galactic_region.habitability_factor()
    );

    println!("\n   Galaktische Dynamik:");
    println!(
        "     - Rotationsgeschwindigkeit: {:.1} km/s",
        system.galactic_dynamics.rotation_velocity
    );
    println!(
        "     - Umlaufperiode (galaktisch): {:.1} Myr",
        system.galactic_dynamics.orbital_period
    );
    match &system.galactic_dynamics.spiral_arm_context {
        cosmic_environment::SpiralArmContext::InArm {
            arm_name,
            position_in_arm,
        } => {
            println!(
                "     - Spiralarm: Im {} (Position: {:.2})",
                arm_name, position_in_arm
            );
        }
        cosmic_environment::SpiralArmContext::InterArm {
            distance_to_nearest_arm,
            nearest_arm_name,
        } => {
            let dist_val = distance_to_nearest_arm.value; // Annahme: value ist in kpc, wenn Astronomical
            let unit_str = match distance_to_nearest_arm.system {
                UnitSystem::Astronomical => "kpc", // Oder was auch immer die Einheit in generate_random ist
                UnitSystem::SI => "m",
            };
            println!(
                "     - Spiralarm: Zwischen den Armen, {:.2} {} zu {}",
                dist_val, unit_str, nearest_arm_name
            );
        }
        cosmic_environment::SpiralArmContext::CorotationResonance => {
            println!("     - Spiralarm: Korotationsresonanz");
        }
        cosmic_environment::SpiralArmContext::LindBladResonance => {
            println!("     - Spiralarm: Lindblad-Resonanz");
        }
    }
    println!(
        "     - Umgebungsstabilität (Dynamik): {:.2}",
        system.galactic_dynamics.environmental_stability()
    );
    println!("\n   Kosmische Strahlungsumgebung:");
    println!(
        "     - AGN Risiko: {:.2}",
        system.radiation_environment.agn_risk
    );
    println!(
        "     - Supernova Frequenz (relativ): {:.2}",
        system.radiation_environment.supernova_frequency
    );
    println!(
        "     - GRB Risiko: {:.2}",
        system.radiation_environment.grb_risk
    );
    println!(
        "     - Kosmischer Strahlungsfluss: {:.2e} GeV/cm²/s",
        system.radiation_environment.cosmic_ray_flux
    );
    println!(
        "     - UV Hintergrund: {:.2}",
        system.radiation_environment.uv_background
    );
    println!(
        "     - Gesamtstrahlungsrisiko: {:.3}",
        system.radiation_environment.total_radiation_risk()
    );
    println!(
        "     - Ist Strahlungsumgebung lebensfreundlich: {}",
        system.radiation_environment.is_life_friendly()
    );

    println!("\n🔭 System Details:");
    match &system.system_type {
        SystemType::Single(star) => {
            println!("  Typ: Einzelsternsystem");
            print_star_details("  Stern", star);
            let hz = star.calculate_habitable_zone();
            println!(
                "    Habitable Zone (konservativ): {:.2} - {:.2} {} (optimistisch: {:.2} - {:.2} {})",
                hz.inner_edge.value,
                hz.outer_edge.value,
                get_distance_unit_str(star.unit_system),
                hz.optimistic_inner.value,
                hz.optimistic_outer.value,
                get_distance_unit_str(star.unit_system)
            );
        }
        SystemType::Binary {
            primary,
            secondary,
            orbital_properties,
        } => {
            println!("  Typ: Binärsternsystem");
            print_star_details("  Primärstern", primary);
            print_star_details("  Sekundärstern", secondary);
            println!("  Orbitale Eigenschaften:");

            let periapsis_dist = orbital_properties.orbital_elements.periapsis();
            let apoapsis_dist = orbital_properties.orbital_elements.apoapsis();
            let unit_str_orbit = get_distance_unit_str(periapsis_dist.system);
            println!(
                "    Periapsis: {:.2} {}, Apoapsis: {:.2} {}",
                periapsis_dist.value, unit_str_orbit, apoapsis_dist.value, unit_str_orbit
            );
            println!(
                "    Gegenseitige Hill-Sphäre: {:.3} {}",
                orbital_properties.mutual_hill_sphere.value,
                get_distance_unit_str(orbital_properties.mutual_hill_sphere.system)
            );
            println!(
                "    Semi-major Axis: {:.2} {}",
                orbital_properties.orbital_elements.semimajor_axis.value,
                get_distance_unit_str(primary.unit_system)
            );
            println!(
                "    Exzentrizität: {:.3}",
                orbital_properties.orbital_elements.eccentricity
            );
            let total_mass_for_period =
                Mass::kilograms(primary.mass.in_kg() + secondary.mass.in_kg());
            let period = orbital_properties
                .orbital_elements
                .orbital_period(&total_mass_for_period);
            println!(
                "    Periode: {:.2} {}",
                period.value,
                get_time_unit_str(primary.unit_system)
            );
            if let Some(ref lagrange_sys) = orbital_properties.lagrange_system {
                println!("  Lagrange-System:");
                println!("    L4/L5 Stabil: {}", lagrange_sys.l4_l5_stable);
                if lagrange_sys.l4_l5_stable {
                    let (l4_x, l4_y) = lagrange_sys.l4_position();
                    let (l5_x, l5_y) = lagrange_sys.l5_position(); // l5_position verwenden
                    let unit_str = get_distance_unit_str(lagrange_sys.unit_system);
                    println!(
                        "      L4 Position (relativ): ({:.2} {}, {:.2} {})",
                        l4_x.value, unit_str, l4_y.value, unit_str
                    );
                    println!(
                        "      L5 Position (relativ): ({:.2} {}, {:.2} {})",
                        l5_x.value, unit_str, l5_y.value, unit_str
                    );

                    // Beispielhafte Nutzung von can_capture_at_lagrange_point und hill_sphere
                    // Für einen hypothetischen Testkörper
                    let test_mass = Mass::kilograms(1e15); // Z.B. ein Asteroid
                    if lagrange_sys.can_capture_at_lagrange_point(
                        4,
                        &test_mass.to_system(lagrange_sys.unit_system),
                    ) {
                        println!("      L4 kann Testkörper potenziell einfangen.");
                        if let Some(hill_sphere_l4) = lagrange_sys.hill_sphere_at_lagrange_point(
                            4,
                            &test_mass.to_system(lagrange_sys.unit_system),
                        ) {
                            println!(
                                "        Hill-Sphäre an L4 für Testkörper: {:.3e} {}",
                                hill_sphere_l4.value,
                                get_distance_unit_str(hill_sphere_l4.system)
                            );
                        }
                    }
                }
                if !lagrange_sys.trojans.is_empty() {
                    println!("    Anzahl Trojaner: {}", lagrange_sys.trojans.len());
                    for (i, trojan) in lagrange_sys.trojans.iter().enumerate() {
                        println!(
                            "      Trojaner {}: L{}, Masse: {:.2e} {}",
                            i + 1,
                            trojan.lagrange_point,
                            trojan.mass.value,
                            get_mass_unit_str(trojan.mass.system)
                        );
                        println!("        Langzeitstabil: {}", trojan.is_long_term_stable()); // is_long_term_stable verwenden
                        let max_dist = trojan.maximum_distance_from_lagrange_point(); // verwenden
                        println!(
                            "        Max. Oszillationsentfernung: {:.3} {}",
                            max_dist.value,
                            get_distance_unit_str(max_dist.system)
                        );
                    }
                }
            }
        }
        SystemType::Multiple {
            components,
            hierarchy,
        } => {
            println!("  Typ: Mehrfachsternsystem ({} Sterne)", components.len());
            for (i, star) in components.iter().enumerate() {
                print_star_details(&format!("  Stern {}", i + 1), star);
            }
            println!(
                "  Hierarchie-Stabilität: {:.1}%",
                hierarchy.stability_factor * 100.0
            );
            println!(
                "    Langzeitstabil (simuliert): {}",
                hierarchy.is_long_term_stable()
            );
            let dyn_timescale = hierarchy.dynamical_timescale();
            println!(
                "    Dynamische Zeitskala (innerster Orbit): {:.2} {}",
                dyn_timescale.value,
                get_time_unit_str(dyn_timescale.system)
            );
        }
    }

    // Wenn du die "Extras" trotzdem sehen willst, kannst du sie hier ausgeben:
    println!("\n🧪 Elementhäufigkeiten-Analyse:");
    println!(
        "    Wasserstoff: {:.2}%",
        system.elemental_abundance.hydrogen * 100.0
    );
    println!(
        "    Helium: {:.2}%",
        system.elemental_abundance.helium * 100.0
    );
    println!(
        "    Sauerstoff: {:.4}%",
        system.elemental_abundance.oxygen * 100.0
    );
    println!(
        "    Kohlenstoff: {:.4}%",
        system.elemental_abundance.carbon * 100.0
    );
    println!(
        "    C/O Verhältnis: {:.3}",
        system.elemental_abundance.carbon_to_oxygen_ratio()
    );
    println!(
        "    Unterstützt terrestrische Planeten: {}",
        system.elemental_abundance.supports_terrestrial_planets()
    );
    println!(
        "    Astrobiologisches Potenzial: {:.2}",
        system.elemental_abundance.astrobiological_potential()
    );

    println!("\n🌍 Bewohnbarkeits-Assessment (Gesamt):");
    println!(
        "    Gesamtbewohnbarkeit: {:.1}%",
        system.habitability_assessment.overall_habitability * 100.0
    );

    println!("\n{}", "=".repeat(50));
    println!("ℹ️  Das System generiert intern immer noch alle kosmischen und Habitabilitätsdaten.");
    println!("   Diese vereinfachte main zeigt nur einen Ausschnitt davon an.");
    println!("\n💾 Test Serialisierung:");
    match system.to_ron_string() {
        Ok(ron_data) => {
            println!(
                "  System erfolgreich nach RON serialisiert ({} Bytes).",
                ron_data.len()
            );
            match StarSystem::from_ron_string(&ron_data) {
                Ok(reimported_system) => {
                    if reimported_system.seed == system.seed {
                        println!(
                            "  System erfolgreich aus RON deserialisiert und Seed stimmt überein."
                        );
                    } else {
                        println!("  System deserialisiert, aber Seed stimmt nicht überein!");
                    }
                }
                Err(e) => println!("  Fehler beim Deserialisieren: {}", e),
            }
        }
        Err(e) => println!("  Fehler beim Serialisieren nach RON: {}", e),
    }
}

fn print_star_details(prefix: &str, star: &StellarProperties) {
    println!("{}:", prefix);
    println!(
        "    Masse: {:.2} {}",
        star.mass.value,
        get_mass_unit_str(star.unit_system)
    );
    println!(
        "    Radius: {:.2} {}",
        star.radius.value,
        get_radius_unit_str(star.unit_system) // Radius wird speziell behandelt, da es R☉ oder m sein kann
    );
    println!(
        "    Spektraltyp: {:?} {:?}",
        star.spectral_type,
        star.luminosity_class // Verwende die Methode, um den Luminositätsklasse-String zu bekommen
    );
    println!("    Temperatur: {:.0} K", star.effective_temperature);
    println!("    Leuchtkraft: {:.3} L☉", star.luminosity); // Leuchtkraft ist immer relativ zu L☉
    println!(
        "    Alter: {:.2} {}",
        star.age.value,
        get_time_unit_str(star.unit_system)
    );
    println!("    Evolutionsstadium: {:?}", star.evolutionary_stage);
}

// Hilfsfunktionen, um die Einheiten im Print schön darzustellen
fn get_mass_unit_str(system: UnitSystem) -> &'static str {
    match system {
        UnitSystem::Astronomical => "M☉",
        UnitSystem::SI => "kg",
    }
}

// Radius ist ein Spezialfall, da stellar_properties.radius.value in R☉ (AU System) oder m (SI System) ist
fn get_radius_unit_str(system: UnitSystem) -> &'static str {
    match system {
        UnitSystem::Astronomical => "R☉", // Der Wert in star.radius ist bereits in Sonnenradien
        UnitSystem::SI => "m",            // Der Wert in star.radius ist bereits in Metern
    }
}

fn get_distance_unit_str(system: UnitSystem) -> &'static str {
    match system {
        UnitSystem::Astronomical => "AU",
        UnitSystem::SI => "m",
    }
}

fn get_time_unit_str(system: UnitSystem) -> &'static str {
    match system {
        UnitSystem::Astronomical => "Jahre",
        UnitSystem::SI => "s",
    }
}
