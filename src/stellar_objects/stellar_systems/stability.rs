use crate::physics::astrophysics::LagrangePointsStatus;
use crate::physics::astrophysics::orbit::two_body::BinaryOrbit;
use crate::physics::units::{Distance, Mass, Time};
use crate::stellar_objects::stars::properties::StellarProperties;
use crate::stellar_objects::stellar_systems::hierarchy::SystemHierarchy;
use crate::stellar_objects::stellar_systems::types::SystemType;
use crate::stellar_objects::trojans_asteroid::objects::MutualTrojanSystem;
use serde::{Deserialize, Serialize};

/// Stabilitätsanalyse für Sternsysteme über Millionen-Jahre-Zeiträume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStability {
    /// Charakteristische Stabilitäts-Zeitskala (typisch 1-10 Myr)
    pub stability_timescale: Time,
    /// Wahrscheinlichkeit für Sternauswurf in 1 Myr (0.0-1.0)
    pub ejection_probability: f64,
    /// Kollisionsrisiko in 1 Myr (0.0-1.0)
    pub collision_risk: f64,
    /// Hill-Sphären überlappen sich (instabil wenn true)
    pub hill_sphere_overlap: bool,
    /// Gesamtstabilitäts-Score (0.0-1.0, höher = stabiler)
    pub overall_stability_score: f64,
    /// Detaillierte Risikofaktoren
    pub risk_factors: Vec<StabilityRiskFactor>,
    pub trojan_analysis: Option<TrojanStabilityAnalysis>,
}

/// Spezifische Risikofaktoren für Systemstabilität
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StabilityRiskFactor {
    pub name: String,
    pub severity: f64,    // 0.0-1.0
    pub probability: f64, // 0.0-1.0
    pub description: String,
}

/// Stabilitätsanalyse für Trojaner-Systeme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanStabilityAnalysis {
    /// Anzahl stabiler Trojaner
    pub stable_trojans_count: usize,
    /// Anzahl instabiler Trojaner  
    pub unstable_trojans_count: usize,
    /// Mutual Trojaner Systeme
    pub mutual_systems: Vec<MutualTrojanSystem>,
    /// Durchschnittliche Trojaner-Stabilität
    pub average_trojan_stability: f64,
    /// Trojaner-spezifische Risikofaktoren
    pub trojan_risks: Vec<StabilityRiskFactor>,
    /// Lagrange-Punkte Status
    pub lagrange_points_status: LagrangePointsStatus,
}

/// Detaillierte Analyse der Hill-Sphären-Dynamik
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HillSphereAnalysis {
    /// Hill-Radius für jede Komponente
    pub hill_radii: Vec<Distance>,
    /// Minimum-Abstand zwischen Komponenten
    pub minimum_separations: Vec<Distance>,
    /// Stabilität basierend auf Hill-Kriterium
    pub hill_stability_ratios: Vec<f64>,
}

impl SystemStability {
    // ERSETZE die bestehende analyze_system Methode:
    pub fn analyze_system(system_type: &SystemType) -> Self {
        Self::analyze_system_enhanced(system_type)
    }

    // NEU hinzufügen:
    pub fn analyze_system_enhanced(system_type: &SystemType) -> Self {
        match system_type {
            SystemType::Single(_) => Self::single_star_stability(),
            SystemType::Binary {
                primary,
                secondary,
                orbital_properties,
            } => Self::binary_stability_with_trojans(primary, secondary, orbital_properties),
            SystemType::Multiple {
                components,
                hierarchy,
            } => {
                let mut base = Self::multiple_star_stability(components, hierarchy);
                base.trojan_analysis = None; // Trojaner nur in Binärsystemen
                base
            }
        }
    }

    /// Stabilität für Einzelsternsysteme (immer stabil)
    fn single_star_stability() -> Self {
        Self {
            stability_timescale: Time::years(1e10), // Stellar evolution timescale
            ejection_probability: 0.0,
            collision_risk: 0.0,
            hill_sphere_overlap: false,
            overall_stability_score: 1.0,
            risk_factors: vec![],
            trojan_analysis: None,
        }
    }

    /// Stabilitätsanalyse für Binärsysteme
    fn binary_stability(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        orbital_properties: &BinaryOrbit,
    ) -> Self {
        let mass_ratio = primary.mass.in_kg() / secondary.mass.in_kg();
        let separation = orbital_properties.orbital_elements.semimajor_axis.in_au();
        let eccentricity = orbital_properties.orbital_elements.eccentricity;

        // Einfache Heuristiken basierend auf dem Artikel
        let mut risk_factors = Vec::new();
        let mut overall_score: f64 = 1.0;

        // Risiko durch hohe Exzentrizität
        if eccentricity > 0.7 {
            let risk = StabilityRiskFactor {
                name: "High eccentricity".to_string(),
                severity: eccentricity,
                probability: 0.3,
                description: format!(
                    "Eccentricity of {:.2} may cause close approaches",
                    eccentricity
                ),
            };
            overall_score *= 0.8;
            risk_factors.push(risk);
        }

        // Risiko durch geringe Separation
        if separation < 0.1 {
            let risk = StabilityRiskFactor {
                name: "Close binary".to_string(),
                severity: 0.1 / separation.max(0.01),
                probability: 0.5,
                description: format!(
                    "Separation of {:.3} AU may lead to mass transfer",
                    separation
                ),
            };
            overall_score *= 0.6;
            risk_factors.push(risk);
        }

        // Risiko durch extreme Massenverhältnisse
        if mass_ratio > 10.0 || mass_ratio < 0.1 {
            let risk = StabilityRiskFactor {
                name: "Extreme mass ratio".to_string(),
                severity: if mass_ratio > 10.0 {
                    mass_ratio / 50.0
                } else {
                    (0.1 / mass_ratio) / 10.0
                },
                probability: 0.2,
                description: format!(
                    "Mass ratio of {:.1}:1 may cause orbital evolution",
                    mass_ratio
                ),
            };
            overall_score *= 0.9;
            risk_factors.push(risk);
        }

        // Stabilität über Millionen Jahre (meist sehr hoch für Binärsysteme)
        let stability_myr = if separation > 0.05 && eccentricity < 0.9 {
            Time::years(1e8) // 100 Myr für normale Binärsysteme
        } else {
            Time::years(1e6) // 1 Myr für extreme Binärsysteme
        };

        Self {
            stability_timescale: stability_myr,
            ejection_probability: 0.01, // Sehr niedrig für Binärsysteme
            collision_risk: if separation < 0.01 { 0.1 } else { 0.001 },
            hill_sphere_overlap: false, // Binärsysteme haben definitionsgemäß keine Überlappung
            overall_stability_score: overall_score.max(0.1),
            risk_factors,
            trojan_analysis: None,
        }
    }
    fn binary_stability_with_trojans(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        orbital_properties: &BinaryOrbit,
    ) -> Self {
        // Basis-Binär-Stabilität
        let mut base_stability = Self::binary_stability(primary, secondary, orbital_properties);

        // Trojaner-Analyse hinzufügen
        let trojan_analysis = Self::analyze_binary_trojans(primary, secondary, orbital_properties);

        // Gesamtstabilität anpassen
        if let Some(ref trojan_data) = trojan_analysis {
            if trojan_data.stable_trojans_count > 0 {
                base_stability.overall_stability_score *= 1.05;
            }
            if trojan_data.unstable_trojans_count > 0 {
                base_stability.overall_stability_score *= 0.95;
            }
        }

        base_stability.trojan_analysis = trojan_analysis;
        base_stability
    }

    fn analyze_binary_trojans(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        orbital_properties: &BinaryOrbit,
    ) -> Option<TrojanStabilityAnalysis> {
        if let Some(ref lagrange_system) = orbital_properties.lagrange_system {
            let mut trojan_risks = Vec::new();
            let mut stable_count = 0;
            let mut unstable_count = 0;
            let mut total_stability = 0.0;

            for trojan in &lagrange_system.trojans {
                let dynamics = trojan.calculate_libration_dynamics(
                    &primary.mass,
                    &secondary.mass,
                    &orbital_properties.orbital_elements.semimajor_axis,
                );

                if dynamics.long_term_stability > 0.7 {
                    stable_count += 1;
                } else {
                    unstable_count += 1;
                    trojan_risks.push(StabilityRiskFactor {
                        name: format!("Unstable trojan at L{}", trojan.lagrange_point),
                        severity: 1.0 - dynamics.long_term_stability,
                        probability: 0.8,
                        description: format!(
                            "Trojan instability: {:.2}",
                            dynamics.long_term_stability
                        ),
                    });
                }
                total_stability += dynamics.long_term_stability;
            }

            let lagrange_status = LagrangePointsStatus {
                l1_stable: false,
                l2_stable: false,
                l3_stable: false,
                l4_stable: lagrange_system.l4_l5_stable,
                l5_stable: lagrange_system.l4_l5_stable,
                l4_trojans: lagrange_system
                    .trojans
                    .iter()
                    .filter(|t| t.lagrange_point == 4)
                    .cloned()
                    .collect(),
                l5_trojans: lagrange_system
                    .trojans
                    .iter()
                    .filter(|t| t.lagrange_point == 5)
                    .cloned()
                    .collect(),
            };

            let average_stability = if lagrange_system.trojans.is_empty() {
                0.0
            } else {
                total_stability / lagrange_system.trojans.len() as f64
            };

            Some(TrojanStabilityAnalysis {
                stable_trojans_count: stable_count,
                unstable_trojans_count: unstable_count,
                mutual_systems: Vec::new(),
                average_trojan_stability: average_stability,
                trojan_risks,
                lagrange_points_status: lagrange_status,
            })
        } else {
            None
        }
    }

    /// Stabilitätsanalyse für Mehrsternsysteme (komplexer)
    pub fn multiple_star_stability(
        components: &[StellarProperties],
        hierarchy: &SystemHierarchy,
    ) -> Self {
        let n_bodies = components.len();
        let mut risk_factors = Vec::new();
        let mut overall_score: f64 = hierarchy.stability_factor;

        // Hill-Sphären-Analyse
        let hill_analysis = Self::analyze_hill_spheres(components, hierarchy);
        let hill_overlap = hill_analysis
            .hill_stability_ratios
            .iter()
            .any(|&ratio| ratio < 2.5); // Kritisches Verhältnis nach dem Artikel

        if hill_overlap {
            let risk = StabilityRiskFactor {
                name: "Hill sphere overlap".to_string(),
                severity: 0.9,
                probability: 0.8,
                description: "Hill spheres overlap, indicating potential instability".to_string(),
            };
            overall_score *= 0.3;
            risk_factors.push(risk);
        }

        // Risiko durch Anzahl der Körper (n-body chaos)
        if n_bodies > 3 {
            let chaos_factor = (n_bodies as f64 - 3.0) * 0.2;
            let risk = StabilityRiskFactor {
                name: format!("{}-body chaos", n_bodies),
                severity: chaos_factor.min(0.9),
                probability: 0.6,
                description: format!("Systems with {} bodies are inherently chaotic", n_bodies),
            };
            overall_score *= (1.0 - chaos_factor * 0.3).max(0.2);
            risk_factors.push(risk);
        }

        // Berechnung der charakteristischen Zeitskalen
        let shortest_period = hierarchy.dynamical_timescale().in_years();
        let n_body_factor = (n_bodies as f64).ln(); // Logarithmische Skalierung

        // Stabilität nimmt mit Komplexität ab
        let stability_years = (1e6 * hierarchy.stability_factor / n_body_factor).max(1e4);

        // Auswurfwahrscheinlichkeit steigt mit n-body Komplexität
        let ejection_prob = match n_bodies {
            3 => 0.05,
            4 => 0.15,
            5 => 0.3,
            _ => 0.5,
        } * (1.0 - hierarchy.stability_factor);

        // Kollisionsrisiko basierend auf Hill-Analyse
        let collision_risk = if hill_overlap { 0.2 } else { 0.01 };

        Self {
            stability_timescale: Time::years(stability_years),
            ejection_probability: ejection_prob.min(0.9),
            collision_risk,
            hill_sphere_overlap: hill_overlap,
            overall_stability_score: overall_score.max(0.05),
            risk_factors,
            trojan_analysis: None,
        }
    }

    /// Analysiert Hill-Sphären für Mehrsternsysteme
    fn analyze_hill_spheres(
        components: &[StellarProperties],
        hierarchy: &SystemHierarchy,
    ) -> HillSphereAnalysis {
        let mut hill_radii = Vec::new();
        let mut minimum_separations = Vec::new();
        let mut stability_ratios = Vec::new();

        for (i, level) in hierarchy.hierarchy_levels.iter().enumerate() {
            if level.components.len() >= 2 {
                let comp1_idx = level.components[0];
                let comp2_idx = level.components[1];

                if comp1_idx < components.len() && comp2_idx < components.len() {
                    let comp1 = &components[comp1_idx];
                    let comp2 = &components[comp2_idx];

                    // Hill radius für die weniger massive Komponente
                    let smaller_mass = if comp1.mass.in_kg() < comp2.mass.in_kg() {
                        &comp1.mass
                    } else {
                        &comp2.mass
                    };
                    let total_mass = Mass::kilograms(comp1.mass.in_kg() + comp2.mass.in_kg());

                    let hill_radius = level.orbit.hill_radius(smaller_mass, &total_mass);
                    let separation = level.orbit.semimajor_axis.clone();

                    // Stabilitätsverhältnis: Separation / Hill-Radius
                    let stability_ratio = separation.in_meters() / hill_radius.in_meters();

                    hill_radii.push(hill_radius);
                    minimum_separations.push(separation);
                    stability_ratios.push(stability_ratio);
                }
            }
        }

        HillSphereAnalysis {
            hill_radii,
            minimum_separations,
            hill_stability_ratios: stability_ratios,
        }
    }

    /// Gibt eine menschenlesbare Zusammenfassung der Stabilität zurück
    pub fn stability_summary(&self) -> String {
        let stability_class = match self.overall_stability_score {
            s if s > 0.8 => "Highly Stable",
            s if s > 0.6 => "Moderately Stable",
            s if s > 0.4 => "Marginally Stable",
            s if s > 0.2 => "Unstable",
            _ => "Highly Unstable",
        };

        format!(
            "{} (Score: {:.2}) - Stable for ~{:.1} Myr",
            stability_class,
            self.overall_stability_score,
            self.stability_timescale.in_years() / 1e6
        )
    }

    /// Prüft ob das System über 1 Myr stabil ist
    pub fn is_million_year_stable(&self) -> bool {
        self.stability_timescale.in_years() >= 1e6 && self.overall_stability_score > 0.5
    }
}
