// habitability.rs - Detaillierte Bewohnbarkeitsanalyse

use crate::cosmic_environment::*;
use crate::stellar_properties::*;
use crate::system_hierarchy::*;
use crate::units::*;
use serde::{Deserialize, Serialize};

/// Umfassendes Bewohnbarkeits-Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitabilityAssessment {
    /// Gesamter Bewohnbarkeitsfaktor (0.0-1.0)
    pub overall_habitability: f64,
    /// Bewohnbare Zone des Systems
    pub system_habitable_zone: HabitableZone,
    /// Strahlungsrisiken für Leben
    pub radiation_risks: RadiationRisks,
    /// Detaillierte Bewohnbarkeitsbedingungen
    pub habitability_conditions: Vec<String>,
    /// Planetare Bewohnbarkeitsanalyse
    pub planetary_analysis: Vec<PlanetaryHabitability>,
    /// Zeitliche Entwicklung der Bewohnbarkeit
    pub temporal_evolution: TemporalHabitability,
    /// Risikofaktoren
    pub risk_factors: Vec<RiskFactor>,
}

/// Strahlungsrisiken für Leben
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiationRisks {
    /// UV/XUV Strahlung während Pre-MS Phase
    pub pre_main_sequence_hazard: f64,
    /// Flare-Aktivität Risiko
    pub stellar_flare_risk: f64,
    /// Galaktische Strahlungsrisiken
    pub galactic_radiation_risk: f64,
    /// Röntgenstrahlung vom Stern
    pub x_ray_flux: f64,
    /// Kosmische Strahlung
    pub cosmic_ray_flux: f64,
}

/// Planetare Bewohnbarkeitsanalyse für spezifische Orbits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryHabitability {
    /// Orbitalentfernung des Planeten
    pub orbital_distance: Distance,
    /// Bewohnbarkeitsscore für diese Position (0.0-1.0)
    pub habitability_score: f64,
    /// Tidal Locking Analyse
    pub tidal_locking: TidalLockingAnalysis,
    /// Temperaturbereiche
    pub temperature_analysis: TemperatureAnalysis,
    /// Atmosphärische Überlegungen
    pub atmospheric_considerations: Vec<String>,
    /// Mögliche Bewohnbarkeitszonen (Tag/Nacht-Seite, etc.)
    pub habitable_regions: Vec<HabitableRegion>,
}

/// Temperaturanalyse für einen Planeten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureAnalysis {
    /// Gleichgewichtstemperatur (ohne Atmosphäre, K)
    pub equilibrium_temperature: f64,
    /// Temperaturbereich mit dünner Atmosphäre (K)
    pub thin_atmosphere_range: (f64, f64),
    /// Temperaturbereich mit dichter Atmosphäre (K)
    pub thick_atmosphere_range: (f64, f64),
    /// Greenhouse-Effekt Potenzial
    pub greenhouse_potential: f64,
}

/// Bewohnbare Regionen auf einem Planeten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HabitableRegion {
    /// Gesamte Oberfläche bewohnbar
    Global,
    /// Nur Tag-Seite bewohnbar (tidal locked)
    DaySide,
    /// Nur Terminator-Zone bewohnbar
    TerminatorZone,
    /// Polare Regionen bewohnbar
    PolarRegions,
    /// Äquatoriale Regionen bewohnbar
    EquatorialRegions,
    /// Keine bewohnbaren Regionen
    None,
}

/// Zeitliche Entwicklung der Bewohnbarkeit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalHabitability {
    /// Bewohnbarkeit in der Vergangenheit (Gyr ago -> habitability)
    pub past_habitability: Vec<(f64, f64)>,
    /// Aktuelle Bewohnbarkeit
    pub current_habitability: f64,
    /// Zukünftige Bewohnbarkeit (Gyr from now -> habitability)
    pub future_habitability: Vec<(f64, f64)>,
    /// Gesamte bewohnbare Lebensdauer (Gyr)
    pub total_habitable_lifetime: f64,
    /// Bewohnbarkeitsfenster (Start, Ende in Gyr)
    pub habitability_window: (f64, f64),
}

/// Risikofaktoren für Bewohnbarkeit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Name des Risikofaktors
    pub name: String,
    /// Schweregrad (0.0-1.0)
    pub severity: f64,
    /// Wahrscheinlichkeit des Auftretens (0.0-1.0)
    pub probability: f64,
    /// Zeitskala des Risikos
    pub timescale: Time,
    /// Beschreibung der Auswirkungen
    pub impact_description: String,
}

impl HabitabilityAssessment {
    /// Erstellt ein vollständiges Bewohnbarkeits-Assessment
    pub fn comprehensive_analysis(
        system_type: &SystemType,
        radiation_env: &CosmicRadiationEnvironment,
        target_distances: &[Distance],
    ) -> Self {
        match system_type {
            SystemType::Single(star) => {
                Self::analyze_single_star(star, radiation_env, target_distances)
            }
            SystemType::Binary {
                primary,
                secondary,
                orbital_properties,
            } => Self::analyze_binary_system(
                primary,
                secondary,
                orbital_properties,
                radiation_env,
                target_distances,
            ),
            SystemType::Multiple {
                components,
                hierarchy,
            } => Self::analyze_multiple_system(
                components,
                hierarchy,
                radiation_env,
                target_distances,
            ),
        }
    }

    fn analyze_single_star(
        star: &StellarProperties,
        radiation_env: &CosmicRadiationEnvironment,
        target_distances: &[Distance],
    ) -> Self {
        let system_habitable_zone = star.calculate_habitable_zone();
        let radiation_risks = Self::calculate_radiation_risks(star, radiation_env);

        // Planetare Analyse für alle Zieldistanzen
        let planetary_analysis = target_distances
            .iter()
            .map(|distance| Self::analyze_planetary_position(star, distance, &radiation_risks))
            .collect();

        // Zeitliche Entwicklung
        let temporal_evolution = Self::calculate_temporal_evolution(star);

        // Risikofaktoren
        let risk_factors = Self::identify_risk_factors(star, radiation_env);

        // Bewohnbarkeitsbedingungen
        let habitability_conditions = Self::generate_habitability_conditions(star);

        // Gesamtbewohnbarkeit
        let overall_habitability =
            Self::calculate_overall_habitability(star, &radiation_risks, &temporal_evolution);

        Self {
            overall_habitability,
            system_habitable_zone,
            radiation_risks,
            habitability_conditions,
            planetary_analysis,
            temporal_evolution,
            risk_factors,
        }
    }

    fn analyze_binary_system(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        orbital_properties: &crate::system_hierarchy::BinaryOrbit,
        radiation_env: &CosmicRadiationEnvironment,
        target_distances: &[Distance],
    ) -> Self {
        // Vereinfachte Binäranalyse - kombiniert beide Sterne
        let _combined_luminosity = primary.luminosity + secondary.luminosity;
        let dominant_star = if primary.luminosity > secondary.luminosity {
            primary
        } else {
            secondary
        };

        let system_habitable_zone = orbital_properties.combined_habitable_zone(primary, secondary);
        let radiation_risks = Self::calculate_radiation_risks(dominant_star, radiation_env);

        // Angepasste planetare Analyse für Binärsysteme
        let planetary_analysis = target_distances
            .iter()
            .map(|distance| {
                let mut analysis =
                    Self::analyze_planetary_position(dominant_star, distance, &radiation_risks);

                // Binär-spezifische Modifikationen
                analysis.habitability_score *= 0.8; // Binäre sind komplizierter
                analysis
                    .atmospheric_considerations
                    .push("Complex irradiation from dual stars".to_string());
                analysis
                    .atmospheric_considerations
                    .push("Potential for complex seasonal cycles".to_string());

                // S-Type vs P-Type Orbits
                if orbital_properties.s_type_primary_possible(distance) {
                    analysis
                        .atmospheric_considerations
                        .push("S-Type orbit around primary star".to_string());
                } else if orbital_properties.s_type_secondary_possible(distance) {
                    analysis
                        .atmospheric_considerations
                        .push("S-Type orbit around secondary star".to_string());
                } else if orbital_properties.p_type_possible(distance) {
                    analysis
                        .atmospheric_considerations
                        .push("P-Type circumbinary orbit".to_string());
                } else {
                    analysis.habitability_score *= 0.1; // Instabiler Orbit
                    analysis
                        .atmospheric_considerations
                        .push("Unstable orbital region".to_string());
                }

                analysis
            })
            .collect();

        let temporal_evolution = Self::calculate_temporal_evolution(dominant_star);
        let mut risk_factors = Self::identify_risk_factors(dominant_star, radiation_env);

        // Binär-spezifische Risiken
        risk_factors.push(RiskFactor {
            name: "Orbital instability".to_string(),
            severity: 0.3,
            probability: 0.1,
            timescale: Time::years(1e8),
            impact_description: "Close stellar encounters may destabilize planetary orbits"
                .to_string(),
        });

        let habitability_conditions = vec![
            "Dual star system with complex dynamics".to_string(),
            "Multiple potential orbital configurations".to_string(),
            "Enhanced stellar activity during close approaches".to_string(),
        ];

        let base_habitability = Self::calculate_overall_habitability(
            dominant_star,
            &radiation_risks,
            &temporal_evolution,
        );
        let overall_habitability = base_habitability * 0.85; // Binär-Penalty

        Self {
            overall_habitability,
            system_habitable_zone,
            radiation_risks,
            habitability_conditions,
            planetary_analysis,
            temporal_evolution,
            risk_factors,
        }
    }

    fn analyze_multiple_system(
        components: &[StellarProperties],
        hierarchy: &crate::system_hierarchy::SystemHierarchy,
        radiation_env: &CosmicRadiationEnvironment,
        target_distances: &[Distance],
    ) -> Self {
        // Vereinfachte Multiple-System Analyse
        let dominant_star = components
            .iter()
            .max_by(|a, b| a.luminosity.partial_cmp(&b.luminosity).unwrap())
            .unwrap();

        let total_luminosity: f64 = components.iter().map(|s| s.luminosity).sum();
        let system_habitable_zone = HabitableZone {
            inner_edge: Distance::new(0.95 * total_luminosity.sqrt(), dominant_star.unit_system),
            outer_edge: Distance::new(1.37 * total_luminosity.sqrt(), dominant_star.unit_system),
            optimistic_inner: Distance::new(
                0.84 * total_luminosity.sqrt(),
                dominant_star.unit_system,
            ),
            optimistic_outer: Distance::new(
                1.67 * total_luminosity.sqrt(),
                dominant_star.unit_system,
            ),
        };

        let radiation_risks = Self::calculate_radiation_risks(dominant_star, radiation_env);

        let planetary_analysis = target_distances
            .iter()
            .map(|distance| {
                let mut analysis =
                    Self::analyze_planetary_position(dominant_star, distance, &radiation_risks);
                analysis.habitability_score *= hierarchy.stability_factor * 0.6; // Multiple-System Penalty
                analysis.atmospheric_considerations.push(format!(
                    "Complex {}-body gravitational dynamics",
                    components.len()
                ));
                analysis
            })
            .collect();

        let temporal_evolution = Self::calculate_temporal_evolution(dominant_star);
        let mut risk_factors = Self::identify_risk_factors(dominant_star, radiation_env);

        // Multiple-System Risiken
        risk_factors.push(RiskFactor {
            name: "Chaotic evolution".to_string(),
            severity: 1.0 - hierarchy.stability_factor,
            probability: 0.8,
            timescale: hierarchy.chaos_timescale.clone(),
            impact_description: "N-body chaos may lead to stellar ejection or collision"
                .to_string(),
        });

        let habitability_conditions = vec![
            format!("Multiple star system with {} components", components.len()),
            format!(
                "System stability factor: {:.1}%",
                hierarchy.stability_factor * 100.0
            ),
            "Highly complex orbital mechanics".to_string(),
        ];

        let base_habitability = Self::calculate_overall_habitability(
            dominant_star,
            &radiation_risks,
            &temporal_evolution,
        );
        let overall_habitability = base_habitability * hierarchy.stability_factor * 0.5;

        Self {
            overall_habitability,
            system_habitable_zone,
            radiation_risks,
            habitability_conditions,
            planetary_analysis,
            temporal_evolution,
            risk_factors,
        }
    }

    fn calculate_radiation_risks(
        star: &StellarProperties,
        radiation_env: &CosmicRadiationEnvironment,
    ) -> RadiationRisks {
        let pre_main_sequence_hazard = match &star.evolutionary_stage {
            crate::stellar_properties::EvolutionaryStage::PreMainSequence { .. } => 0.9,
            _ => 0.1,
        };

        let stellar_flare_risk = match star.mass.in_solar_masses() {
            m if m < 0.3 => 0.9, // M-Zwerge haben extreme Flares
            m if m < 0.6 => 0.5, // Moderate Flare-Aktivität
            _ => 0.1,            // Geringe Flare-Aktivität
        };

        let x_ray_flux = match star.mass.in_solar_masses() {
            m if m < 0.5 => 10.0, // M-Zwerge haben hohe X-Ray Emission
            m if m < 1.5 => 1.0,  // Sonnenähnlich
            _ => 0.1,             // Wenig X-Ray
        };

        let cosmic_ray_flux = radiation_env.grb_risk + radiation_env.supernova_frequency * 0.5;

        RadiationRisks {
            pre_main_sequence_hazard,
            stellar_flare_risk,
            galactic_radiation_risk: (radiation_env.agn_risk
                + radiation_env.supernova_frequency
                + radiation_env.grb_risk)
                / 3.0,
            x_ray_flux,
            cosmic_ray_flux,
        }
    }

    fn analyze_planetary_position(
        star: &StellarProperties,
        distance: &Distance,
        radiation_risks: &RadiationRisks,
    ) -> PlanetaryHabitability {
        let tidal_locking = star.analyze_tidal_locking(distance);
        let temperature_analysis = Self::calculate_temperature_analysis(star, distance);

        // Bewohnbarkeitsscore basierend auf verschiedenen Faktoren
        let mut habitability_score = 1.0;

        // Temperatur-Faktor
        let temp_factor = if temperature_analysis.equilibrium_temperature >= 273.0 // 0°C
            && temperature_analysis.equilibrium_temperature <= 323.0
        // 50°C (engerer Optimalbereich)
        {
            1.0 // Flüssiges Wasser möglich
        } else if temperature_analysis.thick_atmosphere_range.0 <= 373.0
            && temperature_analysis.thick_atmosphere_range.1 >= 273.0
        {
            0.8 // Mit Atmosphäre möglich
        } else {
            0.1 // Schwierig
        };
        habitability_score *= temp_factor;

        // Tidal Locking Faktor
        let tidal_factor = if tidal_locking.tidal_lock_probability > 0.8 {
            0.4 // Schwierig aber möglich
        } else if tidal_locking.tidal_lock_probability > 0.3 {
            0.7 // Resonanzen möglich
        } else {
            1.0 // Freie Rotation
        };
        habitability_score *= tidal_factor;

        // Strahlungsfaktor
        let radiation_factor = 1.0
            - (radiation_risks.stellar_flare_risk * 0.3 + radiation_risks.x_ray_flux * 0.2 / 10.0);
        habitability_score *= radiation_factor.max(0.1);

        // Bewohnbare Regionen bestimmen
        let habitable_regions = if tidal_locking.tidal_lock_probability > 0.8 {
            if temperature_analysis.equilibrium_temperature > 273.0 {
                vec![HabitableRegion::TerminatorZone, HabitableRegion::DaySide]
            } else {
                vec![HabitableRegion::TerminatorZone]
            }
        } else if temperature_analysis.equilibrium_temperature >= 200.0
            && temperature_analysis.equilibrium_temperature <= 400.0
        {
            vec![HabitableRegion::Global]
        } else if temperature_analysis.equilibrium_temperature > 400.0 {
            vec![HabitableRegion::PolarRegions]
        } else {
            vec![HabitableRegion::EquatorialRegions]
        };

        // Atmosphärische Überlegungen
        let mut atmospheric_considerations = Vec::new();

        if tidal_locking.tidal_lock_probability > 0.5 {
            atmospheric_considerations
                .push("Thick atmosphere required for heat redistribution".to_string());
            atmospheric_considerations.push("Day-night temperature extremes".to_string());
        }

        if radiation_risks.stellar_flare_risk > 0.5 {
            atmospheric_considerations
                .push("Strong magnetic field needed for radiation protection".to_string());
            atmospheric_considerations.push("Atmospheric loss from stellar winds".to_string());
        }

        if distance.in_au() < 0.1 {
            atmospheric_considerations.push("Extreme tidal heating possible".to_string());
        }

        PlanetaryHabitability {
            orbital_distance: distance.clone(),
            habitability_score: habitability_score.max(0.0).min(1.0),
            tidal_locking,
            temperature_analysis,
            atmospheric_considerations,
            habitable_regions,
        }
    }

    fn calculate_temperature_analysis(
        star: &StellarProperties,
        distance: &Distance,
    ) -> TemperatureAnalysis {
        // Stefan-Boltzmann Gesetz für Planetentemperatur
        // T = (L☉ * (1-A) / (16π * σ * d²))^0.25
        let luminosity_watts = star.luminosity * crate::constants::SOLAR_LUMINOSITY;
        let distance_m = distance.in_meters();
        let albedo = 0.3; // Erdähnliche Albedo

        let equilibrium_temperature = (luminosity_watts * (1.0 - albedo)
            / (16.0
                * crate::constants::PI
                * crate::constants::STEFAN_BOLTZMANN
                * distance_m
                * distance_m))
            .powf(0.25);

        // Atmosphäreneffekte (vereinfacht)
        let greenhouse_potential =
            if equilibrium_temperature > 200.0 && equilibrium_temperature < 350.0 {
                0.5 // Moderate Greenhouse-Erwärmung möglich
            } else {
                0.2 // Wenig Greenhouse-Effekt
            };

        let thin_atmosphere_range = (equilibrium_temperature * 0.9, equilibrium_temperature * 1.1);

        let thick_atmosphere_range = (
            equilibrium_temperature * (1.0 + greenhouse_potential * 0.5),
            equilibrium_temperature * (1.0 + greenhouse_potential),
        );

        TemperatureAnalysis {
            equilibrium_temperature,
            thin_atmosphere_range,
            thick_atmosphere_range,
            greenhouse_potential,
        }
    }

    fn calculate_temporal_evolution(star: &StellarProperties) -> TemporalHabitability {
        let current_age = star.age.in_years();
        let ms_lifetime = star.main_sequence_lifetime.in_years();

        // Vereinfachte zeitliche Entwicklung
        let mut past_habitability = Vec::new();
        let mut future_habitability = Vec::new();

        // Vergangenheit
        for age_gyr in [0.5, 1.0, 2.0, 3.0, 4.0].iter() {
            if *age_gyr < current_age {
                let habitability = if *age_gyr < 0.5 {
                    0.1 // Sehr früh, instabil
                } else if *age_gyr < 1.0 {
                    0.5 // Stabilisierung
                } else {
                    0.9 // Hauptreihe
                };
                past_habitability.push((*age_gyr, habitability));
            }
        }

        // Zukunft
        let remaining_ms_time = ms_lifetime - current_age;
        if remaining_ms_time > 0.0 {
            let steps = [0.5, 1.0, 2.0, 5.0, 10.0];
            for &step_gyr in steps.iter() {
                if step_gyr < remaining_ms_time {
                    let habitability = if step_gyr < remaining_ms_time * 0.8 {
                        0.9 // Stabile Hauptreihe
                    } else {
                        0.5 // Späte Hauptreihe
                    };
                    future_habitability.push((step_gyr, habitability));
                } else {
                    future_habitability.push((step_gyr, 0.1)); // Post-Hauptreihe
                }
            }
        }

        let current_habitability = if current_age < ms_lifetime * 0.9 {
            0.9
        } else {
            0.3
        };
        let total_habitable_lifetime = ms_lifetime * 0.8; // ~80% der Hauptreihen-Zeit
        let habitability_window = (0.5, ms_lifetime * 0.9);

        TemporalHabitability {
            past_habitability,
            current_habitability,
            future_habitability,
            total_habitable_lifetime,
            habitability_window,
        }
    }

    fn identify_risk_factors(
        star: &StellarProperties,
        radiation_env: &CosmicRadiationEnvironment,
    ) -> Vec<RiskFactor> {
        let mut risk_factors = Vec::new();

        // Stellar Evolution Risk
        if star.age.in_years() > star.main_sequence_lifetime.in_years() * 0.8 {
            risk_factors.push(RiskFactor {
                name: "Stellar evolution".to_string(),
                severity: 0.8,
                probability: 1.0,
                timescale: Time::years(star.main_sequence_lifetime.in_years() * 0.1),
                impact_description: "Star will leave main sequence, altering habitable zone"
                    .to_string(),
            });
        }

        // Flare Risk
        if star.mass.in_solar_masses() < 0.5 {
            risk_factors.push(RiskFactor {
                name: "Stellar flares".to_string(),
                severity: 0.6,
                probability: 0.9,
                timescale: Time::years(1e6),
                impact_description: "Frequent superflares may erode planetary atmospheres"
                    .to_string(),
            });
        }

        // Galactic Risks
        if radiation_env.supernova_frequency > 0.5 {
            risk_factors.push(RiskFactor {
                name: "Nearby supernovae".to_string(),
                severity: radiation_env.supernova_frequency,
                probability: radiation_env.supernova_frequency,
                timescale: Time::years(1e8),
                impact_description: "Nearby supernovae may sterilize planetary surfaces"
                    .to_string(),
            });
        }

        if radiation_env.grb_risk > 0.4 {
            risk_factors.push(RiskFactor {
                name: "Gamma-ray bursts".to_string(),
                severity: radiation_env.grb_risk,
                probability: radiation_env.grb_risk * 0.1,
                timescale: Time::years(1e9),
                impact_description: "GRB could destroy ozone layer and sterilize planet"
                    .to_string(),
            });
        }

        risk_factors
    }

    fn generate_habitability_conditions(star: &StellarProperties) -> Vec<String> {
        let mut conditions = Vec::new();

        match star.mass.in_solar_masses() {
            m if m < 0.3 => {
                conditions.push("Red dwarf system - long-lived but challenging".to_string());
                conditions.push("Tidal locking likely for close planets".to_string());
                conditions.push("Flare activity may impact atmospheres".to_string());
                conditions.push("Very long habitable periods possible (>10 Gyr)".to_string());
            }
            m if m < 0.8 => {
                conditions.push("Orange dwarf system - optimal for habitability".to_string());
                conditions.push("Stable radiation environment".to_string());
                conditions.push("Long-lived main sequence (>10 Gyr)".to_string());
                conditions.push("Moderate tidal effects".to_string());
            }
            m if m < 1.5 => {
                conditions.push("Sun-like system with Earth-analogous conditions".to_string());
                conditions.push("Stable main sequence evolution".to_string());
                conditions.push("Moderate habitable periods (1-10 Gyr)".to_string());
            }
            _ => {
                conditions.push("Massive star with short lifetime".to_string());
                conditions.push("High UV radiation levels".to_string());
                conditions.push("Complex life unlikely due to time constraints".to_string());
            }
        }

        conditions
    }

    fn calculate_overall_habitability(
        star: &StellarProperties,
        radiation_risks: &RadiationRisks,
        temporal_evolution: &TemporalHabitability,
    ) -> f64 {
        let mut habitability = 1.0;

        // Stellar type factor
        habitability *= match star.mass.in_solar_masses() {
            m if m < 0.08 => 0.0, // Brown dwarfs
            m if m < 0.3 => 0.6,  // M dwarfs
            m if m < 0.8 => 1.0,  // K dwarfs (optimal)
            m if m < 1.4 => 0.9,  // G dwarfs
            m if m < 2.0 => 0.5,  // F dwarfs
            _ => 0.1,             // Too massive
        };

        // Evolutionary stage
        habitability *= match &star.evolutionary_stage {
            crate::stellar_properties::EvolutionaryStage::MainSequence { .. } => 1.0,
            crate::stellar_properties::EvolutionaryStage::PreMainSequence { .. } => 0.3,
            _ => 0.1,
        };

        // Radiation risks
        habitability *= 1.0 - radiation_risks.stellar_flare_risk * 0.4;
        habitability *= 1.0 - radiation_risks.galactic_radiation_risk * 0.3;
        habitability *= 1.0 - radiation_risks.x_ray_flux * 0.1 / 10.0;

        // Temporal factor
        let temporal_factor = temporal_evolution.total_habitable_lifetime / 10.0; // Normalize to 10 Gyr
        habitability *= temporal_factor.min(1.0);

        habitability.max(0.0).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_habitability_assessment() {
        let sun = StellarProperties::sun_like();
        let radiation_env = crate::cosmic_environment::CosmicRadiationEnvironment {
            agn_risk: 0.1,
            supernova_frequency: 0.2,
            grb_risk: 0.3,
            stellar_encounter_rate: 0.1,
            cosmic_ray_flux: 10.0,            // Hinzugefügt
            uv_background: 1.0,               // Hinzugefügt
            gravitational_wave_activity: 0.1, // Hinzugefügt
        };

        let target_distances = vec![Distance::au(0.5), Distance::au(1.0), Distance::au(1.5)];

        let system_type = SystemType::Single(sun);
        let assessment = HabitabilityAssessment::comprehensive_analysis(
            &system_type,
            &radiation_env,
            &target_distances,
        );

        assert!(assessment.overall_habitability > 0.5);
        assert_eq!(assessment.planetary_analysis.len(), 3);
        assert!(assessment.temporal_evolution.total_habitable_lifetime > 0.0);
    }

    #[test]
    fn test_temperature_analysis() {
        let sun = StellarProperties::sun_like();
        let earth_distance = Distance::au(1.0);

        let temp_analysis =
            HabitabilityAssessment::calculate_temperature_analysis(&sun, &earth_distance);

        // Should be roughly Earth-like temperatures
        assert!(temp_analysis.equilibrium_temperature > 250.0);
        assert!(temp_analysis.equilibrium_temperature < 300.0);
        assert!(temp_analysis.greenhouse_potential > 0.0);
    }

    #[test]
    fn test_risk_factors() {
        let m_dwarf = StellarProperties::new(Mass::solar_masses(0.3), Time::years(5.0), 0.0);

        let high_risk_env = crate::cosmic_environment::CosmicRadiationEnvironment {
            // Verwende den direkten Pfad für Klarheit
            agn_risk: 0.8,
            supernova_frequency: 0.7,
            grb_risk: 0.6,
            stellar_encounter_rate: 0.5,
            cosmic_ray_flux: 50.0,            // Hinzugefügt
            uv_background: 5.0,               // Hinzugefügt
            gravitational_wave_activity: 0.5, // Hinzugefügt
        };

        let risk_factors = HabitabilityAssessment::identify_risk_factors(&m_dwarf, &high_risk_env);

        assert!(!risk_factors.is_empty());
        assert!(risk_factors.iter().any(|r| r.name.contains("flare")));
        assert!(risk_factors.iter().any(|r| r.name.contains("supernova")));
    }
}
