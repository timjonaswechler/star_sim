use super::{CosmicTime, UniverseBuilder};
use crate::stellar_objects::galaxy::properties::*;

#[derive(Debug, Clone)]
pub struct Universe {
    pub cosmic_time: CosmicTime,
    pub galaxy: Galaxy,
    pub radiation_history: Vec<RadiationEvent>,
    pub seed: u64,
}

impl Universe {
    /// Erstellt einen neuen Universe Builder
    pub fn builder() -> UniverseBuilder {
        UniverseBuilder::new()
    }

    /// Bewertet die Bewohnbarkeit an einer galaktischen Position
    pub fn evaluate_habitability(&self, position: &GalacticPosition) -> f64 {
        let base_habitability = self
            .galaxy
            .habitability_at_position(position, &self.cosmic_time);

        // Reduziere Bewohnbarkeit basierend auf Strahlungsgeschichte
        let radiation_factor = self
            .radiation_history
            .iter()
            .map(|event| {
                let damage = event.damage_at_position(position);
                1.0 - damage * 0.5 // Strahlung reduziert Bewohnbarkeit um bis zu 50%
            })
            .product::<f64>();

        base_habitability * radiation_factor
    }

    /// Fügt ein Strahlungsereignis zur Geschichte hinzu
    pub fn add_radiation_event(&mut self, event: RadiationEvent) {
        self.radiation_history.push(event);
    }

    /// Simuliert die Entwicklung des Universums über eine Zeitspanne
    pub fn evolve(&mut self, time_step_years: f64) {
        self.cosmic_time.years_since_big_bang.value += time_step_years;

        // Hier könnten weitere Evolutionsmechanismen hinzugefügt werden
        // z.B. Galaxienentwicklung, neue Strahlungsereignisse, etc.
    }
}
