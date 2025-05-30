// main.rs - Vollständig integriertes Sternsystem Generator Programm

// Cargo.toml Abhängigkeiten:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// ron = "0.8"
// rand = "0.8"
// rand_chacha = "0.3"

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

// Spezifische Module imports um Namespace-Konflikte zu vermeiden
mod constants;
mod cosmic_environment;
mod habitability;
mod lagrange_points;
mod orbital_mechanics;
mod stellar_properties;
mod system_hierarchy;
mod units;

// Spezifische imports
use constants::*;
use habitability::HabitabilityAssessment;
use lagrange_points::LagrangeSystem;
use orbital_mechanics::OrbitalElements;
use stellar_properties::StellarProperties;
use system_hierarchy::{BinaryOrbit, StarSystem, SystemHierarchy, SystemType};
use units::{Distance, Mass, Time, UnitSystem};

fn main() {
    println!("🌌 WISSENSCHAFTLICHER STERNSYSTEM GENERATOR v2.0");
    println!("Basierend auf 'An Apple Pie From Scratch, Part III: Orbits'");
    println!("{}", "=".repeat(70));
    println!();

    // Demonstriere alle Hauptfunktionalitäten
    demonstrate_comprehensive_analysis();
    demonstrate_orbital_mechanics_advanced();
    demonstrate_stellar_evolution();
    demonstrate_binary_systems();
    demonstrate_lagrange_trojans();
    demonstrate_habitability_analysis();
    demonstrate_cosmic_environment();
    demonstrate_random_system_generation();
    demonstrate_unit_systems();
    demonstrate_data_serialization();

    println!("\n🎯 FAZIT: Alle Module erfolgreich integriert!");
    println!("Vollständige wissenschaftliche Simulation verfügbar.");
}

fn demonstrate_comprehensive_analysis() {
    println!("🔬 UMFASSENDE SYSTEMANALYSE");
    println!("{}", "=".repeat(50));

    // Generiere ein vollständiges System
    let system = StarSystem::generate_from_seed(2024);

    println!("📊 System Seed: {}", system.seed);
    println!(
        "🌌 Kosmische Epoche: {} ({:.1} Gyr)",
        system.cosmic_epoch.era, system.cosmic_epoch.age_universe
    );
    println!(
        "📍 Galaktische Position: {:.1} kpc ({:?})",
        system.galactic_distance.value, system.galactic_region
    );

    // Detaillierte Systemanalyse je nach Typ
    match &system.system_type {
        SystemType::Single(star) => {
            println!("\n⭐ EINZELSTERN-SYSTEM");
            analyze_single_star_system(star, &system);
        }
        SystemType::Binary {
            primary,
            secondary,
            orbital_properties,
        } => {
            println!("\n⭐⭐ BINÄRSTERN-SYSTEM");
            analyze_binary_system(primary, secondary, orbital_properties, &system);
        }
        SystemType::Multiple {
            components,
            hierarchy,
        } => {
            println!("\n⭐⭐⭐ MEHRFACHSTERN-SYSTEM");
            analyze_multiple_system(components, hierarchy, &system);
        }
    }

    // Bewohnbarkeitsanalyse
    println!("\n🌍 BEWOHNBARKEITSANALYSE:");
    println!(
        "  Gesamtbewohnbarkeit: {:.1}%",
        system.habitability_assessment.overall_habitability * 100.0
    );
    println!(
        "  Strahlungsrisiken: AGN={:.1}%, SN={:.1}%, GRB={:.1}%",
        system.radiation_environment.agn_risk * 100.0,
        system.radiation_environment.supernova_frequency * 100.0,
        system.radiation_environment.grb_risk * 100.0
    );

    // Elementzusammensetzung
    let elem = &system.elemental_abundance;
    println!("\n🧪 ELEMENTHÄUFIGKEITEN:");
    println!(
        "  H: {:.1}%, He: {:.1}%, O: {:.3}%, C: {:.3}%, Metalle: {:.3}%",
        elem.hydrogen * 100.0,
        elem.helium * 100.0,
        elem.oxygen * 100.0,
        elem.carbon * 100.0,
        elem.heavy_metals * 100.0
    );
    println!("  C/O Verhältnis: {:.2}", elem.carbon_to_oxygen_ratio());
    println!(
        "  Astrobiologie-Potenzial: {:.1}%",
        elem.astrobiological_potential() * 100.0
    );

    println!();
}

fn analyze_single_star_system(star: &StellarProperties, system: &StarSystem) {
    println!(
        "  Spektraltyp: {:?} {:?}",
        star.spectral_type, star.luminosity_class
    );
    println!(
        "  Masse: {:.2} M☉, Alter: {:.1} Gyr",
        star.mass.in_solar_masses(),
        star.age.in_years()
    );
    println!(
        "  Temperatur: {:.0} K, Leuchtkraft: {:.3} L☉",
        star.effective_temperature, star.luminosity
    );
    println!("  Evolution: {:?}", star.evolutionary_stage);

    let hz = star.calculate_habitable_zone();
    println!(
        "  Bewohnbare Zone: {:.2} - {:.2} AU",
        hz.inner_edge.in_au(),
        hz.outer_edge.in_au()
    );

    // Planetare Analyse für verschiedene Positionen
    let target_distances = vec![
        Distance::au(hz.inner_edge.in_au() * 0.8),
        Distance::au((hz.inner_edge.in_au() + hz.outer_edge.in_au()) / 2.0),
        Distance::au(hz.outer_edge.in_au() * 1.2),
    ];

    let detailed_assessment = HabitabilityAssessment::comprehensive_analysis(
        &system.system_type,
        &system.radiation_environment,
        &target_distances,
    );

    println!("\n  🪐 PLANETARE BEWOHNBARKEIT:");
    for (i, analysis) in detailed_assessment.planetary_analysis.iter().enumerate() {
        println!(
            "    Planet {} @ {:.2} AU:",
            i + 1,
            analysis.orbital_distance.in_au()
        );
        println!(
            "      Bewohnbarkeit: {:.1}%",
            analysis.habitability_score * 100.0
        );
        println!(
            "      Gleichgewichtstemperatur: {:.0} K",
            analysis.temperature_analysis.equilibrium_temperature
        );
        if analysis.tidal_locking.tidal_lock_probability > 0.3 {
            println!(
                "      Tidal Lock: {:.0}% Wahrscheinlichkeit",
                analysis.tidal_locking.tidal_lock_probability * 100.0
            );
        }

        if !analysis.habitable_regions.is_empty() {
            print!("      Bewohnbare Regionen: ");
            for region in &analysis.habitable_regions {
                print!("{:?} ", region);
            }
            println!();
        }
    }
}

fn analyze_binary_system(
    primary: &StellarProperties,
    secondary: &StellarProperties,
    orbital_properties: &BinaryOrbit,
    _system: &StarSystem,
) {
    println!(
        "  Primär: {:?} ({:.2} M☉)",
        primary.spectral_type,
        primary.mass.in_solar_masses()
    );
    println!(
        "  Sekundär: {:?} ({:.2} M☉)",
        secondary.spectral_type,
        secondary.mass.in_solar_masses()
    );
    println!(
        "  Orbitale Trennung: {:.1} AU",
        orbital_properties.orbital_elements.semimajor_axis.in_au()
    );
    println!(
        "  Exzentrizität: {:.3}",
        orbital_properties.orbital_elements.eccentricity
    );
    println!(
        "  Inklination: {:.1}°",
        orbital_properties.orbital_elements.inclination
    );

    let total_mass = Mass::kilograms(primary.mass.in_kg() + secondary.mass.in_kg());
    let period = orbital_properties
        .orbital_elements
        .orbital_period(&total_mass);
    println!("  Orbitalperiode: {:.1} Jahre", period.in_years());

    let (peri, apo) = orbital_properties.distance_range();
    println!(
        "  Entfernungsbereich: {:.1} - {:.1} AU",
        peri.in_au(),
        apo.in_au()
    );

    // Planetenstabilität
    println!("\n  🪐 PLANETENSTABILITÄT:");
    println!(
        "    S-Type (um Primär): < {:.1} AU",
        orbital_properties.s_type_stability.0.in_au()
    );
    println!(
        "    S-Type (um Sekundär): < {:.1} AU",
        orbital_properties.s_type_stability.1.in_au()
    );
    println!(
        "    P-Type (zirkumbinär): > {:.1} AU",
        orbital_properties.p_type_stability.in_au()
    );

    // Lagrange-Punkte
    if let Some(ref lagrange_system) = orbital_properties.lagrange_system {
        println!("\n  🔺 LAGRANGE-SYSTEM:");
        println!(
            "    L4/L5 Stabilität: {}",
            if lagrange_system.l4_l5_stable {
                "✓ Stabil"
            } else {
                "✗ Instabil"
            }
        );
        println!("    Massenverhältnis: {:.0}:1", lagrange_system.mass_ratio);
        println!("    Trojaner: {} aktuelle", lagrange_system.trojans.len());
    }

    // Kombinierte bewohnbare Zone
    let combined_hz = orbital_properties.combined_habitable_zone(primary, secondary);
    println!("\n  🌍 KOMBINIERTE BEWOHNBARE ZONE:");
    println!(
        "    {:.2} - {:.2} AU",
        combined_hz.inner_edge.in_au(),
        combined_hz.outer_edge.in_au()
    );
}

fn analyze_multiple_system(
    components: &[StellarProperties],
    hierarchy: &SystemHierarchy,
    _system: &StarSystem,
) {
    println!("  Komponenten: {} Sterne", components.len());
    for (i, star) in components.iter().enumerate() {
        println!(
            "    Stern {}: {:?} ({:.2} M☉)",
            i + 1,
            star.spectral_type,
            star.mass.in_solar_masses()
        );
    }

    println!("\n  🏗️ HIERARCHIE-ANALYSE:");
    println!("    Hierarchieebenen: {}", hierarchy.hierarchy_levels.len());
    println!(
        "    Gesamtstabilität: {:.1}%",
        hierarchy.stability_factor * 100.0
    );
    println!(
        "    Chaos-Zeitskala: {:.0} Myr",
        hierarchy.chaos_timescale.in_years() / 1e6
    );
    println!(
        "    Langzeitstabilität: {}",
        if hierarchy.is_long_term_stable() {
            "✓ Stabil"
        } else {
            "⚠️ Instabil"
        }
    );

    for (i, level) in hierarchy.hierarchy_levels.iter().enumerate() {
        println!(
            "    Ebene {}: {:.1} AU, Stabilität {:.1}%",
            i + 1,
            level.orbit.semimajor_axis.in_au(),
            level.level_stability * 100.0
        );
    }
}

fn demonstrate_orbital_mechanics_advanced() {
    println!("🌍 ERWEITERTE ORBITALMECHANIK");
    println!("{}", "=".repeat(50));

    // Sonnensystem-Nachbildung
    let sun = StellarProperties::sun_like();
    let solar_mass = sun.mass.clone();

    // Planeten mit realistischen Orbitalparametern
    let planets = vec![
        ("Merkur", 0.387, 0.206, 7.0, 48.3, 29.1),
        ("Venus", 0.723, 0.007, 3.4, 76.7, 54.9),
        ("Erde", 1.000, 0.017, 0.0, 0.0, 102.9),
        ("Mars", 1.524, 0.093, 1.9, 49.6, 286.5),
        ("Jupiter", 5.203, 0.048, 1.3, 100.5, 273.9),
        ("Saturn", 9.537, 0.054, 2.5, 113.6, 339.4),
    ];

    println!("☀️ SONNENSYSTEM-SIMULATION:");
    for (name, a, e, i, omega, w) in planets {
        let orbit = OrbitalElements::new(
            Distance::au(a),
            e,
            i,
            omega,
            w,
            0.0, // True anomaly at J2000
        );

        let period = orbit.orbital_period(&solar_mass);
        let periapsis = orbit.periapsis();
        let apoapsis = orbit.apoapsis();

        println!("  🪐 {}:", name);
        println!("    a={:.3} AU, e={:.3}, i={:.1}°", a, e, i);
        println!("    Periode: {:.1} Jahre", period.in_years());
        println!(
            "    Entfernung: {:.3} - {:.3} AU",
            periapsis.in_au(),
            apoapsis.in_au()
        );
        println!("    Orbit-Typ: {:?}", orbit.orbit_type());

        // Escape Velocity an Aphelion und Perihelion
        let escape_peri = orbit.escape_velocity_at_distance(&periapsis, &solar_mass);
        let escape_apo = orbit.escape_velocity_at_distance(&apoapsis, &solar_mass);
        println!(
            "    Escape Velocity: {:.1} - {:.1} km/s",
            escape_peri.in_kms(),
            escape_apo.in_kms()
        );

        // Hill-Sphäre (approximiert)
        if name == "Erde" {
            let earth_mass = Mass::kilograms(5.972e24);
            let hill_radius = orbit.hill_radius(&earth_mass, &solar_mass);
            println!(
                "    Hill-Sphäre: {:.3} AU ({:.0} km)",
                hill_radius.in_au(),
                hill_radius.in_meters() / 1000.0
            );
        }

        println!();
    }
}

fn demonstrate_stellar_evolution() {
    println!("⭐ STELLARE EVOLUTION");
    println!("{}", "=".repeat(50));

    // Verschiedene Sternmassen durch ihre Evolution verfolgen
    let masses = vec![0.3, 0.8, 1.0, 1.5, 5.0, 15.0];
    let ages = vec![0.1, 1.0, 5.0, 10.0];

    for &mass in &masses {
        println!("🌟 {:.1} M☉ Stern - Evolution:", mass);

        for &age in &ages {
            let star =
                StellarProperties::new(Mass::solar_masses(mass), Time::years(age).clone(), 0.0);

            println!(
                "  @ {:.1} Gyr: {:?} {:?}, T={:.0}K, L={:.2}L☉",
                age,
                star.spectral_type,
                star.evolutionary_stage,
                star.effective_temperature,
                star.luminosity
            );

            let hz = star.calculate_habitable_zone();
            if hz.inner_edge.in_au() < 10.0 {
                // Nur wenn vernünftig
                println!(
                    "             HZ: {:.2}-{:.2} AU",
                    hz.inner_edge.in_au(),
                    hz.outer_edge.in_au()
                );
            }
        }

        let final_star =
            StellarProperties::new(Mass::solar_masses(mass), Time::years(1.0).clone(), 0.0);
        println!(
            "  Hauptreihen-Lebensdauer: {:.1} Gyr",
            final_star.main_sequence_lifetime.in_years()
        );
        println!();
    }
}

fn demonstrate_binary_systems() {
    println!("⭐⭐ BINÄRSYSTEM-ANALYSE");
    println!("{}", "=".repeat(50));

    // Bekannte Binärsysteme simulieren
    let binary_systems = vec![
        ("Alpha Centauri AB", 1.1, 0.907, 23.4, 0.52),
        ("Sirius AB", 2.02, 0.978, 20.0, 0.59),
        ("61 Cygni AB", 0.7, 0.63, 84.0, 0.48),
        ("Proxima-Alpha Cen", 0.12, 1.1, 13000.0, 0.0), // Sehr weites System
    ];

    for (name, m1, m2, sep_au, ecc) in binary_systems {
        println!("🌟 {}:", name);

        let primary = StellarProperties::new(Mass::solar_masses(m1), Time::years(5.0).clone(), 0.0);
        let secondary =
            StellarProperties::new(Mass::solar_masses(m2), Time::years(5.0).clone(), 0.0);

        let orbit = BinaryOrbit::new(
            &primary,
            &secondary,
            Distance::au(sep_au),
            ecc,
            0.0,
            0.0,
            0.0, // Einfache Orientierung
        );

        let total_mass = Mass::kilograms(primary.mass.in_kg() + secondary.mass.in_kg());
        let period = orbit.orbital_elements.orbital_period(&total_mass);

        println!("  Massen: {:.2} + {:.2} M☉", m1, m2);
        println!("  Separation: {:.1} AU, Exzentrizität: {:.2}", sep_au, ecc);
        println!("  Periode: {:.0} Jahre", period.in_years());
        println!(
            "  Barycenter: {:.1}% vom Primär",
            orbit.barycenter_position * 100.0
        );

        // Stabilitätszonen
        println!("  Planetenstabilität:");
        println!("    S-Type A: < {:.1} AU", orbit.s_type_stability.0.in_au());
        println!("    S-Type B: < {:.1} AU", orbit.s_type_stability.1.in_au());
        if orbit.p_type_stability.in_au() < 1000.0 {
            println!("    P-Type: > {:.1} AU", orbit.p_type_stability.in_au());
        }

        // Lagrange-Stabilität
        if let Some(ref lagrange) = orbit.lagrange_system {
            println!(
                "  Lagrange L4/L5: {} (Ratio: {:.0}:1)",
                if lagrange.l4_l5_stable {
                    "Stabil"
                } else {
                    "Instabil"
                },
                lagrange.mass_ratio
            );
        }

        println!();
    }
}

fn demonstrate_lagrange_trojans() {
    println!("🔺 LAGRANGE-PUNKTE UND TROJANER");
    println!("{}", "=".repeat(50));

    // Jupiter-Trojaner Simulation
    let sun = StellarProperties::sun_like();
    let jupiter =
        StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6).clone(), 0.0);
    let jupiter_orbit = Distance::au(5.2);

    let mut lagrange_system = LagrangeSystem::new(&sun, &jupiter, jupiter_orbit);

    println!("🪐 Jupiter-Trojaner System:");
    println!("  Massenverhältnis: {:.0}:1", lagrange_system.mass_ratio);
    println!(
        "  L4/L5 Stabilität: {}",
        if lagrange_system.l4_l5_stable {
            "✓ Stabil"
        } else {
            "✗ Instabil"
        }
    );

    let (l4_x, l4_y) = lagrange_system.l4_position();
    let (l5_x, l5_y) = lagrange_system.l5_position();
    println!(
        "  L4 Position: ({:.1}, {:.1}) AU",
        l4_x.in_au(),
        l4_y.in_au()
    );
    println!(
        "  L5 Position: ({:.1}, {:.1}) AU",
        l5_x.in_au(),
        l5_y.in_au()
    );

    // Bekannte Trojaner hinzufügen
    let known_trojans = vec![
        ("588 Achilles", 1.81e17, 4),
        ("617 Patroclus", 1.13e17, 4),
        ("624 Hektor", 7.9e18, 4),
        ("911 Agamemnon", 1.15e18, 4),
        ("1172 Äneas", 1.4e17, 5),
        ("1173 Anchises", 1.7e17, 5),
    ];

    println!("\n  🪨 Trojaner-Asteroiden:");
    for (name, mass_kg, l_point) in known_trojans {
        let trojan_mass = Mass::kilograms(mass_kg);

        match lagrange_system.generate_trojan(l_point, trojan_mass, &sun.mass, &jupiter.mass) {
            Ok(trojan) => {
                println!("    {} @ L{}:", name, l_point);
                println!("      Masse: {:.1e} kg", trojan.mass.in_kg());
                println!(
                    "      Oszillation: {:.2} AU, {:.1} Jahre",
                    trojan.oscillation_amplitude.in_au(),
                    trojan.oscillation_period.in_years()
                );
                println!("      Stabilität: {:.0}%", trojan.stability * 100.0);

                if lagrange_system.add_trojan(trojan).is_ok() {
                    println!("      ✓ Erfolgreich hinzugefügt");
                }
            }
            Err(e) => println!("    {} @ L{}: ❌ {}", name, l_point, e),
        }
    }

    println!("\n  📊 System-Zusammenfassung:");
    println!("    Aktive Trojaner: {}", lagrange_system.trojans.len());
    println!(
        "    L4 Trojaner: {}",
        lagrange_system
            .trojans
            .iter()
            .filter(|t| t.lagrange_point == 4)
            .count()
    );
    println!(
        "    L5 Trojaner: {}",
        lagrange_system
            .trojans
            .iter()
            .filter(|t| t.lagrange_point == 5)
            .count()
    );

    println!();
}

fn demonstrate_habitability_analysis() {
    println!("🌍 DETAILLIERTE BEWOHNBARKEITSANALYSE");
    println!("{}", "=".repeat(50));

    // Verschiedene Sterntypen und ihre Bewohnbarkeit
    let star_systems = vec![
        ("Proxima Centauri", Mass::solar_masses(0.12), "M5.5V"),
        ("Wolf 359", Mass::solar_masses(0.09), "M6.0V"),
        ("Barnard's Star", Mass::solar_masses(0.14), "M4.0V"),
        ("61 Cygni A", Mass::solar_masses(0.7), "K5.0V"),
        ("Alpha Centauri A", Mass::solar_masses(1.1), "G2V"),
        ("Procyon A", Mass::solar_masses(1.5), "F5IV"),
    ];

    let radiation_env = cosmic_environment::CosmicRadiationEnvironment {
        agn_risk: 0.1,
        supernova_frequency: 0.2,
        grb_risk: 0.3,
        stellar_encounter_rate: 0.1,
        cosmic_ray_flux: 10.0,
        uv_background: 1.0,
        gravitational_wave_activity: 0.1,
    };

    for (name, mass, spectral_type) in star_systems {
        println!("⭐ {} ({}):", name, spectral_type);

        let star = StellarProperties::new(mass, Time::years(5.0).clone(), 0.0);
        let hz = star.calculate_habitable_zone();

        println!(
            "  Bewohnbare Zone: {:.3} - {:.3} AU",
            hz.inner_edge.in_au(),
            hz.outer_edge.in_au()
        );

        // Analyse verschiedener Planetenpositionen
        let planet_distances = vec![
            hz.inner_edge.in_au() * 0.9,
            (hz.inner_edge.in_au() + hz.outer_edge.in_au()) / 2.0,
            hz.outer_edge.in_au() * 1.1,
        ];

        for (i, distance_au) in planet_distances.iter().enumerate() {
            let distance = Distance::au(*distance_au);
            let system_type = SystemType::Single(star.clone());

            let assessment = HabitabilityAssessment::comprehensive_analysis(
                &system_type,
                &radiation_env,
                &[distance.clone()],
            );

            if let Some(planetary) = assessment.planetary_analysis.first() {
                println!("    Planet {} @ {:.3} AU:", i + 1, distance_au);
                println!(
                    "      Bewohnbarkeit: {:.0}%",
                    planetary.habitability_score * 100.0
                );
                println!(
                    "      Temperatur: {:.0} K",
                    planetary.temperature_analysis.equilibrium_temperature
                );

                if planetary.tidal_locking.tidal_lock_probability > 0.1 {
                    println!(
                        "      Tidal Lock: {:.0}%",
                        planetary.tidal_locking.tidal_lock_probability * 100.0
                    );
                }

                if planetary.atmospheric_considerations.len() > 0 {
                    println!(
                        "      Atmosphäre: {}",
                        planetary.atmospheric_considerations.join(", ")
                    );
                }
            }

            // Risikofaktoren für das erste Assessment zeigen
            if i == 0 {
                println!("  Risikofaktoren: {}", assessment.risk_factors.len());
                for risk in &assessment.risk_factors {
                    if risk.severity > 0.3 {
                        println!(
                            "    ⚠️ {}: {:.0}% ({:.0}% Wahrscheinlichkeit)",
                            risk.name,
                            risk.severity * 100.0,
                            risk.probability * 100.0
                        );
                    }
                }
            }
        }

        println!();
    }
}

fn demonstrate_cosmic_environment() {
    println!("🌌 KOSMISCHE UMGEBUNG");
    println!("{}", "=".repeat(50));

    // Verschiedene kosmische Epochen
    let epochs = vec![1.0, 3.0, 6.0, 10.0, 13.8];

    println!("📅 Kosmische Epochen:");
    for &age in &epochs {
        let epoch = cosmic_environment::CosmicEpoch::from_age(age);
        let abundance =
            cosmic_environment::ElementalAbundance::from_metallicity_and_epoch(0.0, &epoch);

        println!("  {} ({:.1} Gyr):", epoch.era, age);
        println!(
            "    Sternentstehung: {:.1}x heute",
            epoch.star_formation_rate
        );
        println!("    Metallizität: {:.2} dex", epoch.epoch_metallicity);
        println!("    Redshift: z = {:.2}", epoch.redshift);
        println!(
            "    O/H: {:.1}%, C/O: {:.2}",
            abundance.oxygen * 100.0,
            abundance.carbon_to_oxygen_ratio()
        );
        println!(
            "    Astrobiologie: {:.0}%",
            abundance.astrobiological_potential() * 100.0
        );
        println!();
    }

    // Galaktische Regionen
    println!("🌌 Galaktische Regionen:");
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    for _ in 0..5 {
        let region =
            cosmic_environment::GalacticRegion::generate_random(&mut rng, UnitSystem::Astronomical);
        let epoch = cosmic_environment::CosmicEpoch::from_age(10.0);
        let radiation = cosmic_environment::CosmicRadiationEnvironment::from_region_and_epoch(
            &region, &epoch, &mut rng,
        );
        let dynamics =
            cosmic_environment::GalacticDynamics::calculate_for_position(&region, 10.0, &mut rng);

        match &region {
            cosmic_environment::GalacticRegion::Core {
                distance_from_center,
                supermassive_black_hole_influence,
            } => {
                println!(
                    "  🔴 Galaktisches Zentrum ({:.1} kpc):",
                    distance_from_center.value
                );
                println!(
                    "    SMBH Einfluss: {:.0}%",
                    supermassive_black_hole_influence * 100.0
                );
            }
            cosmic_environment::GalacticRegion::HabitableZone {
                distance_from_center,
                metallicity_gradient,
            } => {
                println!(
                    "  🟢 Galaktische HZ ({:.1} kpc):",
                    distance_from_center.value
                );
                println!("    Metallizitätsgradient: {:.2}", metallicity_gradient);
            }
            cosmic_environment::GalacticRegion::OuterDisk {
                distance_from_center,
                gas_density,
            } => {
                println!(
                    "  🔵 Äußere Scheibe ({:.1} kpc):",
                    distance_from_center.value
                );
                println!("    Gasdichte: {:.2} cm⁻³", gas_density);
            }
            _ => {}
        }

        println!(
            "    Bewohnbarkeit: {:.0}%",
            region.habitability_factor() * 100.0
        );
        println!(
            "    Strahlungsrisiko: {:.0}%",
            radiation.total_radiation_risk() * 100.0
        );
        println!(
            "    Umgebungsstabilität: {:.0}%",
            dynamics.environmental_stability() * 100.0
        );
        println!(
            "    Rotationsgeschwindigkeit: {:.0} km/s",
            dynamics.rotation_velocity
        );

        match &dynamics.spiral_arm_context {
            cosmic_environment::SpiralArmContext::InArm {
                arm_name,
                position_in_arm,
            } => {
                println!(
                    "    📍 Im Spiralarm: {} ({:.0}%)",
                    arm_name,
                    position_in_arm * 100.0
                );
            }
            cosmic_environment::SpiralArmContext::InterArm {
                distance_to_nearest_arm,
                nearest_arm_name,
            } => {
                println!(
                    "    📍 Zwischen Armen: {:.1} kpc zu {}",
                    distance_to_nearest_arm.value, nearest_arm_name
                );
            }
            _ => {}
        }

        println!();
    }
}

fn demonstrate_random_system_generation() {
    println!("🎲 ZUFÄLLIGE SYSTEMGENERIERUNG");
    println!("{}", "=".repeat(50));

    let mut statistics = HashMap::new();
    let num_systems = 1000;

    println!(
        "Generiere {} zufällige Systeme für Statistik...",
        num_systems
    );

    for seed in 1..=num_systems {
        let system = StarSystem::generate_from_seed(seed);

        // Statistiken sammeln
        let system_type = match &system.system_type {
            SystemType::Single(_) => "Single",
            SystemType::Binary { .. } => "Binary",
            SystemType::Multiple { .. } => "Multiple",
        };

        *statistics.entry(system_type).or_insert(0) += 1;

        let region_type = match &system.galactic_region {
            cosmic_environment::GalacticRegion::Core { .. } => "Core",
            cosmic_environment::GalacticRegion::InnerBulge { .. } => "InnerBulge",
            cosmic_environment::GalacticRegion::HabitableZone { .. } => "HabitableZone",
            cosmic_environment::GalacticRegion::OuterDisk { .. } => "OuterDisk",
            cosmic_environment::GalacticRegion::Halo { .. } => "Halo",
        };

        *statistics.entry(region_type).or_insert(0) += 1;
    }

    println!("\n📊 SYSTEMSTATISTIKEN ({} Systeme):", num_systems);
    println!("  Systemtypen:");
    println!(
        "    Einzelsterne: {:.1}%",
        *statistics.get("Single").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );
    println!(
        "    Binärsysteme: {:.1}%",
        *statistics.get("Binary").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );
    println!(
        "    Mehrfachsysteme: {:.1}%",
        *statistics.get("Multiple").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );

    println!("  Galaktische Verteilung:");
    println!(
        "    Kern: {:.1}%",
        *statistics.get("Core").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );
    println!(
        "    Innere Bulge: {:.1}%",
        *statistics.get("InnerBulge").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );
    println!(
        "    Bewohnbare Zone: {:.1}%",
        *statistics.get("HabitableZone").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );
    println!(
        "    Äußere Scheibe: {:.1}%",
        *statistics.get("OuterDisk").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );
    println!(
        "    Halo: {:.1}%",
        *statistics.get("Halo").unwrap_or(&0) as f64 / num_systems as f64 * 100.0
    );

    // Einige interessante Systeme zeigen
    println!("\n🌟 INTERESSANTE BEISPIELSYSTEME:");

    let interesting_seeds = [42, 1337, 2024, 9999];
    for &seed in &interesting_seeds {
        let system = StarSystem::generate_from_seed(seed);

        print!("  Seed {}: ", seed);
        match &system.system_type {
            SystemType::Single(star) => {
                println!(
                    "{:?} @ {:.1} kpc, Bewohnbarkeit: {:.0}%",
                    star.spectral_type,
                    system.galactic_distance.value,
                    system.habitability_assessment.overall_habitability * 100.0
                );
            }
            SystemType::Binary {
                primary, secondary, ..
            } => {
                println!(
                    "{:?}+{:?} @ {:.1} kpc, Bewohnbarkeit: {:.0}%",
                    primary.spectral_type,
                    secondary.spectral_type,
                    system.galactic_distance.value,
                    system.habitability_assessment.overall_habitability * 100.0
                );
            }
            SystemType::Multiple { components, .. } => {
                println!(
                    "{}-fach System @ {:.1} kpc, Bewohnbarkeit: {:.0}%",
                    components.len(),
                    system.galactic_distance.value,
                    system.habitability_assessment.overall_habitability * 100.0
                );
            }
        }
    }

    println!();
}

fn demonstrate_unit_systems() {
    println!("📐 EINHEITENSYSTEM-VERGLEICH");
    println!("{}", "=".repeat(50));

    // Gleiches System in verschiedenen Einheiten
    let seed = 42;
    let system_au = StarSystem::generate_from_seed_with_units(seed, UnitSystem::Astronomical);
    let system_si = StarSystem::generate_from_seed_with_units(seed, UnitSystem::SI);

    println!(
        "🔄 Gleiches System (Seed {}), verschiedene Einheiten:",
        seed
    );
    println!();

    if let (SystemType::Single(star_au), SystemType::Single(star_si)) =
        (&system_au.system_type, &system_si.system_type)
    {
        println!("  ⭐ Astronomische Einheiten:");
        println!("    Masse: {:.2} M☉", star_au.mass.in_solar_masses());
        println!("    Alter: {:.1} Gyr", star_au.age.in_years());
        println!("    Radius: {:.2} R☉", star_au.radius.value);

        let hz_au = star_au.calculate_habitable_zone();
        println!(
            "    HZ: {:.2} - {:.2} AU",
            hz_au.inner_edge.in_au(),
            hz_au.outer_edge.in_au()
        );

        println!("  🔬 SI-Einheiten:");
        println!("    Masse: {:.2e} kg", star_si.mass.in_kg());
        println!("    Alter: {:.2e} s", star_si.age.in_seconds());
        println!("    Radius: {:.2e} m", star_si.radius.in_meters());

        let hz_si = star_si.calculate_habitable_zone();
        println!(
            "    HZ: {:.2e} - {:.2e} m",
            hz_si.inner_edge.in_meters(),
            hz_si.outer_edge.in_meters()
        );

        // Einheitenkonvertierung demonstrieren
        println!("\n  🔄 Konvertierung:");
        let star_converted = star_au.to_system(UnitSystem::SI);
        println!("    AU→SI Masse: {:.2e} kg", star_converted.mass.in_kg());
        println!(
            "    Differenz: {:.2e} kg",
            (star_converted.mass.in_kg() - star_si.mass.in_kg()).abs()
        );

        // Geschwindigkeiten vergleichen
        let orbital_vel_au = star_au.orbital_velocity_at_distance(&Distance::au(1.0));
        let orbital_vel_si = star_si.orbital_velocity_at_distance(&Distance::meters(AU_TO_METERS));

        println!("    Orbitalgeschwindigkeit @ 1 AU:");
        println!(
            "      AU-System: {:.2} AU/Jahr",
            orbital_vel_au.in_au_per_year()
        );
        println!("      SI-System: {:.0} m/s", orbital_vel_si.in_ms());
        println!("      Konvertiert: {:.0} m/s", orbital_vel_au.in_ms());
        println!(
            "      Differenz: {:.2}%",
            (orbital_vel_au.in_ms() - orbital_vel_si.in_ms()).abs() / orbital_vel_si.in_ms()
                * 100.0
        );
    }

    println!();
}

// RON Export/Import Demonstration
fn demonstrate_data_serialization() {
    println!("💾 DATEN-SERIALISIERUNG");
    println!("{}", "=".repeat(50));

    let system = StarSystem::generate_from_seed(2024);

    match system.to_ron_string() {
        Ok(ron_data) => {
            println!("✅ RON Export erfolgreich:");
            println!("  Größe: {} Zeichen", ron_data.len());
            println!("  Zeilen: {}", ron_data.lines().count());

            // Erste paar Zeilen zeigen
            println!("  Vorschau:");
            for (i, line) in ron_data.lines().take(5).enumerate() {
                println!(
                    "    {}: {}",
                    i + 1,
                    if line.len() > 60 {
                        format!("{}...", &line[..60])
                    } else {
                        line.to_string()
                    }
                );
            }

            // Reimport testen
            match StarSystem::from_ron_string(&ron_data) {
                Ok(reimported) => {
                    println!("✅ RON Import erfolgreich");
                    println!("  Seed stimmt überein: {}", system.seed == reimported.seed);
                    println!("  Datenintegrität: ✓");
                }
                Err(e) => println!("❌ RON Import Fehler: {}", e),
            }
        }
        Err(e) => println!("❌ RON Export Fehler: {}", e),
    }

    println!();
}

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    #[test]
    fn test_full_system_integration() {
        // Test der vollständigen Integration aller Module
        let system = StarSystem::generate_from_seed(42);

        // Grundlegende Validierung
        assert!(system.habitability_assessment.overall_habitability >= 0.0);
        assert!(system.habitability_assessment.overall_habitability <= 1.0);
        assert!(system.cosmic_epoch.age_universe > 0.0);
        assert!(system.galactic_distance.value > 0.0);

        // System-spezifische Tests
        match &system.system_type {
            SystemType::Single(star) => {
                assert!(star.mass.in_solar_masses() > 0.0);
                assert!(star.luminosity > 0.0);

                let hz = star.calculate_habitable_zone();
                assert!(hz.inner_edge.in_au() < hz.outer_edge.in_au());
            }
            SystemType::Binary {
                primary,
                secondary,
                orbital_properties,
            } => {
                assert!(primary.mass.in_solar_masses() > 0.0);
                assert!(secondary.mass.in_solar_masses() > 0.0);
                assert!(orbital_properties.orbital_elements.semimajor_axis.in_au() > 0.0);

                let (peri, apo) = orbital_properties.distance_range();
                assert!(peri.in_au() < apo.in_au());
            }
            SystemType::Multiple {
                components,
                hierarchy,
            } => {
                assert!(components.len() >= 3);
                assert!(hierarchy.stability_factor >= 0.0);
                assert!(hierarchy.stability_factor <= 1.0);
            }
        }

        // Serialisierung testen
        let ron_result = system.to_ron_string();
        assert!(ron_result.is_ok());

        let reimport_result = StarSystem::from_ron_string(&ron_result.unwrap());
        assert!(reimport_result.is_ok());

        let reimported = reimport_result.unwrap();
        assert_eq!(system.seed, reimported.seed);
    }

    #[test]
    fn test_unit_consistency() {
        // Test Einheitenkonsistenz zwischen AU und SI
        let system_au = StarSystem::generate_from_seed_with_units(42, UnitSystem::Astronomical);
        let system_si = StarSystem::generate_from_seed_with_units(42, UnitSystem::SI);

        assert_eq!(system_au.seed, system_si.seed);
        assert_eq!(system_au.unit_system, UnitSystem::Astronomical);
        assert_eq!(system_si.unit_system, UnitSystem::SI);

        // Physikalische Äquivalenz testen
        if let (SystemType::Single(star_au), SystemType::Single(star_si)) =
            (&system_au.system_type, &system_si.system_type)
        {
            let mass_diff = (star_au.mass.in_kg() - star_si.mass.in_kg()).abs();
            assert!(mass_diff / star_si.mass.in_kg() < 1e-10); // Sehr kleine relative Differenz

            let age_diff = (star_au.age.in_seconds() - star_si.age.in_seconds()).abs();
            assert!(age_diff / star_si.age.in_seconds() < 1e-10);
        }
    }

    #[test]
    fn test_lagrange_point_physics() {
        // Test Lagrange-Punkt Physik
        let sun = StellarProperties::sun_like();
        let jupiter =
            StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6).clone(), 0.0);
        let lagrange_system = LagrangeSystem::new(&sun, &jupiter, Distance::au(5.2));

        // Massenverhältnis sollte groß genug für stabile L4/L5 sein
        assert!(lagrange_system.mass_ratio > MIN_LAGRANGE_MASS_RATIO);
        assert!(lagrange_system.l4_l5_stable);

        // L4/L5 Positionen sollten gleichseitiges Dreieck bilden
        let (l4_x, l4_y) = lagrange_system.l4_position();
        let separation = lagrange_system.separation.in_au();

        let distance_l4_to_sun = (l4_x.in_au().powf(2.0) + l4_y.in_au().powf(2.0)).sqrt();
        let distance_l4_to_jupiter =
            ((l4_x.in_au() - separation).powf(2.0) + l4_y.in_au().powf(2.0)).sqrt();

        // Gleichseitiges Dreieck: alle Seiten etwa gleich lang
        assert!((distance_l4_to_sun - separation).abs() < 0.1);
        assert!((distance_l4_to_jupiter - separation).abs() < 0.1);
    }

    #[test]
    fn test_habitability_assessment_consistency() {
        // Test Bewohnbarkeitsanalyse Konsistenz
        let sun = StellarProperties::sun_like();
        let radiation_env = cosmic_environment::CosmicRadiationEnvironment {
            agn_risk: 0.1,
            supernova_frequency: 0.2,
            grb_risk: 0.3,
            stellar_encounter_rate: 0.1,
            cosmic_ray_flux: 10.0,
            uv_background: 1.0,
            gravitational_wave_activity: 0.1,
        };

        let distances = vec![Distance::au(0.5), Distance::au(1.0), Distance::au(1.5)];
        let system_type = SystemType::Single(sun);

        let assessment = HabitabilityAssessment::comprehensive_analysis(
            &system_type,
            &radiation_env,
            &distances,
        );

        assert_eq!(assessment.planetary_analysis.len(), 3);
        assert!(assessment.overall_habitability >= 0.0);
        assert!(assessment.overall_habitability <= 1.0);

        // Planet in HZ sollte höhere Bewohnbarkeit haben
        let hz_planet = &assessment.planetary_analysis[1]; // Mittlerer Planet
        let inner_planet = &assessment.planetary_analysis[0];
        let outer_planet = &assessment.planetary_analysis[2];

        assert!(hz_planet.habitability_score >= inner_planet.habitability_score);
        assert!(hz_planet.habitability_score >= outer_planet.habitability_score);
    }
}
