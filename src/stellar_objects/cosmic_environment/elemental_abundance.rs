use crate::stellar_objects::cosmic_environment::epoch::CosmicEpoch;
use serde::{Deserialize, Serialize};

/// Elementhäufigkeiten in der kosmischen Umgebung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalAbundance {
    /// Wasserstoff (Massenanteil)
    pub hydrogen: f64,
    /// Helium (Massenanteil)
    pub helium: f64,
    /// Lithium (Massenanteil)
    pub lithium: f64,
    /// Kohlenstoff (Massenanteil)
    pub carbon: f64,
    /// Stickstoff (Massenanteil)
    pub nitrogen: f64,
    /// Sauerstoff (Massenanteil)
    pub oxygen: f64,
    /// Schwere Metalle (Z > 8, Massenanteil)
    pub heavy_metals: f64,
    /// Alpha-Elemente (O, Ne, Mg, Si, S, Ar, Ca, Ti)
    pub alpha_elements: f64,
    /// Eisengruppe (Fe, Co, Ni)
    pub iron_group: f64,
    /// s-Prozess Elemente
    pub s_process_elements: f64,
    /// r-Prozess Elemente
    pub r_process_elements: f64,
}

impl ElementalAbundance {
    /// Erstellt eine neue Elementhäufigkeit aus Metallizität und kosmischer Epoche
    ///
    /// # Parameter
    /// * `metallicity` - Metallizität Z (Massenanteil aller Elemente Z > 2)
    /// * `epoch` - Kosmische Epoche zur Bestimmung der Elementverteilung
    ///
    /// # Wissenschaftliche Grundlagen
    /// - Basiert auf Big Bang Nukleosynthese für H/He Verhältnisse
    /// - Berücksichtigt stellare Nukleosynthese und chemische Evolution
    /// - Alpha-Enhancement in frühen kosmischen Epochen
    /// - Typische Elementverteilungen aus Beobachtungen
    pub fn from_metallicity_and_epoch(metallicity: f64, epoch: &CosmicEpoch) -> Self {
        // Begrenzung der Metallizität auf physikalisch sinnvolle Werte
        let z = metallicity.clamp(0.0, 0.1);

        // Primordiale Helium-Häufigkeit aus Big Bang Nukleosynthese
        let primordial_helium = 0.2485; // Yp ≈ 0.2485

        // Helium-Anreicherung durch stellare Nukleosynthese
        // ΔY/ΔZ ≈ 2.0 (typischer Wert)
        let helium_enrichment_ratio = 2.0;
        let helium = primordial_helium + helium_enrichment_ratio * z;

        // Wasserstoff als Rest
        let hydrogen = 1.0 - helium - z;

        // Alpha-Enhancement Faktor basierend auf kosmischer Epoche
        let alpha_enhancement = match epoch.era.as_str() {
            "Primordial Era" => 0.6,      // Sehr frühe Population II Sterne
            "Early Universe" => 0.5,      // [α/Fe] ≈ +0.5 für frühe Epochen
            "Peak Star Formation" => 0.3, // [α/Fe] ≈ +0.3
            "Stellar Era" => 0.1,         // Übergang zu solaren Verhältnissen
            "Mature Universe" => 0.0,     // Solare [α/Fe] Verhältnisse
            "Late Universe" => -0.1,      // Leicht sub-solare α/Fe
            _ => 0.2,
        };

        // Zeitabhängiger r-Prozess Enhancement basierend auf Epoche und Redshift
        let r_process_enhancement = match epoch.era.as_str() {
            "Primordial Era" => 0.5,      // Wenige frühe r-Prozess Ereignisse
            "Early Universe" => 2.0,      // Verstärkte Neutronenstern-Kollisionen
            "Peak Star Formation" => 1.8, // Höchste Core-Collapse SN Rate
            "Stellar Era" => 1.2,
            "Mature Universe" => 1.0, // Moderne Verhältnisse
            "Late Universe" => 0.8,   // Reduzierte Aktivität
            _ => 1.0,
        };

        // s-Prozess Enhancement - AGB-Sterne brauchen Zeit zur Entwicklung
        let s_process_enhancement = match epoch.era.as_str() {
            "Primordial Era" => 0.1,      // Kaum AGB-Sterne
            "Early Universe" => 0.3,      // Wenige AGB-Sterne
            "Peak Star Formation" => 0.7, // Erste Generation AGB-Sterne
            "Stellar Era" => 1.0,         // Voll entwickelte AGB-Population
            "Mature Universe" => 1.3,     // Maximum der s-Prozess Produktion
            "Late Universe" => 1.5,       // Akkumulierte s-Prozess Elemente
            _ => 1.0,
        };

        // Elementverteilung basierend auf solaren Häufigkeiten und Nukleosynthese
        // Referenz: Asplund et al. 2009, Lodders 2003

        // CNO-Enhancement in frühen Epochen (Population II Sterne)
        let cno_enhancement = match epoch.era.as_str() {
            "Primordial Era" => 0.7, // Weniger CNO-Cycling
            "Early Universe" => 0.8,
            "Peak Star Formation" => 0.9,
            _ => 1.0,
        };

        // Grundfraktionen basierend auf solaren Häufigkeiten
        let base_oxygen_fraction = 0.457;
        let base_carbon_fraction = 0.236;
        let base_nitrogen_fraction = 0.070;
        let base_iron_fraction = 0.295;
        let base_alpha_fraction = 0.520;
        let base_s_process_fraction = 0.015;
        let base_r_process_fraction = 0.008;

        // Anpassung der Fraktionen basierend auf epochenspezifischen Effekten
        let alpha_factor = 1.0 + alpha_enhancement;
        let iron_factor = 1.0 / alpha_factor;

        // Normalisierung um Massenerhaltung sicherzustellen
        let total_factor = base_oxygen_fraction * alpha_factor
            + base_carbon_fraction * cno_enhancement
            + base_nitrogen_fraction * cno_enhancement * cno_enhancement
            + base_iron_fraction * iron_factor
            + base_alpha_fraction * alpha_factor
            + base_s_process_fraction * s_process_enhancement
            + base_r_process_fraction * r_process_enhancement;

        // Normalisierte Fraktionen
        let oxygen_fraction = base_oxygen_fraction * alpha_factor / total_factor;
        let carbon_fraction = base_carbon_fraction * cno_enhancement / total_factor;
        let nitrogen_fraction =
            base_nitrogen_fraction * cno_enhancement * cno_enhancement / total_factor;
        let iron_fraction = base_iron_fraction * iron_factor / total_factor;
        let alpha_fraction = base_alpha_fraction * alpha_factor / total_factor;
        let s_process_fraction = base_s_process_fraction * s_process_enhancement / total_factor;
        let r_process_fraction = base_r_process_fraction * r_process_enhancement / total_factor;

        // Anwendung auf die gegebene Metallizität
        let oxygen = z * oxygen_fraction;
        let carbon = z * carbon_fraction;
        let nitrogen = z * nitrogen_fraction;
        let iron_group = z * iron_fraction;
        let alpha_elements = z * alpha_fraction;
        let s_process_elements = z * s_process_fraction;
        let r_process_elements = z * r_process_fraction;

        // Lithium - spezielle Behandlung wegen stellarer Zerstörung und kosmischer Zeit
        let lithium = match epoch.era.as_str() {
            "Primordial Era" => {
                // Primordiales 7Li aus BBN
                2.5e-10 * (1.0 + epoch.age_universe / 0.1) // Langsame Anreicherung
            }
            _ => {
                // Lithium wird in Sternen zerstört, aber auch produziert (AGB, Novae)
                let destruction_factor = match epoch.era.as_str() {
                    "Early Universe" => 0.3,       // Weniger Durchmischung
                    "Peak Star Formation" => 0.15, // Viel stellare Aktivität
                    _ => 0.1,                      // Nur ~10% überlebt
                };
                let production_factor = epoch.age_universe / 13.8; // Zeit für Li-Produktion
                (2.5e-10 + z * 1e-9 * production_factor) * destruction_factor
            }
        };

        // Schwere Metalle (Z > 8) - alle außer H, He, Li, C, N, O
        let light_elements = carbon + nitrogen + oxygen + lithium;
        let heavy_metals = (z - light_elements).max(0.0);

        Self {
            hydrogen,
            helium,
            lithium,
            carbon,
            nitrogen,
            oxygen,
            heavy_metals,
            alpha_elements,
            iron_group,
            s_process_elements,
            r_process_elements,
        }
    }

    /// Erstellt Elementhäufigkeiten nur aus einer CosmicEpoch (verwendet epoch_metallicity)
    pub fn from_epoch(epoch: &CosmicEpoch) -> Self {
        // Konvertiere [Z/H] zu absoluter Metallizität
        let solar_metallicity = 0.0134; // Z_solar ≈ 0.0134
        let absolute_metallicity = solar_metallicity * 10_f64.powf(epoch.epoch_metallicity);

        Self::from_metallicity_and_epoch(absolute_metallicity, epoch)
    }

    /// Validiert, dass die Massenanteile physikalisch sinnvoll sind
    pub fn validate(&self) -> Result<(), String> {
        let total_mass = self.hydrogen
            + self.helium
            + self.lithium
            + self.carbon
            + self.nitrogen
            + self.oxygen
            + self.heavy_metals;

        if (total_mass - 1.0).abs() > 1e-6 {
            return Err(format!("Gesamtmasse ist nicht 1.0: {}", total_mass));
        }

        // Prüfe auf negative Werte
        if self.hydrogen < 0.0
            || self.helium < 0.0
            || self.lithium < 0.0
            || self.carbon < 0.0
            || self.nitrogen < 0.0
            || self.oxygen < 0.0
        {
            return Err("Negative Massenanteile sind nicht physikalisch".to_string());
        }

        Ok(())
    }

    /// Berechnet die Metallizität Z aus den Elementhäufigkeiten
    pub fn metallicity(&self) -> f64 {
        1.0 - self.hydrogen - self.helium
    }

    /// Berechnet das [Fe/H] Verhältnis relativ zur Sonne
    pub fn iron_to_hydrogen_ratio(&self) -> f64 {
        const SOLAR_FE_H: f64 = 7.50e-5; // Solare Fe/H Massenverhältnis
        let fe_h = self.iron_group / self.hydrogen;
        (fe_h / SOLAR_FE_H).log10()
    }

    /// Berechnet das [α/Fe] Verhältnis
    pub fn alpha_to_iron_ratio(&self) -> f64 {
        const SOLAR_ALPHA_FE: f64 = 1.76; // Solares α/Fe Verhältnis
        if self.iron_group > 0.0 {
            let alpha_fe = self.alpha_elements / self.iron_group;
            (alpha_fe / SOLAR_ALPHA_FE).log10()
        } else {
            0.0
        }
    }
}
