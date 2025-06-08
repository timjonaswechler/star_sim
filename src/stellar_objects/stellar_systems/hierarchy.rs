use crate::physics::astrophysics::OrbitalElements;
use crate::physics::units::{Distance, Mass, Time};
use crate::stellar_objects::stars::properties::StellarProperties;
use crate::stellar_objects::stellar_systems::stability::SystemStability;
use serde::{Deserialize, Serialize};

/// Hierarchische Struktur für Mehrsternsysteme///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHierarchy {
    /// Hierarchische Ebenen von innersten zu äußersten Orbits
    pub hierarchy_levels: Vec<HierarchyLevel>,
    /// Gesamtstabilität des Systems (0.0-1.0)
    pub stability_factor: f64,
    /// Charakteristische Zeitskala für chaotische Entwicklung
    pub chaos_timescale: Time,
}

/// Eine Ebene in der Systemhierarchie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyLevel {
    /// Orbitale Elemente für diese Ebene
    pub orbit: OrbitalElements,
    /// Komponenten auf dieser Ebene
    pub components: Vec<usize>, // Indizes in components-Array
    /// Massenverhältnis der Komponenten
    pub mass_ratio: f64,
    /// Stabilität dieser Ebene
    pub level_stability: f64,
}

impl SystemHierarchy {
    pub fn new(components: &[StellarProperties]) -> Self {
        let mut hierarchy_levels = Vec::new();
        let n = components.len();

        if n < 3 {
            return Self {
                hierarchy_levels,
                stability_factor: 1.0,
                chaos_timescale: Time::years(1e12),
            };
        }

        let _total_mass: f64 = components.iter().map(|c| c.mass.in_kg()).sum();
        let mut current_separation = Distance::au(1.0);

        for i in 0..(n - 1) {
            let mass_ratio = components[i].mass.in_kg() / components[i + 1].mass.in_kg();
            let level_stability =
                Self::estimate_level_stability(mass_ratio, current_separation.in_au());

            let level = HierarchyLevel {
                orbit: OrbitalElements::new(current_separation.clone(), 0.1, 0.0, 0.0, 0.0, 0.0),
                components: vec![i, i + 1],
                mass_ratio,
                level_stability,
            };

            hierarchy_levels.push(level);
            current_separation =
                Distance::new(current_separation.value * 3.0, current_separation.system);
        }

        let stability_factor = hierarchy_levels
            .iter()
            .map(|level| level.level_stability)
            .fold(1.0, |acc, s| acc * s);

        let shortest_period_years = hierarchy_levels
            .first()
            .map(|level| {
                let total_mass_kg = components
                    .iter()
                    .take(2)
                    .map(|c| c.mass.in_kg())
                    .sum::<f64>();
                let total_mass_solar = Mass::kilograms(total_mass_kg);
                level.orbit.orbital_period(&total_mass_solar).in_years()
            })
            .unwrap_or(1.0);

        let chaos_timescale = Time::years(shortest_period_years * 1e6 * stability_factor);

        Self {
            hierarchy_levels,
            stability_factor,
            chaos_timescale,
        }
    }

    fn estimate_level_stability(mass_ratio: f64, separation_au: f64) -> f64 {
        let mass_factor = if mass_ratio > 10.0 || mass_ratio < 0.1 {
            0.9
        } else {
            0.5
        };

        let separation_factor = if separation_au > 10.0 {
            0.9
        } else if separation_au > 1.0 {
            0.7
        } else {
            0.3
        };

        mass_factor * separation_factor
    }

    pub fn is_long_term_stable(&self) -> bool {
        self.stability_factor > 0.7 && self.chaos_timescale.in_years() > 1e9
    }

    pub fn dynamical_timescale(&self) -> Time {
        self.hierarchy_levels
            .first()
            .map(|level| {
                let a_au = level.orbit.semimajor_axis.in_au();
                Time::years(a_au.powf(1.5))
            })
            .unwrap_or(Time::years(1.0))
    }
    /// Erweiterte Stabilitätsbewertung über Millionen-Jahre-Zeitrahmen
    pub fn assess_million_year_stability(
        &self,
        components: &[StellarProperties],
    ) -> SystemStability {
        SystemStability::multiple_star_stability(components, self)
    }

    /// Berechnet charakteristische Zeitskala für chaotische Evolution
    pub fn chaos_timescale_estimate(&self) -> Time {
        let base_timescale = self.dynamical_timescale().in_years();
        let n_levels = self.hierarchy_levels.len() as f64;

        // Chaotische Zeitskala skaliert exponentiell mit Komplexität
        let chaos_years = base_timescale * (1e3 * self.stability_factor / n_levels.ln());
        Time::years(chaos_years.max(1e4)) // Minimum 10,000 Jahre
    }

    /// Verbesserte Langzeit-Stabilitätsprüfung
    pub fn is_long_term_stable_enhanced(&self, components: &[StellarProperties]) -> bool {
        let stability = self.assess_million_year_stability(components);
        stability.is_million_year_stable() && !stability.hill_sphere_overlap
    }
}
