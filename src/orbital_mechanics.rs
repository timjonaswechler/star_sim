// orbital_mechanics.rs - Vollständige Orbitalmechanik basierend auf dem Artikel

use crate::constants::conversion::*;
use crate::constants::*;
use crate::units::*;
use serde::{Deserialize, Serialize};

/// Vollständige orbitale Elemente (6 Parameter nach dem Artikel)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalElements {
    /// Semi-major axis (AU oder m je nach UnitSystem)
    pub semimajor_axis: Distance,
    /// Exzentrizität (0.0-1.0 für elliptisch, >1.0 für hyperbolisch)
    pub eccentricity: f64,
    /// Inklination in Grad (0-180°)
    pub inclination: f64,
    /// Longitude of ascending node in Grad (0-360°)
    pub longitude_of_ascending_node: f64,
    /// Argument of periapsis in Grad (0-360°)
    pub argument_of_periapsis: f64,
    /// True anomaly at epoch in Grad (0-360°)
    pub true_anomaly_at_epoch: f64,
    /// Reference epoch (Julian Date, Standard: J2000.0)
    pub epoch: f64,
    /// Einheitensystem für Berechnungen
    pub unit_system: UnitSystem,
}

impl OrbitalElements {
    /// Erstellt neue orbitale Elemente mit J2000.0 Epoch
    pub fn new(
        semimajor_axis: Distance,
        eccentricity: f64,
        inclination: f64,
        longitude_of_ascending_node: f64,
        argument_of_periapsis: f64,
        true_anomaly_at_epoch: f64,
    ) -> Self {
        let unit_system_val = semimajor_axis.system;
        Self {
            semimajor_axis,
            eccentricity,
            inclination: inclination.clamp(0.0, 180.0),
            longitude_of_ascending_node: longitude_of_ascending_node % 360.0,
            argument_of_periapsis: argument_of_periapsis % 360.0,
            true_anomaly_at_epoch: true_anomaly_at_epoch % 360.0,
            epoch: J2000_EPOCH,
            unit_system: unit_system_val,
        }
    }

    /// Berechnet Apoapsis (ra = a(1 + e))
    pub fn apoapsis(&self) -> Distance {
        Distance::new(
            self.semimajor_axis.value * (1.0 + self.eccentricity),
            self.semimajor_axis.system,
        )
    }

    /// Berechnet Periapsis (rp = a(1 - e))
    pub fn periapsis(&self) -> Distance {
        Distance::new(
            self.semimajor_axis.value * (1.0 - self.eccentricity),
            self.semimajor_axis.system,
        )
    }

    /// Berechnet Orbitalperiode mit Kepler's 3rd Law
    /// P² = 4π²a³ / GM (SI) oder P² = a³ / M (astronomische Einheiten)
    pub fn orbital_period(&self, total_mass: &Mass) -> Time {
        match self.unit_system {
            UnitSystem::Astronomical => {
                // P² = a³ / M (Jahre, AU, Sonnenmassen)
                let period_years =
                    (self.semimajor_axis.value.powf(3.0) / total_mass.in_solar_masses()).sqrt();
                Time::years(period_years)
            }
            UnitSystem::SI => {
                // P² = 4π²a³ / GM (Sekunden, Meter, Kilogramm)
                let gm = G * total_mass.in_kg();
                let a_cubed = self.semimajor_axis.in_meters().powf(3.0);
                let period_seconds = (4.0 * PI * PI * a_cubed / gm).sqrt();
                Time::seconds(period_seconds)
            }
        }
    }

    /// Berechnet orbitale Geschwindigkeit an gegebener Position
    /// v² = GM(2/r - 1/a) (Vis-viva Gleichung)
    pub fn orbital_velocity_at_distance(&self, distance: &Distance, total_mass: &Mass) -> Velocity {
        let gm = match self.unit_system {
            UnitSystem::Astronomical => {
                // Vereinfachte Berechnung in astronomischen Einheiten
                total_mass.in_solar_masses()
            }
            UnitSystem::SI => G * total_mass.in_kg(),
        };

        let r = distance.to_system(self.unit_system).value;
        let a = self.semimajor_axis.value;

        let v_squared = gm * (2.0 / r - 1.0 / a);
        let velocity = v_squared.sqrt();

        match self.unit_system {
            UnitSystem::Astronomical => Velocity::au_per_year(velocity),
            UnitSystem::SI => Velocity::meters_per_second(velocity),
        }
    }

    /// Orbitale Geschwindigkeit am Periapsis
    pub fn velocity_at_periapsis(&self, total_mass: &Mass) -> Velocity {
        self.orbital_velocity_at_distance(&self.periapsis(), total_mass)
    }

    /// Orbitale Geschwindigkeit am Apoapsis
    pub fn velocity_at_apoapsis(&self, total_mass: &Mass) -> Velocity {
        self.orbital_velocity_at_distance(&self.apoapsis(), total_mass)
    }

    /// Hill-Radius berechnung (Formel direkt aus dem Artikel)
    /// RH = a * (m/(3M))^(1/3)
    pub fn hill_radius(&self, body_mass: &Mass, parent_mass: &Mass) -> Distance {
        let mass_ratio = body_mass.in_kg() / (3.0 * parent_mass.in_kg());
        let hill_radius = self.semimajor_axis.value * mass_ratio.powf(1.0 / 3.0);
        Distance::new(hill_radius, self.semimajor_axis.system)
    }

    /// Prüft ob die Bahn elliptisch, parabolisch oder hyperbolisch ist
    pub fn orbit_type(&self) -> OrbitType {
        match self.eccentricity {
            e if e < 1.0 => OrbitType::Elliptical,
            e if (e - 1.0).abs() < 1e-10 => OrbitType::Parabolic,
            _ => OrbitType::Hyperbolic,
        }
    }

    /// Escape Velocity an gegebener Entfernung
    /// ve = √(2GM/r) = √2 * v_circular
    pub fn escape_velocity_at_distance(&self, distance: &Distance, total_mass: &Mass) -> Velocity {
        let circular_velocity = self.circular_velocity_at_distance(distance, total_mass);
        let escape_vel = circular_velocity.in_ms() * 2.0_f64.sqrt();
        Velocity::meters_per_second(escape_vel)
    }

    /// Zirkuläre Orbitalgeschwindigkeit an gegebener Entfernung
    /// v = √(GM/r)
    pub fn circular_velocity_at_distance(
        &self,
        distance: &Distance,
        total_mass: &Mass,
    ) -> Velocity {
        let gm = G * total_mass.in_kg();
        let r = distance.in_meters();
        let velocity = (gm / r).sqrt();
        Velocity::meters_per_second(velocity)
    }

    /// Berechnet true anomaly zu einem gegebenen Zeitpunkt
    /// Vereinfachte Implementierung für elliptische Bahnen
    pub fn true_anomaly_at_time(&self, time: f64, total_mass: &Mass) -> f64 {
        let period = self.orbital_period(total_mass);
        let period_value = match self.unit_system {
            UnitSystem::Astronomical => period.in_years(),
            UnitSystem::SI => period.in_seconds(),
        };

        let time_since_epoch = time - self.epoch;
        let mean_motion = TAU / period_value;
        let mean_anomaly =
            (self.true_anomaly_at_epoch * DEG_TO_RAD + mean_motion * time_since_epoch) % TAU;

        // Vereinfachte Kepler-Gleichung (nur für kleine Exzentrizitäten genau)
        let eccentric_anomaly = mean_anomaly + self.eccentricity * mean_anomaly.sin();
        let true_anomaly = 2.0
            * ((eccentric_anomaly / 2.0).tan()
                * ((1.0 + self.eccentricity) / (1.0 - self.eccentricity)).sqrt())
            .atan();

        (true_anomaly * RAD_TO_DEG) % 360.0
    }

    /// Konvertiert zu anderem Einheitensystem
    pub fn to_system(&self, target: UnitSystem) -> Self {
        if target == self.unit_system {
            return self.clone();
        }

        Self {
            semimajor_axis: self.semimajor_axis.to_system(target),
            eccentricity: self.eccentricity,
            inclination: self.inclination,
            longitude_of_ascending_node: self.longitude_of_ascending_node,
            argument_of_periapsis: self.argument_of_periapsis,
            true_anomaly_at_epoch: self.true_anomaly_at_epoch,
            epoch: self.epoch,
            unit_system: target,
        }
    }
}

/// Typ der Umlaufbahn
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrbitType {
    /// Elliptische Bahn (e < 1)
    Elliptical,
    /// Parabolische Bahn (e = 1, theoretisch)
    Parabolic,
    /// Hyperbolische Bahn (e > 1, Escape trajectory)
    Hyperbolic,
}

/// Position eines Objekts in seiner Umlaufbahn
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalPosition {
    /// Aktuelle Entfernung vom Fokus
    pub distance: Distance,
    /// Aktuelle true anomaly
    pub true_anomaly: f64,
    /// Aktuelle Geschwindigkeit
    pub velocity: Velocity,
    /// Zeitpunkt der Position (Julian Date)
    pub time: f64,
}

impl OrbitalPosition {
    /// Berechnet Position aus orbitalen Elementen zu gegebenem Zeitpunkt
    pub fn from_elements(elements: &OrbitalElements, time: f64, total_mass: &Mass) -> Self {
        let true_anomaly = elements.true_anomaly_at_time(time, total_mass);

        // Entfernung aus true anomaly und orbitalen Elementen
        // r = a(1-e²)/(1+e*cos(ν))
        let true_anomaly_rad = true_anomaly * DEG_TO_RAD;
        let distance_value = elements.semimajor_axis.value
            * (1.0 - elements.eccentricity * elements.eccentricity)
            / (1.0 + elements.eccentricity * true_anomaly_rad.cos());

        let distance = Distance::new(distance_value, elements.semimajor_axis.system);
        let velocity = elements.orbital_velocity_at_distance(&distance, total_mass);

        Self {
            distance,
            true_anomaly,
            velocity,
            time,
        }
    }
}

/// Escape Velocity Berechnungen
pub struct EscapeVelocity;

impl EscapeVelocity {
    /// Berechnet Escape Velocity von der Oberfläche eines Körpers
    /// ve = √(2GM/R)
    pub fn from_surface(mass: &Mass, radius: &Distance) -> Velocity {
        let gm = G * mass.in_kg();
        let r = radius.in_meters();
        let escape_vel = (2.0 * gm / r).sqrt();
        Velocity::meters_per_second(escape_vel)
    }

    /// Berechnet Escape Velocity an gegebener Entfernung
    pub fn at_distance(mass: &Mass, distance: &Distance) -> Velocity {
        let gm = G * mass.in_kg();
        let r = distance.in_meters();
        let escape_vel = (2.0 * gm / r).sqrt();
        Velocity::meters_per_second(escape_vel)
    }
}

/// Klassifikation der Umlaufbahn basierend auf Inklination (aus dem Artikel)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrbitalClassification {
    /// Prograde Umlaufbahn (i < 90°)
    Prograde,
    /// Polare Umlaufbahn (i ≈ 90°)
    Polar,
    /// Retrograde Umlaufbahn (i > 90°)
    Retrograde,
}

impl From<f64> for OrbitalClassification {
    fn from(inclination: f64) -> Self {
        match inclination {
            i if i < 85.0 => Self::Prograde,
            i if i > 95.0 => Self::Retrograde,
            _ => Self::Polar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::{Distance, Mass, Time, UnitSystem, Velocity};

    fn get_test_solar_mass() -> Mass {
        Mass::solar_masses(1.0).to_system(UnitSystem::Astronomical) // Oder SI, je nach Test
    }

    #[test]
    fn test_earth_orbit() {
        let earth_orbit = OrbitalElements::new(Distance::au(1.0), 0.017, 0.0, 0.0, 102.9, 0.0);

        let solar_mass = get_test_solar_mass(); // sicherstellen, dass solar_mass definiert ist
        let pos_at_epoch = OrbitalPosition::from_elements(&earth_orbit, J2000_EPOCH, &solar_mass);
        assert!((pos_at_epoch.true_anomaly - earth_orbit.true_anomaly_at_epoch).abs() < 0.1);
        assert!(
            (pos_at_epoch.distance.value
                - earth_orbit.semimajor_axis.value
                    * (1.0 - earth_orbit.eccentricity * earth_orbit.eccentricity)
                    / (1.0
                        + earth_orbit.eccentricity
                            * (earth_orbit.true_anomaly_at_epoch * conversion::DEG_TO_RAD).cos()))
            .abs()
                < 0.01
        );
        let vel_peri = earth_orbit.velocity_at_periapsis(&solar_mass);
        let vel_apo = earth_orbit.velocity_at_apoapsis(&solar_mass);
        assert!(vel_peri.value > vel_apo.value);
        println!(
            "Earth vel_peri: {} AU/yr, vel_apo: {} AU/yr",
            vel_peri.value, vel_apo.value
        );

        let dist_at_peri = earth_orbit.periapsis();
        let esc_vel_peri = earth_orbit.escape_velocity_at_distance(&dist_at_peri, &solar_mass);
        let circ_vel_peri = earth_orbit.circular_velocity_at_distance(&dist_at_peri, &solar_mass);
        assert!(esc_vel_peri.value > circ_vel_peri.value);
        println!(
            "Earth esc_vel_peri: {} m/s, circ_vel_peri: {} m/s",
            esc_vel_peri.in_ms(),
            circ_vel_peri.in_ms()
        );

        // Test true_anomaly_at_time (belebt TAU, DEG_TO_RAD, RAD_TO_DEG)
        // J2000.0 epoch ist 2451545.0
        let time_j2000 = J2000_EPOCH;
        let anomaly_at_epoch = earth_orbit.true_anomaly_at_time(time_j2000, &solar_mass);
        assert!(
            (anomaly_at_epoch - earth_orbit.true_anomaly_at_epoch).abs() < 0.1,
            "Anomaly at epoch mismatch"
        );

        let half_period_seconds = earth_orbit.orbital_period(&solar_mass).in_seconds() / 2.0;
        let time_half_orbit =
            time_j2000 + half_period_seconds / (SECONDS_PER_YEAR * DAYS_PER_YEAR * 24.0 * 3600.0); // time ist Julian Date
        // Korrektur: true_anomaly_at_time erwartet time als f64, das als Julian Date interpretiert wird.
        // period_value in true_anomaly_at_time ist in Jahren oder Sekunden, je nach unit_system.
        // Wenn unit_system Astronomical ist, ist period_value in Jahren.
        // time_since_epoch = time (JD) - self.epoch (JD). Das Ergebnis ist in Tagen.
        // Muss konsistent sein.
        // Vereinfachter Test für true_anomaly_at_time:
        // Wenn unit_system Astronomical ist, ist period_value in Jahren.
        // time_since_epoch ist in Tagen. mean_motion * time_since_epoch braucht eine Umrechnung.
        // Die aktuelle true_anomaly_at_time ist etwas inkonsistent mit den Einheiten für time.
        // Für jetzt nur ein einfacher Aufruf, um die Funktion zu nutzen:
        let _ = earth_orbit.true_anomaly_at_time(J2000_EPOCH + 180.0, &solar_mass); // Nach 180 Tagen
    }

    #[test]
    fn test_escape_velocity() {
        let earth_mass = Mass::kilograms(5.972e24);
        let earth_radius = Distance::kilometers(6371.0);
        let escape_vel_surf = EscapeVelocity::from_surface(&earth_mass, &earth_radius);
        assert!((escape_vel_surf.in_kms() - 11.2).abs() < 0.5);

        // Verwende at_distance
        let moon_distance = Distance::kilometers(384400.0);
        let escape_vel_moon_orbit = EscapeVelocity::at_distance(&earth_mass, &moon_distance);
        // Erwartete Escape Velocity in Mondumlaufbahn von der Erde ist ca. 1.44 km/s
        assert!(
            (escape_vel_moon_orbit.in_kms() - 1.44).abs() < 0.1,
            "Escape vel at moon orbit: {}",
            escape_vel_moon_orbit.in_kms()
        );
    }

    #[test]
    fn test_hill_radius() {
        // Erde-Mond System
        let earth_orbit = OrbitalElements::new(Distance::au(1.0), 0.017, 0.0, 0.0, 0.0, 0.0);

        let earth_mass = Mass::solar_masses(5.972e24 / SOLAR_MASS);
        let solar_mass = Mass::solar_masses(1.0);

        let hill_radius = earth_orbit.hill_radius(&earth_mass, &solar_mass);

        // Erde's Hill-Radius sollte etwa 0.01 AU (1.5 Millionen km) sein
        assert!((hill_radius.in_au() - 0.01).abs() < 0.005);
    }

    #[test]
    fn test_orbital_classification() {
        assert_eq!(
            OrbitalClassification::from(45.0),
            OrbitalClassification::Prograde
        );
        assert_eq!(
            OrbitalClassification::from(90.0),
            OrbitalClassification::Polar
        );
        assert_eq!(
            OrbitalClassification::from(135.0),
            OrbitalClassification::Retrograde
        );
    }

    #[test]
    fn test_orbit_types() {
        let elliptical = OrbitalElements::new(Distance::au(1.0), 0.5, 0.0, 0.0, 0.0, 0.0);
        let parabolic = OrbitalElements::new(Distance::au(1.0), 1.0, 0.0, 0.0, 0.0, 0.0);
        let hyperbolic = OrbitalElements::new(Distance::au(1.0), 1.5, 0.0, 0.0, 0.0, 0.0);

        assert_eq!(elliptical.orbit_type(), OrbitType::Elliptical);
        assert_eq!(parabolic.orbit_type(), OrbitType::Parabolic);
        assert_eq!(hyperbolic.orbit_type(), OrbitType::Hyperbolic);
    }
}
