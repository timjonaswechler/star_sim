// main.rs - Erweiterte Hauptdatei mit modularem System

// Cargo.toml erweitert um:
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// ron = "0.8"
// rand = "0.8"
// rand_chacha = "0.3"

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// Import der neuen Module
mod constants;
mod lagrange_points;
mod orbital_mechanics;
mod stellar_properties;
mod units;

use constants::*;
use lagrange_points::*;
use orbital_mechanics::*;
use stellar_properties::*;
use units::*;

fn main() {
    println!("=== Wissenschaftlicher Sternsystem Generator v2.0 ===");
    println!("Basierend auf 'An Apple Pie From Scratch, Part III: Orbits'\n");

    demonstrate_orbital_mechanics();
    demonstrate_stellar_systems();
    demonstrate_lagrange_points();
    demonstrate_unit_conversions();
    demonstrate_escape_velocities();
}

fn demonstrate_orbital_mechanics() {
    println!("🌍 ORBITALMECHANIK DEMONSTRATION");
    println!("{}", "=".repeat(50));

    // Erde um Sonne
    let earth_orbit = OrbitalElements::new(
        Distance::au(1.0),
        0.0167, // Erde's Exzentrizität
        0.0,    // Inklination zur Ekliptik
        0.0,    // Longitude of ascending node
        102.9,  // Argument of periapsis
        0.0,    // True anomaly at J2000
    );

    let solar_mass = Mass::solar_masses(1.0);
    let period = earth_orbit.orbital_period(&solar_mass);

    println!("🌍 Erde-Sonne System:");
    println!(
        "  Semi-major axis: {:.3} AU",
        earth_orbit.semimajor_axis.in_au()
    );
    println!("  Exzentrizität: {:.4}", earth_orbit.eccentricity);
    println!("  Orbitalperiode: {:.2} Jahre", period.in_years());
    println!(
        "  Periapsis (Perihel): {:.3} AU",
        earth_orbit.periapsis().in_au()
    );
    println!(
        "  Apoapsis (Aphel): {:.3} AU",
        earth_orbit.apoapsis().in_au()
    );

    let earth_mass = Mass::kilograms(5.972e24);
    let hill_radius = earth_orbit.hill_radius(&earth_mass, &solar_mass);
    println!(
        "  Erde's Hill-Sphäre: {:.3} AU ({:.0} km)",
        hill_radius.in_au(),
        hill_radius.in_meters() / 1000.0
    );

    // Escape Velocity von verschiedenen Positionen
    let escape_vel_periapsis =
        earth_orbit.escape_velocity_at_distance(&earth_orbit.periapsis(), &solar_mass);
    let escape_vel_apoapsis =
        earth_orbit.escape_velocity_at_distance(&earth_orbit.apoapsis(), &solar_mass);

    println!(
        "  Escape Velocity @ Perihel: {:.1} km/s",
        escape_vel_periapsis.in_kms()
    );
    println!(
        "  Escape Velocity @ Aphel: {:.1} km/s",
        escape_vel_apoapsis.in_kms()
    );

    // Mars Orbit als Vergleich
    let mars_orbit = OrbitalElements::new(
        Distance::au(1.524),
        0.0934,
        1.85,  // Inklination zu Erde's Orbit
        49.6,  // Longitude of ascending node
        286.5, // Argument of periapsis
        19.4,  // True anomaly
    );

    let mars_period = mars_orbit.orbital_period(&solar_mass);
    println!("\n🔴 Mars-Sonne System:");
    println!(
        "  Semi-major axis: {:.3} AU",
        mars_orbit.semimajor_axis.in_au()
    );
    println!("  Orbitalperiode: {:.1} Jahre", mars_period.in_years());
    println!("  Orbit-Typ: {:?}", mars_orbit.orbit_type());
    println!(
        "  Klassifikation: {:?}",
        OrbitalClassification::from(mars_orbit.inclination)
    );

    println!();
}

fn demonstrate_stellar_systems() {
    println!("⭐ STELLARE SYSTEME");
    println!("{}", "=".repeat(50));

    // Verschiedene Sterntypen
    let systems = vec![
        ("Sonne (G2V)", Mass::solar_masses(1.0), Time::years(4.6)),
        (
            "Proxima Centauri (M5.5V)",
            Mass::solar_masses(0.12),
            Time::years(4.85),
        ),
        (
            "Sirius A (A1V)",
            Mass::solar_masses(2.02),
            Time::years(0.25),
        ),
        (
            "Betelgeuse (M1-2 Ia-ab)",
            Mass::solar_masses(18.0),
            Time::years(0.01),
        ),
    ];

    for (name, mass, age) in systems {
        let star = StellarProperties::new(mass, age, 0.0);
        let hz = star.calculate_habitable_zone();
        let surface_escape = star.surface_escape_velocity();

        println!("⭐ {}:", name);
        println!("  Masse: {:.2} M☉", star.mass.in_solar_masses());
        println!("  Spektraltyp: {:?}", star.spectral_type);
        println!("  Temperatur: {:.0} K", star.effective_temperature);
        println!("  Leuchtkraft: {:.3} L☉", star.luminosity);
        println!(
            "  Hauptreihen-Lebensdauer: {:.1} Gyr",
            star.main_sequence_lifetime.in_years()
        );
        println!("  Evolutionsstadium: {:?}", star.evolutionary_stage);
        println!(
            "  Bewohnbare Zone: {:.2} - {:.2} AU",
            hz.inner_edge.in_au(),
            hz.outer_edge.in_au()
        );
        println!(
            "  Surface Escape Velocity: {:.0} km/s",
            surface_escape.in_kms()
        );

        // Tidal Locking Analysis für Planeten in HZ
        let hz_center = Distance::au((hz.inner_edge.in_au() + hz.outer_edge.in_au()) / 2.0);
        let tidal_analysis = star.analyze_tidal_locking(&hz_center);
        if tidal_analysis.tidal_lock_probability > 0.1 {
            println!(
                "  Tidal Lock @ HZ-Zentrum: {:.0}% Wahrscheinlichkeit",
                tidal_analysis.tidal_lock_probability * 100.0
            );
        }

        println!();
    }
}

fn demonstrate_lagrange_points() {
    println!("🔺 LAGRANGE-PUNKTE UND TROJANER");
    println!("{}", "=".repeat(50));

    // Sonne-Jupiter System (bekannt für Trojaner-Asteroiden)
    let sun = StellarProperties::sun_like();
    let jupiter = StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6), 0.0);
    let separation = Distance::au(5.2);

    let mut lagrange_system = LagrangeSystem::new(&sun, &jupiter, separation);

    println!("🌞 Sonne-Jupiter Lagrange-System:");
    println!("  Separation: {:.1} AU", lagrange_system.separation.in_au());
    println!("  Massenverhältnis: {:.0}:1", lagrange_system.mass_ratio);
    println!(
        "  L4/L5 Stabilität: {}",
        if lagrange_system.l4_l5_stable {
            "✓ Stabil"
        } else {
            "✗ Instabil"
        }
    );

    if lagrange_system.l4_l5_stable {
        println!(
            "  L1 Entfernung von Jupiter: {:.3} AU",
            lagrange_system.l1_distance_from_secondary.in_au()
        );
        println!(
            "  L2 Entfernung von Jupiter: {:.3} AU",
            lagrange_system.l2_distance_from_secondary.in_au()
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

        // Trojaner-Asteroiden simulieren
        let trojan_masses = vec![
            ("588 Achilles", Mass::kilograms(1.81e17)),
            ("617 Patroclus", Mass::kilograms(1.13e17)),
            ("624 Hektor", Mass::kilograms(7.9e18)),
        ];

        println!("\n  🪨 Trojaner-Simulation:");
        for (name, mass) in trojan_masses {
            if let Ok(trojan) = lagrange_system.generate_trojan(4, mass, &sun.mass, &jupiter.mass) {
                println!("    {} @ L4:", name);
                println!("      Masse: {:.1e} kg", trojan.mass.in_kg());
                println!(
                    "      Oszillationsamplitude: {:.2} AU",
                    trojan.oscillation_amplitude.in_au()
                );
                println!(
                    "      Oszillationsperiode: {:.1} Jahre",
                    trojan.oscillation_period.in_years()
                );
                println!("      Stabilität: {:.1}%", trojan.stability * 100.0);

                if lagrange_system.add_trojan(trojan).is_ok() {
                    println!("      ✓ Erfolgreich hinzugefügt");
                }
            }
        }
    }

    // Instabiles System zum Vergleich
    println!("\n🌟 Alpha Centauri A-B (instabiles L4/L5):");
    let alpha_cen_a = StellarProperties::new(Mass::solar_masses(1.1), Time::years(4.85), 0.22);
    let alpha_cen_b = StellarProperties::new(Mass::solar_masses(0.907), Time::years(4.85), 0.23);
    let alpha_cen_separation = Distance::au(23.4); // Durchschnittliche Separation

    let alpha_cen_lagrange = LagrangeSystem::new(&alpha_cen_a, &alpha_cen_b, alpha_cen_separation);
    println!("  Massenverhältnis: {:.1}:1", alpha_cen_lagrange.mass_ratio);
    println!(
        "  L4/L5 Stabilität: {}",
        if alpha_cen_lagrange.l4_l5_stable {
            "✓ Stabil"
        } else {
            "✗ Instabil (< 24.96:1)"
        }
    );

    println!();
}

fn demonstrate_unit_conversions() {
    println!("🔄 EINHEITEN-KONVERTIERUNG");
    println!("{}", "=".repeat(50));

    // Distanzen
    let distance_au = Distance::au(1.0);
    let distance_m = distance_au.to_si();
    let distance_km = Distance::kilometers(149597870.7);

    println!("🌍 Distanz-Konvertierung:");
    println!("  1 AU = {:.6e} m", distance_m.value);
    println!("  1 AU = {:.0} km", distance_m.in_meters() / 1000.0);
    println!(
        "  {:.1} Mio km = {:.3} AU",
        distance_km.in_meters() / 1e6,
        distance_km.in_au()
    );

    // Geschwindigkeiten
    let earth_orbital_vel = Velocity::km_per_second(29.78);
    let earth_orbital_au_year = earth_orbital_vel.to_astronomical();

    println!("\n🚀 Geschwindigkeits-Konvertierung:");
    println!(
        "  Erde's Orbitalgeschwindigkeit: {:.1} km/s",
        earth_orbital_vel.in_kms()
    );
    println!(
        "  Entspricht: {:.2} AU/Jahr",
        earth_orbital_au_year.in_au_per_year()
    );
    println!("  Theoretisch: {:.2} AU/Jahr (2π)", 2.0 * PI);

    // Massen
    let jupiter_mass_kg = Mass::kilograms(1.898e27);
    let jupiter_mass_solar = jupiter_mass_kg.to_astronomical();

    println!("\n⚖️ Masse-Konvertierung:");
    println!("  Jupiter: {:.3e} kg", jupiter_mass_kg.in_kg());
    println!("  Jupiter: {:.6} M☉", jupiter_mass_solar.in_solar_masses());
    println!(
        "  Verhältnis Sonne:Jupiter = {:.0}:1",
        1.0 / jupiter_mass_solar.in_solar_masses()
    );

    // Zeit
    let year = Time::years(1.0);
    let seconds = year.to_si();

    println!("\n⏰ Zeit-Konvertierung:");
    println!("  1 Jahr = {:.0} Sekunden", seconds.in_seconds());
    println!(
        "  1 Jahr = {:.1} Tage",
        seconds.in_seconds() / (24.0 * 3600.0)
    );

    println!();
}

fn demonstrate_escape_velocities() {
    println!("🚀 ESCAPE VELOCITIES");
    println!("{}", "=".repeat(50));

    let bodies = vec![
        (
            "Erde",
            Mass::kilograms(5.972e24),
            Distance::kilometers(6371.0),
        ),
        (
            "Mond",
            Mass::kilograms(7.342e22),
            Distance::kilometers(1737.4),
        ),
        (
            "Mars",
            Mass::kilograms(6.39e23),
            Distance::kilometers(3389.5),
        ),
        (
            "Jupiter",
            Mass::kilograms(1.898e27),
            Distance::kilometers(69911.0),
        ),
        (
            "Sonne",
            Mass::solar_masses(1.0),
            Distance::kilometers(696000.0),
        ),
    ];

    for (name, mass, radius) in bodies {
        let escape_vel = EscapeVelocity::from_surface(&mass, &radius);
        let orbital_vel =
            Velocity::meters_per_second((G * mass.in_kg() / radius.in_meters()).sqrt());

        println!("🪐 {}:", name);
        println!("  Radius: {:.0} km", radius.in_meters() / 1000.0);
        println!("  Masse: {:.2e} kg", mass.in_kg());
        println!("  Escape Velocity: {:.1} km/s", escape_vel.in_kms());
        println!("  Orbital Velocity: {:.1} km/s", orbital_vel.in_kms());
        println!(
            "  Verhältnis v_e/v_o: {:.2}",
            escape_vel.in_ms() / orbital_vel.in_ms()
        );
        println!();
    }

    // Vergleich mit theoretischem √2 Verhältnis
    println!(
        "📐 Theoretisches Verhältnis v_escape/v_orbital = √2 = {:.3}",
        2.0_f64.sqrt()
    );
}

// Zufällige System-Generierung (vereinfacht aus original main.rs)
fn generate_random_systems() {
    println!("🎲 ZUFÄLLIGE SYSTEM-GENERIERUNG");
    println!("{}", "=".repeat(50));

    let mut rng = ChaCha8Rng::seed_from_u64(42);

    for i in 1..=3 {
        println!("System {}:", i);

        // Zufällige Sternmasse (IMF-gewichtet)
        let mass_solar = match rng.r#gen::<f64>() {
            x if x < 0.6 => rng.gen_range(0.1..0.5),  // M-Zwerge
            x if x < 0.85 => rng.gen_range(0.5..1.0), // K-Zwerge
            x if x < 0.95 => rng.gen_range(1.0..2.0), // G-F Sterne
            _ => rng.gen_range(2.0..10.0),            // A-B Sterne
        };

        let age_gyr = rng.gen_range(1.0..12.0);
        let metallicity = rng.gen_range(-0.5..0.5);

        let star = StellarProperties::new(
            Mass::solar_masses(mass_solar),
            Time::years(age_gyr),
            metallicity,
        );

        let hz = star.calculate_habitable_zone();

        println!(
            "  Stern: {:?} {:.2} M☉",
            star.spectral_type,
            star.mass.in_solar_masses()
        );
        println!(
            "  Alter: {:.1} Gyr, Metallizität: {:.2}",
            age_gyr, metallicity
        );
        println!(
            "  Bewohnbare Zone: {:.2} - {:.2} AU",
            hz.inner_edge.in_au(),
            hz.outer_edge.in_au()
        );

        // Zufällige Planeten in der bewohnbaren Zone
        if rng.r#gen::<f64>() < 0.6 {
            // 60% Chance für Planet in HZ
            let planet_distance =
                Distance::au(rng.gen_range(hz.inner_edge.in_au()..hz.outer_edge.in_au()));
            let tidal_analysis = star.analyze_tidal_locking(&planet_distance);

            println!("  🪐 Planet @ {:.2} AU:", planet_distance.in_au());
            if tidal_analysis.tidal_lock_probability > 0.5 {
                println!(
                    "    ⚠️ Wahrscheinlich tidal locked ({:.0}%)",
                    tidal_analysis.tidal_lock_probability * 100.0
                );
            } else {
                println!("    ✓ Freie Rotation möglich");
            }
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrated_system() {
        // Test der Integration aller Module
        let sun = StellarProperties::sun_like();
        let earth_orbit = OrbitalElements::new(Distance::au(1.0), 0.0167, 0.0, 0.0, 102.9, 0.0);

        let period = earth_orbit.orbital_period(&sun.mass);
        assert!((period.in_years() - 1.0).abs() < 0.01);

        let hz = sun.calculate_habitable_zone();
        assert!(hz.inner_edge.in_au() < 1.0);
        assert!(hz.outer_edge.in_au() > 1.0);
    }

    #[test]
    fn test_lagrange_integration() {
        let sun = StellarProperties::sun_like();
        let jupiter = StellarProperties::new(Mass::solar_masses(0.000954), Time::years(4.6), 0.0);
        let lagrange_system = LagrangeSystem::new(&sun, &jupiter, Distance::au(5.2));

        assert!(lagrange_system.l4_l5_stable);
        assert!(lagrange_system.mass_ratio > MIN_LAGRANGE_MASS_RATIO);
    }
}
