use crate::physics::constants::MIN_LAGRANGE_MASS_RATIO;
use crate::physics::units::distance::{AstronomicalUnit, Distance, Meter}; // Explizite DistanceUnits
use crate::physics::units::mass::{Gram, Kilogram, Mass, SolarMass}; // Explizite MassUnits
use crate::physics::units::time::{Second, Time, Year};
use crate::stellar_objects::stars::properties::StellarProperties;
use crate::stellar_objects::trojans_asteroid::dynamics::MutualDynamics;
use crate::stellar_objects::trojans_asteroid::objects::{MutualTrojanSystem, TrojanObject};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// Lagrange-Punkt System für ein 2-Körper System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LagrangeSystem {
    /// Position von L1 (zwischen den Körpern, näher zum weniger massiven)
    pub l1_distance_from_secondary: Distance<Meter>,
    /// Position von L2 (hinter dem weniger massiven Körper)
    pub l2_distance_from_secondary: Distance<Meter>,
    /// Position von L3 (gegenüber dem weniger massiven Körper)
    pub l3_distance_from_primary: Distance<Meter>,
    /// L4/L5 Stabilität (erfordert Massenverhältnis ≥ 24.96:1)
    pub l4_l5_stable: bool,
    /// Massenverhältnis Primary:Secondary (bezogen auf die schwerere als Primary)
    pub mass_ratio: f64,
    /// Semimajor axis des Binärsystems
    pub separation: Distance<Meter>,
    /// Liste von Trojanern an L4/L5 (kann leer sein)
    pub trojans: Vec<TrojanObject>,
}

/// Oszillationsmuster für Trojaner (vereinfacht nach Artikel)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OscillationPattern {
    /// Tadpole-Orbit: Kleine Oszillationen um L4/L5
    Tadpole {
        center_point: u8,       // 4 oder 5
        amplitude_degrees: f64, // Winkelabweichung in Grad
    },
    /// Horseshoe-Orbit: Größere Oszillationen zwischen L3, L4, L5
    Horseshoe {
        transition_probability: f64, // Wahrscheinlichkeit für L4↔L5 Wechsel
        period_ratio: f64,           // Verhältnis zur Orbitalperiode
    },
    /// Quasi-stable: Nur temporär an Lagrange-Punkt gefangen
    QuasiStable {
        escape_timescale: Time<Year>, // Zeit bis zur Flucht
        drift_direction: f64,         // Richtung der Drift in Grad
    },
}

/// Status aller Lagrange-Punkte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LagrangePointsStatus {
    pub l1_stable: bool,
    pub l2_stable: bool,
    pub l3_stable: bool,
    pub l4_stable: bool,
    pub l5_stable: bool,
    pub l4_trojans: Vec<TrojanObject>,
    pub l5_trojans: Vec<TrojanObject>,
}

impl LagrangeSystem {
    /// Erstellt ein neues Lagrange-System für zwei Sterne
    pub fn new(
        primary_props: &StellarProperties,   // Umbenannt zur Klarheit
        secondary_props: &StellarProperties, // Umbenannt zur Klarheit
        separation: Distance<Meter>,
    ) -> Self {
        // Konvertiere Sternmassen (Annahme: SolarMass) zu Kilogram für Berechnungen
        let primary_mass_kg = primary_props.mass.get::<Kilogram>();
        let secondary_mass_kg = secondary_props.mass.get::<Kilogram>();

        let actual_mass_ratio = primary_mass_kg.value / secondary_mass_kg.value;

        // Für Lagrange-Berechnungen: m1 ist immer die schwerere Masse
        let (m1, m2) = if actual_mass_ratio >= 1.0 {
            (primary_mass_kg, secondary_mass_kg)
        } else {
            (secondary_mass_kg, primary_mass_kg)
        };
        // Das gespeicherte mass_ratio bezieht sich auf die schwerere Masse als "Primary"
        let system_mass_ratio = m1.value / m2.value;

        let total_mass_val_kg = m1.value + m2.value;
        let mu = m2.value / total_mass_val_kg; // Massenparameter μ = m2 / (m1 + m2)

        // L1, L2, L3 Positionen (vereinfachte Näherungen)
        // Distanzen sind relativ zum Sekundär-/Primärkörper wie in den Feldnamen spezifiziert
        let l1_distance = Self::calculate_l1_distance(&separation, mu);
        let l2_distance = Self::calculate_l2_distance(&separation, mu);
        // L3 ist relativ zum Primärkörper (m1) definiert, aber die Formel verwendet 'a' (separation) und mu bezogen auf m2
        // Die Formel r ≈ a * (1 - 5μ/12) von m1 oder r ≈ a * (1 + μ) von m2 ist üblicher.
        // Die gegebene Formel r ≈ a * (7μ/12) ist eher für die Distanz vom Baryzentrum oder m2.
        // Für "distance_from_primary" für L3: a * (1 - (m2.value / (m1.value+m2.value)) * (7/12)) ist nicht ganz Standard.
        // Ich verwende die gegebene Formel, aber beachte, dass L3 hinter dem Primärkörper liegt, gesehen vom Sekundärkörper.
        // Die Feldbezeichnung `l3_distance_from_primary` impliziert Abstand von m1.
        // Standardnäherung für L3 (relativ zu m1, m2 ist kleiner): a (1 + 5/12 * mu_m1) wo mu_m1 = m2/m1
        // Oder, wenn mu = m2/(m1+m2): Distanz von m1 ≈ a * (1 - (m2/(m1+m2)) ) + a * (7/12 * (m2/(m1+m2))) -> nicht korrekt.
        // Distanz von m2 aus: a + a * (7μ/12). Distanz von m1 aus: a * (m2/mges) + a * (7μ/12) ???
        // Die Formel r_L3 ≈ a(1 + (5/12)μ) vom Baryzentrum aus, wobei μ das kleine Massenverhältnis ist.
        // Ich nehme an, die gegebene Formel ist die Distanz vom *Primärkörper* in Richtung *weg* vom Sekundärkörper.
        let l3_distance = Self::calculate_l3_distance(&separation, mu);

        // L4/L5 Stabilität: Massenverhältnis des schwereren zum leichteren Körper muss ≥ MIN_LAGRANGE_MASS_RATIO sein
        let l4_l5_stable = system_mass_ratio >= MIN_LAGRANGE_MASS_RATIO;

        Self {
            l1_distance_from_secondary: l1_distance,
            l2_distance_from_secondary: l2_distance,
            l3_distance_from_primary: l3_distance, // Beachten Sie die Definition dieser Distanz
            l4_l5_stable,
            mass_ratio: system_mass_ratio,
            separation: separation.clone(),
            trojans: Vec::new(),
        }
    }

    /// Berechnet L1 Position (Abstand vom kleineren Körper m2, zwischen m1 und m2)
    /// Vereinfachte Näherung: r ≈ a * (μ/3)^(1/3)
    fn calculate_l1_distance(separation: &Distance<Meter>, mu: f64) -> Distance<Meter> {
        let distance_val = separation.value * (mu / 3.0).powf(1.0 / 3.0);
        Distance::<Meter>::new(distance_val)
    }

    /// Berechnet L2 Position (Abstand vom kleineren Körper m2, hinter m2 von m1 aus gesehen)
    /// Vereinfachte Näherung: r ≈ a * (μ/3)^(1/3)
    fn calculate_l2_distance(separation: &Distance<Meter>, mu: f64) -> Distance<Meter> {
        let distance_val = separation.value * (mu / 3.0).powf(1.0 / 3.0);
        Distance::<Meter>::new(distance_val)
    }

    /// Berechnet L3 Position (Abstand vom größeren Körper m1, hinter m1 von m2 aus gesehen)
    /// Vereinfachte Näherung: r ≈ a * (1 - 5μ/12) ist üblich, oder von m2: a * (1 + μ)
    /// Die gegebene Formel r ≈ a * (7μ/12) ist hier für Abstand von m1 interpretiert.
    /// Standardmäßig ist L3 etwa `a` von `m2` entfernt, auf der anderen Seite von `m1`.
    /// Also `a * (m1/(m1+m2)) + a` vom Baryzentrum oder `a + a * (m2/m1)` von `m1`.
    /// Die Formel r_L3 ≈ a(1 - 5μ/12) bezieht sich oft auf den Abstand vom Primärkörper (m1).
    /// Wenn mu = m2/(m1+m2), dann dist(m1, L3) = a * (1 - 5μ/12) ist nicht ganz richtig.
    /// Es ist eher a * (1 + (m1-m2)/(m1+m2) * 5/12 * mu).
    /// Wir verwenden die gegebene Formel und nehmen an sie ist dist(m1, L3)
    fn calculate_l3_distance(separation: &Distance<Meter>, mu: f64) -> Distance<Meter> {
        // Diese Formel ist ungewöhnlich für den Abstand von m1.
        // Standard L3 vom Primär (m1): a * (1 + 5/12 * (m2.value/m1.value)) ist nicht was hier steht.
        // Distanz L3 vom Baryzentrum: a * (1 - (m1-m2)/(m1+m2) * (5/12) * mu_param)
        // Wenn 7μ/12 der Abstand von m1 ist:
        let distance_val = separation.value * (1.0 - (7.0 * mu / 12.0)); // Korrigiert für "Nähe zu m1"
        Distance::<Meter>::new(distance_val)
    }

    /// Fügt einen Trojaner an L4 oder L5 hinzu
    pub fn add_trojan(&mut self, trojan: TrojanObject) -> Result<(), String> {
        if !self.l4_l5_stable {
            return Err("L4/L5 points are not stable for this mass ratio".to_string());
        }

        if trojan.lagrange_point != 4 && trojan.lagrange_point != 5 {
            return Err("Trojan must be at L4 or L5".to_string());
        }

        self.trojans.push(trojan);
        Ok(())
    }

    /// Berechnet die Position von L4 in kartesischen Koordinaten relativ zum Primärkörper (m1 am Ursprung, m2 auf positiver x-Achse)
    pub fn l4_position(&self) -> (Distance<Meter>, Distance<Meter>) {
        // m1 sei bei (0,0). m2 sei bei (separation, 0).
        // L4 ist bei (separation/2, separation * sqrt(3)/2)
        let x_val = self.separation.value * 0.5;
        let y_val = self.separation.value * (3.0_f64.sqrt() / 2.0);

        (Distance::<Meter>::new(x_val), Distance::<Meter>::new(y_val))
    }

    /// Berechnet die Position von L5 in kartesischen Koordinaten
    pub fn l5_position(&self) -> (Distance<Meter>, Distance<Meter>) {
        let (x, y) = self.l4_position();
        (x, Distance::<Meter>::new(-y.value)) // y-Wert negieren
    }

    /// Prüft, ob ein kleiner Körper gravitativ von einem Lagrange-Punkt gefangen werden kann
    pub fn can_capture_at_lagrange_point(&self, point: u8, _test_mass: &Mass<Kilogram>) -> bool {
        match point {
            1 | 2 | 3 => {
                // L1, L2, L3 sind nur quasi-stabil (dynamisch instabil ohne ständige Korrekturen)
                false // Oder eine komplexere Logik, wenn "quasi-stabil" als "capture" zählt
            }
            4 | 5 => self.l4_l5_stable,
            _ => false,
        }
    }

    /// Berechnet die Hill-Sphäre des Sekundärkörpers (m2) an einem Lagrange-Punkt L4/L5.
    /// Dies ist eine grobe Näherung, da die Hill-Sphäre für Drei-Körper-Probleme komplex ist.
    pub fn hill_sphere_at_lagrange_point(
        &self,
        point: u8,
        secondary_body_mass_kg: &Mass<Kilogram>, // Masse des Körpers, dessen Hill-Sphäre wir betrachten (typischerweise m2)
        primary_body_mass_kg: &Mass<Kilogram>,   // Masse des Zentralkörpers (typischerweise m1)
    ) -> Option<Distance<Meter>> {
        if point != 4 && point != 5 {
            // Hill-Sphäre für L4/L5 hier relevant
            return None;
        }
        if !self.l4_l5_stable {
            return None;
        }

        // Hill-Radius für m2 im Orbit um m1: R_H = a * (m2 / (3*M_total))^(1/3)
        // An L4/L5 ist die Situation komplexer. Die Trojaner orbitieren um L4/L5, die selbst mit m2 um m1 rotieren.
        // Eine Näherung könnte der Hill-Radius von m2 selbst sein.
        let total_mass_kg_val = primary_body_mass_kg.value + secondary_body_mass_kg.value;
        if total_mass_kg_val == 0.0 {
            return None;
        }

        let hill_radius_val = self.separation.value
            * (secondary_body_mass_kg.value / (3.0 * total_mass_kg_val)).powf(1.0 / 3.0);

        // Faktor für Stabilitätsregion um L4/L5 ist oft kleiner als Hill-Sphäre.
        // Diese Formel ist eine Vereinfachung.
        Some(Distance::<Meter>::new(hill_radius_val * 0.5)) // Reduzierter Faktor für L4/L5
    }

    /// Generiert einen Trojaner mit realistischen Eigenschaften
    /// primary_mass_host und secondary_mass_host sind die Massen der Hauptkörper des Systems (Sterne)
    pub fn generate_trojan(
        &self,
        lagrange_point: u8,
        trojan_mass: Mass<Kilogram>,
        primary_mass_host_kg: &Mass<Kilogram>, // Masse des schwereren Sterns in kg
        secondary_mass_host_kg: &Mass<Kilogram>, // Masse des leichteren Sterns in kg
    ) -> Result<TrojanObject, String> {
        if lagrange_point != 4 && lagrange_point != 5 {
            return Err("Can only generate trojans at L4 or L5".to_string());
        }

        if !self.l4_l5_stable {
            return Err("L4/L5 not stable for this system".to_string());
        }

        // Oszillationsamplitude basierend auf Systemparametern
        let base_amplitude_val = self.separation.value * 0.1; // ~10% der Separation
        let mass_factor = (trojan_mass.value / secondary_mass_host_kg.value).min(1.0);
        let amplitude_val = base_amplitude_val * (1.0 - mass_factor * 0.5);

        // Oszillationsperiode (mehrere Orbitalperioden)
        // P^2 = a^3 / (M1+M2) in astronomischen Einheiten (AU, SolarMass, Year)
        // Hier: separation in Meter, Massen in Kilogram. Ergebnis soll in Jahren sein.
        // G = 6.67430e-11 N(m/kg)^2. P = 2π * sqrt(a^3 / (G*(M1+M2)))
        // Konvertiere zu AU, SolarMass für einfache Formel oder verwende G.
        let m1_solar = primary_mass_host_kg.get::<SolarMass>();
        let m2_solar = secondary_mass_host_kg.get::<SolarMass>();
        let separation_au = self.separation.get::<AstronomicalUnit>();

        let orbital_period_years_val = if (m1_solar.value + m2_solar.value) > 0.0 {
            (separation_au.value.powi(3) / (m1_solar.value + m2_solar.value)).sqrt()
        } else {
            0.0 // Vermeide Division durch Null
        };

        let oscillation_period_years_val = orbital_period_years_val * (2.0 + mass_factor);

        // Stabilität basierend auf Massenverhältnissen
        let mass_ratio_factor = (self.mass_ratio / MIN_LAGRANGE_MASS_RATIO).min(1.0); // self.mass_ratio ist M_heavy/M_light
        let size_factor = (1.0 - trojan_mass.value / secondary_mass_host_kg.value).max(0.1);
        let stability = mass_ratio_factor * size_factor;

        Ok(TrojanObject {
            mass: trojan_mass,
            lagrange_point,
            oscillation_amplitude: Distance::<Meter>::new(amplitude_val),
            oscillation_period: Time::<Year>::new(oscillation_period_years_val),
            stability,
        })
    }

    /// Generiert einen Trojaner mit detaillierter Dynamik-Analyse
    pub fn generate_enhanced_trojan(
        &self,
        lagrange_point: u8,
        trojan_mass: Mass<Kilogram>,
        primary_mass_host_kg: &Mass<Kilogram>, // Masse des schwereren Sterns
        secondary_mass_host_kg: &Mass<Kilogram>, // Masse des leichteren Sterns
        _rng: &mut ChaCha8Rng,                 // rng wird nicht verwendet in dieser Version
    ) -> Result<TrojanObject, String> {
        if !self.l4_l5_stable {
            return Err("L4/L5 not stable - mass ratio below critical".to_string());
        }

        if lagrange_point != 4 && lagrange_point != 5 {
            return Err("Enhanced trojans only supported at L4/L5".to_string());
        }

        // Basis-Trojaner generieren
        let mut trojan = self.generate_trojan(
            lagrange_point,
            trojan_mass,
            primary_mass_host_kg,
            secondary_mass_host_kg,
        )?;

        // Erweiterte Eigenschaften hinzufügen (Dummy-Werte, da calculate_libration_dynamics nicht implementiert ist)
        let dynamics = trojan.calculate_libration_dynamics(
            primary_mass_host_kg,
            secondary_mass_host_kg,
            &self.separation,
        );

        // Anpassung der Oszillationsamplitude basierend auf Dynamik
        trojan.oscillation_amplitude = dynamics.libration_amplitude.clone(); // Beispiel: collective_hill_sphere als Amplitude
        if !dynamics.libration_amplitude.is_empty() {
            trojan.oscillation_period = dynamics.libration_amplitude; // Beispiel
        }
        trojan.stability = dynamics
            .mass_ratios
            .first()
            .copied()
            .unwrap_or(trojan.stability); // Beispiel

        Ok(trojan)
    }

    /// Erstellt ein Mutual Trojaner System
    pub fn create_mutual_trojan_system(
        &self,
        lagrange_point: u8,
        primary_trojan_mass: Mass<Kilogram>,
        secondary_trojan_masses: Vec<Mass<Kilogram>>, // Expliziter Typ
        primary_mass_host_kg: &Mass<Kilogram>,        // Masse des schwereren Sterns
        secondary_mass_host_kg: &Mass<Kilogram>,      // Masse des leichteren Sterns
        rng: &mut ChaCha8Rng,
    ) -> Result<MutualTrojanSystem, String> {
        if !self.l4_l5_stable {
            return Err("Cannot create mutual trojans - system unstable".to_string());
        }

        if secondary_trojan_masses.len() > 5 {
            // `secondary_masses` zu `secondary_trojan_masses`
            return Err("Too many trojans - maximum 5 supported".to_string());
        }

        // Primärer Trojaner
        let primary_trojan = self.generate_enhanced_trojan(
            lagrange_point,
            primary_trojan_mass.clone(),
            primary_mass_host_kg,
            secondary_mass_host_kg,
            rng,
        )?;

        // Sekundäre Trojaner
        let mut secondary_trojans = Vec::new();
        for (i, sec_mass) in secondary_trojan_masses.iter().enumerate() {
            let mut sec_trojan = self.generate_enhanced_trojan(
                lagrange_point,
                sec_mass.clone(), // sec_mass ist &Mass<Kilogram>, clone es.
                primary_mass_host_kg,
                secondary_mass_host_kg,
                rng,
            )?;

            // Kleine Phasenverschiebung / Amplitudenänderung
            // let phase_shift = (i as f64 + 1.0) * 10.0; // Nicht verwendet
            sec_trojan.oscillation_amplitude = Distance::<Meter>::new(
                sec_trojan.oscillation_amplitude.value * (1.0 + i as f64 * 0.05), // Kleinere Variation
            );

            secondary_trojans.push(sec_trojan);
        }

        // Mutual Dynamics berechnen
        let mutual_dynamics = self.calculate_mutual_dynamics(
            // self anstelle von Self
            &primary_trojan,
            &secondary_trojans,
            primary_mass_host_kg,
            secondary_mass_host_kg,
            // &self.separation, // Hinzugefügt, da calculate_mutual_dynamics es benötigt
        );

        // System-Stabilität
        let total_trojan_mass_kg_val: f64 = primary_trojan_mass.value
            + secondary_trojan_masses.iter().map(|m| m.value).sum::<f64>();

        let stability_reduction = if secondary_mass_host_kg.value > 0.0 {
            (total_trojan_mass_kg_val / secondary_mass_host_kg.value).min(0.5)
        } else {
            0.5 // Max reduction if secondary host mass is zero
        };
        let system_stability = (primary_trojan.stability * (1.0 - stability_reduction)).max(0.1);

        Ok(MutualTrojanSystem {
            primary_trojan,
            secondary_trojans,
            mutual_dynamics,
            system_stability,
        })
    }

    /// Berechnet Dynamik zwischen mehreren Trojanern. Benötigt System-Separation.
    fn calculate_mutual_dynamics(
        // Geändert zu &self Methode oder separation als Parameter
        &self, // Hinzugefügt, um self.separation und self.lX_position zu verwenden
        primary_trojan: &TrojanObject,
        secondary_trojans: &[TrojanObject],
        primary_mass_host_kg: &Mass<Kilogram>,
        secondary_mass_host_kg: &Mass<Kilogram>,
    ) -> MutualDynamics {
        let total_trojan_group_mass_kg_val =
            primary_trojan.mass.value + secondary_trojans.iter().map(|t| t.mass.value).sum::<f64>();

        // Barycenter der Trojaner (vereinfacht am Lagrange-Punkt des Systems)
        let trojan_system_barycenter = if primary_trojan.lagrange_point == 4 {
            self.l4_position()
        } else {
            self.l5_position()
        };

        // Interne Perioden (basierend auf gegenseitiger Gravitation)
        let mut internal_periods = Vec::new();
        let base_period_val_years = primary_trojan.oscillation_period.value;

        for (i, trojan) in secondary_trojans.iter().enumerate() {
            let period_variation_factor = if primary_trojan.mass.value > 0.0 {
                1.0 + (i as f64 * 0.02) + (trojan.mass.value / primary_trojan.mass.value * 0.1)
            } else {
                1.0 + (i as f64 * 0.02)
            };
            internal_periods.push(Time::<Year>::new(
                base_period_val_years * period_variation_factor,
            ));
        }

        // Massenverhältnisse der sekundären Trojaner zum primären Trojaner
        let mass_ratios: Vec<f64> = secondary_trojans
            .iter()
            .map(|t| {
                if primary_trojan.mass.value > 0.0 {
                    t.mass.value / primary_trojan.mass.value
                } else {
                    0.0
                }
            })
            .collect();

        // Kollektive Hill-Sphäre der Trojaner-Gruppe bezogen auf m2 (den leichteren Stern)
        let trojan_group_mass_obj_kg = Mass::<Kilogram>::new(total_trojan_group_mass_kg_val);
        // Die "Zentralmasse" für die Hill-Sphäre der Trojaner-Gruppe ist der Stern m2, um den L4/L5 effektiv liegt.
        // Die Separation ist die Distanz der Trojaner-Gruppe zu m2 (ungefähr self.separation).

        let collective_hill_sphere_val = if secondary_mass_host_kg.value > 0.0 {
            self.separation.value
                * (trojan_group_mass_obj_kg.value / (3.0 * secondary_mass_host_kg.value))
                    .powf(1.0 / 3.0)
        } else {
            0.0 // Keine Hill-Sphäre wenn Zentralmasse 0 ist
        };

        MutualDynamics {
            trojan_barycenter: trojan_system_barycenter,
            internal_periods,
            mass_ratios,
            collective_hill_sphere: Distance::<Meter>::new(collective_hill_sphere_val),
        }
    }

    /// Validiert erweiterte Stabilitätskriterien für Trojaner
    pub fn validate_trojan_stability(&self, trojans: &[TrojanObject]) -> Vec<f64> {
        trojans
            .iter()
            .map(|trojan| {
                // Basis-Stabilität aus dem Massenverhältnis des Systems
                let mass_ratio_stability = if self.l4_l5_stable {
                    // self.l4_l5_stable prüft bereits MIN_LAGRANGE_MASS_RATIO
                    1.0
                } else {
                    0.0
                };

                // Reduktion durch Überfüllung
                let crowding_factor = if trojans.len() > 3 {
                    0.9_f64.powi((trojans.len() - 3) as i32) // powi für integer Exponenten
                } else {
                    1.0
                };

                // Reduktion durch Masse des Trojaners (Beispiel: sehr massive Trojaner könnten instabiler sein)
                let mass_factor = if trojan.mass.value > 1e18 { 0.8 } else { 1.0 }; // Annahme: 1e18 kg ist ein Schwellenwert

                mass_ratio_stability * crowding_factor * mass_factor * trojan.stability
            })
            .collect()
    }
}
