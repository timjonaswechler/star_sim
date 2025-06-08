use serde::{Deserialize, Serialize};

/// Vollständige physikalische Eigenschaften eines Planeten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalProperties {
    /// Masse des Planeten
    pub mass: Mass,
    /// Radius des Planeten
    pub radius: Distance,
    /// Mittlere Dichte
    pub density: f64, // g/cm³
    /// Oberflächenschwerkraft
    pub surface_gravity: f64, // Earth g
    /// Entweichungsgeschwindigkeit
    pub escape_velocity: Velocity,
    /// Rotationsperiode
    pub rotation_period: Time,
    /// Achsenneigung (Obliquität)
    pub obliquity: f64, // Grad
    /// Exzentrizität der Rotation (Abplattung)
    pub oblateness: f64,
    /// Entfernung zum Horizont für 2m Beobachter
    pub horizon_distance: Distance,
    /// Komposition
    pub composition: PlanetComposition,
    /// Einheitensystem
    pub unit_system: UnitSystem,
}

impl PhysicalProperties {
    /// Erstellt planetare Eigenschaften aus Masse und Komposition
    pub fn from_mass_and_composition(
        mass: Mass,
        composition: PlanetComposition,
        age: Time,
        rng: &mut ChaCha8Rng,
    ) -> Self {
        let unit_system = mass.system;

        // Radius aus Masse-Radius-Beziehung
        let radius = Self::calculate_radius(&mass, &composition, &age);

        // Dichte berechnen
        let volume_earth = (4.0 / 3.0) * PI * radius.in_earth_radii().powi(3);
        let density = mass.in_earth_masses() / volume_earth * 5.514; // Earth density in g/cm³

        // Oberflächenschwerkraft
        let surface_gravity = mass.in_earth_masses() / radius.in_earth_radii().powi(2);

        // Entweichungsgeschwindigkeit: v = sqrt(2GM/R)
        let escape_velocity =
            Velocity::meters_per_second((2.0 * G * mass.in_kg() / radius.in_meters()).sqrt());

        // Rotationsperiode (mit Variationen)
        let rotation_period = Self::generate_rotation_period(&mass, rng);

        // Obliquität (Achsenneigung)
        let obliquity = Self::generate_obliquity(rng);

        // Abplattung durch Rotation
        let oblateness = Self::calculate_oblateness(&mass, &radius, &rotation_period);

        // Horizontentfernung für 2m Beobachter
        let horizon_distance = Self::calculate_horizon_distance(&radius, 2.0);

        Self {
            mass,
            radius,
            density,
            surface_gravity,
            escape_velocity,
            rotation_period,
            obliquity,
            oblateness,
            horizon_distance,
            composition,
            unit_system,
        }
    }

    /// Masse-Radius-Beziehung aus dem Artikel
    pub fn calculate_radius(mass: &Mass, composition: &PlanetComposition, age: &Time) -> Distance {
        let m_earth = mass.in_earth_masses();

        match composition {
            PlanetComposition::Terrestrial { rock_fraction } => {
                Self::terrestrial_radius(m_earth, *rock_fraction)
            }
            PlanetComposition::Waterworld {
                water_fraction,
                rock_fraction,
            } => Self::waterworld_radius(m_earth, *water_fraction, *rock_fraction),
            PlanetComposition::GasGiant {
                bulk_metallicity, ..
            } => Self::gas_giant_radius(m_earth, *bulk_metallicity, age.in_years()),
            PlanetComposition::IceGiant { volatiles_fraction } => {
                Self::ice_giant_radius(m_earth, *volatiles_fraction)
            }
            PlanetComposition::CarbonPlanet { .. } => {
                // Ähnlich terrestrisch, aber andere Dichte
                Self::carbon_planet_radius(m_earth)
            }
        }
    }

    /// Terrestrische Planeten (Artikel Formel)
    fn terrestrial_radius(mass_earth: f64, rock_fraction: f64) -> Distance {
        if mass_earth < 0.01 {
            // Sehr kleine Körper: uncompressed density
            Distance::earth_radii((mass_earth / 4.0).powf(1.0 / 3.0))
        } else if mass_earth <= 100.0 {
            // Artikel Formel für Rock-Metal Planeten
            let r_earth = (0.0592 * rock_fraction + 0.0975) * mass_earth.ln()
                + (0.2337 * rock_fraction + 0.4938);
            Distance::earth_radii(r_earth.max(0.1))
        } else {
            // Sehr massive terrestrische Planeten werden komprimiert
            Distance::earth_radii(2.5 * mass_earth.powf(0.1))
        }
    }

    /// Wasserwelten (vereinfachte Formel aus Artikel)
    fn waterworld_radius(mass_earth: f64, water_fraction: f64, rock_fraction: f64) -> Distance {
        if water_fraction < 0.01 {
            // Praktisch kein Wasser
            return Self::terrestrial_radius(mass_earth, rock_fraction);
        }

        // Artikel Formel für Water-Rock Planeten
        let metal_fraction = 1.0 - water_fraction - rock_fraction;
        let r_rock_metal =
            Self::terrestrial_radius(mass_earth * (1.0 - water_fraction), rock_fraction);

        // Wasseranteil vergrößert Radius
        let water_factor = 1.0 + water_fraction * 0.5; // Vereinfacht
        Distance::earth_radii(r_rock_metal.in_earth_radii() * water_factor)
    }

    /// Gasriesen (stark vereinfacht)
    fn gas_giant_radius(mass_earth: f64, metallicity: f64, age_years: f64) -> Distance {
        let mass_jupiter = mass_earth / 317.8; // Earth to Jupiter masses

        if mass_jupiter < 0.3 {
            // Sub-Neptune
            Distance::earth_radii(2.0 + mass_jupiter * 1.5)
        } else if mass_jupiter < 13.0 {
            // Jupiter-like: Radius sinkt mit Masse durch Kompression
            let base_radius = 11.2 - mass_jupiter * 0.5; // Earth radii
            let age_factor = 1.0 - (age_years / 1e10).min(0.3); // Schrumpfung mit Alter
            Distance::earth_radii((base_radius * age_factor).max(8.0))
        } else {
            // Braune Zwerge
            Distance::earth_radii(8.0)
        }
    }

    /// Eisriesen
    fn ice_giant_radius(mass_earth: f64, volatiles_fraction: f64) -> Distance {
        let base = if mass_earth < 100.0 {
            3.0 + mass_earth.ln() * 0.5
        } else {
            6.0
        };
        Distance::earth_radii(base * (1.0 + volatiles_fraction * 0.3))
    }

    /// Kohlenstoffplaneten
    fn carbon_planet_radius(mass_earth: f64) -> Distance {
        // Carbide/Diamond sind dichter als Silicate
        let terrestrial = Self::terrestrial_radius(mass_earth, 0.85);
        Distance::earth_radii(terrestrial.in_earth_radii() * 0.9)
    }

    /// Rotationsperiode generieren
    fn generate_rotation_period(mass: &Mass, rng: &mut ChaCha8Rng) -> Time {
        let mass_earth = mass.in_earth_masses();

        // Tendenz: Größere Planeten rotieren schneller
        let base_hours = if mass_earth < 0.1 {
            rng.gen_range(100.0..2000.0) // Sehr langsam für kleine Körper
        } else if mass_earth < 2.0 {
            rng.gen_range(10.0..50.0) // Erdähnlich
        } else if mass_earth < 100.0 {
            rng.gen_range(8.0..20.0) // Schnelle Rotation
        } else {
            rng.gen_range(6.0..12.0) // Gasriesen
        };

        Time::hours(base_hours)
    }

    /// Obliquität generieren
    fn generate_obliquity(rng: &mut ChaCha8Rng) -> f64 {
        // Die meisten Planeten haben niedrige Obliquität
        let roll: f64 = rng.r#gen();
        if roll < 0.7 {
            rng.gen_range(0.0..30.0) // Niedrige Neigung
        } else if roll < 0.9 {
            rng.gen_range(30.0..60.0) // Mittlere Neigung
        } else {
            rng.gen_range(60.0..180.0) // Hohe Neigung (Uranus-like)
        }
    }

    /// Abplattung durch Rotation
    fn calculate_oblateness(mass: &Mass, radius: &Distance, rotation_period: &Time) -> f64 {
        let period_hours = rotation_period.in_hours();
        let mass_earth = mass.in_earth_masses();
        let radius_earth = radius.in_earth_radii();

        if period_hours > 100.0 {
            return 0.0; // Vernachlässigbar langsame Rotation
        }

        // Vereinfachte Formel aus Artikel
        let omega = 2.0 * PI / (period_hours * 3600.0); // rad/s
        let centrifugal_force = omega * omega * radius.in_meters();
        let gravity = G * mass.in_kg() / radius.in_meters().powi(2);

        // Elliptizität ≈ (Zentrifugalkraft / Schwerkraft) * Korrekturfaktor
        let ellipticity = (centrifugal_force / gravity) * 0.5;
        ellipticity.min(0.5) // Maximum für Stabilität
    }

    /// Horizontentfernung berechnen
    fn calculate_horizon_distance(radius: &Distance, observer_height: f64) -> Distance {
        // d = sqrt(2 * R * h + h²) ≈ sqrt(2 * R * h) for h << R
        let r_meters = radius.in_meters();
        let distance = (2.0 * r_meters * observer_height).sqrt();
        Distance::meters(distance)
    }

    /// Überprüft ob Planet hydrostatisches Gleichgewicht erreicht (kugelförmig)
    pub fn is_spherical(&self) -> bool {
        self.mass.in_earth_masses() > 0.00005 // 250 km Radius für Eis/Gestein
    }

    /// Minimale Masse für Atmosphärenretention (N2)
    pub fn can_retain_nitrogen_atmosphere(&self, temperature: f64) -> bool {
        // Aus Artikel: v_thermal = 1/6 * v_escape für langfristige Retention
        let n2_molar_mass = 28.0; // g/mol
        let thermal_velocity = Self::thermal_velocity(temperature, n2_molar_mass);
        thermal_velocity < self.escape_velocity.in_ms() / 6.0
    }

    /// Thermische Geschwindigkeit für Gas
    fn thermal_velocity(temperature: f64, molar_mass: f64) -> f64 {
        // v = sqrt(3RT/M)
        let r_gas = 8314.4598; // J/(mol·K)
        (3.0 * r_gas * temperature / molar_mass).sqrt()
    }

    /// Konvertiert zu anderem Einheitensystem
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if target == self.unit_system {
            return self.clone();
        }

        Self {
            mass: self.mass.to_system(target),
            radius: self.radius.to_system(target),
            escape_velocity: self.escape_velocity.to_system(target),
            rotation_period: self.rotation_period.to_system(target),
            horizon_distance: self.horizon_distance.to_system(target),
            unit_system: target,
            ..self.clone()
        }
    }
}
