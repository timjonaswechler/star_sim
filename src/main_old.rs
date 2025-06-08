use lib::*;
use physics::*;

fn main() {
    println!(
        "🌌 Erweiterte Sternsystem Generator v{} mit Trojaner-Unterstützung",
        constants::VERSION
    );

    println!("{}", "=".repeat(50));

    // Konstanten-Ausgabe (wie vorher)...

    let seed = 2024_u64;
    let system = StarSystem::generate_from_seed(seed);

    println!("🌱 Generiertes System mit Seed: {}", system.seed);
    // Basis-Ausgaben (wie vorher)...

    println!("\n🔭 System Details:");
    match &system.system_type {
        SystemType::Single(star) => {
            println!("  Typ: Einzelsternsystem");
            print_star_details("  Stern", star);
            let hz = star.calculate_habitable_zone();
            // Habitable Zone Ausgabe...
        }
        SystemType::Binary {
            primary,
            secondary,
            orbital_properties,
        } => {
            println!("  Typ: Binärsternsystem");
            print_star_details("  Primärstern", primary);
            print_star_details("  Sekundärstern", secondary);

            // Orbitale Eigenschaften...

            if let Some(ref lagrange_sys) = orbital_properties.lagrange_system {
                println!("  Lagrange-System:");
                println!("    L4/L5 Stabil: {}", lagrange_sys.l4_l5_stable);
                println!("    Massenverhältnis: {:.1}:1", lagrange_sys.mass_ratio);

                if lagrange_sys.l4_l5_stable {
                    let (l4_x, l4_y) = lagrange_sys.l4_position();
                    let (l5_x, l5_y) = lagrange_sys.l5_position();
                    let unit_str = get_distance_unit_str(lagrange_sys.unit_system);
                    println!(
                        "      L4 Position: ({:.2} {}, {:.2} {})",
                        l4_x.value, unit_str, l4_y.value, unit_str
                    );
                    println!(
                        "      L5 Position: ({:.2} {}, {:.2} {})",
                        l5_x.value, unit_str, l5_y.value, unit_str
                    );
                }

                // NEU: Detaillierte Trojaner-Ausgabe
                print_trojan_details(lagrange_sys, primary, secondary);
            }
        }
        SystemType::Multiple {
            components,
            hierarchy,
        } => {
            // Multiple System Ausgabe...
        }
    }

    // NEU: Erweiterte Stabilität mit Trojaner-Analyse
    print_system_stability_enhanced(&system);

    // NEU: Trojaner-Bewohnbarkeits-Analyse
    print_trojan_habitability(&system);

    println!("\n🌍 Bewohnbarkeits-Assessment (Erweitert):");
    let enhanced_habitability = HabitabilityAssessment::comprehensive_analysis_with_trojans(
        &system.system_type,
        &system.radiation_environment,
        &vec![Distance::au(0.5), Distance::au(1.0), Distance::au(1.5)],
    );

    println!(
        "    Gesamtbewohnbarkeit: {:.1}% (mit Trojaner-Berücksichtigung)",
        enhanced_habitability.overall_habitability * 100.0
    );

    // Zusätzliche Bewohnbarkeits-Bedingungen anzeigen
    if enhanced_habitability.habitability_conditions.len() > 3 {
        // Mehr als Standard
        println!("    Erweiterte Bedingungen:");
        for condition in enhanced_habitability.habitability_conditions.iter().skip(3) {
            println!("      • {}", condition);
        }
    }

    println!("\n{}", "=".repeat(50));
    println!("✨ System erfolgreich mit erweiterten Trojaner-Features generiert!");
    println!("   Trojaner-Dynamik, Stabilität und Bewohnbarkeit analysiert.");

    // Serialisierung testen
    match system.to_ron_string() {
        Ok(ron_data) => {
            println!(
                "\n💾 System erfolgreich nach RON serialisiert ({} Bytes).",
                ron_data.len()
            );
        }
        Err(e) => println!("  Fehler beim Serialisieren: {}", e),
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

// Zusätzliche Hilfsfunktionen für main.rs
fn print_trojan_details(
    lagrange_system: &LagrangeSystem,
    primary: &StellarProperties,
    secondary: &StellarProperties,
) {
    if lagrange_system.trojans.is_empty() {
        println!("      Keine Trojaner vorhanden");
        return;
    }

    println!("    Trojaner-Analyse:");
    for (i, trojan) in lagrange_system.trojans.iter().enumerate() {
        let dynamics = trojan.calculate_libration_dynamics(
            &primary.mass,
            &secondary.mass,
            &lagrange_system.separation,
        );

        println!(
            "      Trojaner {}: L{}, Masse: {:.2e} {}",
            i + 1,
            trojan.lagrange_point,
            trojan.mass.value,
            get_mass_unit_str(trojan.mass.system)
        );

        println!(
            "        Stabilität: {:.2}, Typ: {}",
            dynamics.long_term_stability,
            match dynamics.oscillation_pattern {
                OscillationPattern::Tadpole {
                    amplitude_degrees, ..
                } => format!("Tadpole ({:.1}°)", amplitude_degrees),
                OscillationPattern::Horseshoe { .. } => "Horseshoe".to_string(),
                OscillationPattern::QuasiStable { .. } => "Quasi-stable".to_string(),
            }
        );

        println!(
            "        Librations-Periode: {:.1} Jahre, Amplitude: {:.3} {}",
            dynamics.libration_period.in_years(),
            dynamics.libration_amplitude.value,
            get_distance_unit_str(dynamics.libration_amplitude.system)
        );

        println!(
            "        Säkulare Drift: {:.2e} AU/Myr",
            dynamics.secular_drift_rate
        );
    }
}

fn print_trojan_habitability(system: &StarSystem) {
    match &system.system_type {
        SystemType::Binary {
            primary,
            secondary,
            orbital_properties,
        } => {
            if let Some(ref lagrange_system) = orbital_properties.lagrange_system {
                if !lagrange_system.trojans.is_empty() {
                    println!("\n🏠 Trojaner-Bewohnbarkeits-Analyse:");

                    for trojan in &lagrange_system.trojans {
                        let trojan_hab = HabitabilityAssessment::calculate_trojan_habitability(
                            trojan,
                            primary,
                            secondary,
                            lagrange_system,
                        );

                        println!(
                            "    L{}-Trojaner Bewohnbarkeit: {:.1}%",
                            trojan.lagrange_point,
                            trojan_hab.habitability_score * 100.0
                        );

                        println!(
                            "      Temperatur-Stabilität: {:.2}, Hill-Schutz: {:.2}",
                            trojan_hab.temperature_stability, trojan_hab.hill_sphere_protection
                        );

                        println!(
                            "      Langzeit-Lebensfähigkeit: {:.1}%",
                            trojan_hab.long_term_viability * 100.0
                        );

                        if trojan_hab.habitability_score > 0.3 {
                            println!("      ✅ Potenzielle Bewohnbarkeit vorhanden!");
                        } else {
                            println!("      ❌ Schwierige Bedingungen für Leben");
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn print_system_stability_enhanced(system: &StarSystem) {
    let stability = SystemStability::analyze_system_enhanced(&system.system_type);

    println!("\n📊 Erweiterte System-Stabilität:");
    println!("    {}", stability.stability_summary());

    if let Some(ref trojan_analysis) = stability.trojan_analysis {
        println!("    Trojaner-Stabilität:");
        println!(
            "      Stabile Trojaner: {}, Instabile: {}",
            trojan_analysis.stable_trojans_count, trojan_analysis.unstable_trojans_count
        );

        if trojan_analysis.stable_trojans_count > 0 {
            println!(
                "      Durchschnittliche Trojaner-Stabilität: {:.1}%",
                trojan_analysis.average_trojan_stability * 100.0
            );
        }

        println!("    Lagrange-Punkte Status:");
        let lp_status = &trojan_analysis.lagrange_points_status;
        println!(
            "      L4: {}, L5: {} (Trojaner: L4={}, L5={})",
            if lp_status.l4_stable {
                "Stabil"
            } else {
                "Instabil"
            },
            if lp_status.l5_stable {
                "Stabil"
            } else {
                "Instabil"
            },
            lp_status.l4_trojans.len(),
            lp_status.l5_trojans.len()
        );

        // Trojaner-spezifische Risiken
        if !trojan_analysis.trojan_risks.is_empty() {
            println!("    Trojaner-Risiken:");
            for risk in &trojan_analysis.trojan_risks {
                println!(
                    "      ⚠️  {}: Schweregrad {:.2}, Wahrscheinlichkeit {:.1}%",
                    risk.name,
                    risk.severity,
                    risk.probability * 100.0
                );
            }
        }
    }

    if !stability.risk_factors.is_empty() {
        println!("    Allgemeine Stabilitäts-Risiken:");
        for risk in &stability.risk_factors {
            println!(
                "      ⚠️  {}: {:.1}% Wahrscheinlichkeit",
                risk.name,
                risk.probability * 100.0
            );
        }
    }
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
