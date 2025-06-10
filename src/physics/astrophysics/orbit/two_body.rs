use crate::physics::astrophysics::lagrange_points::LagrangeSystem;
use crate::physics::astrophysics::orbit::elements::{EscapeVelocity, OrbitalElements};
use crate::physics::constants::MIN_LAGRANGE_MASS_RATIO;
use crate::physics::units::{Distance, GenericUnitValue, Mass, UnitSystem};
use crate::stellar_objects::stars::properties::StellarProperties;
use serde::{Deserialize, Serialize};

/// Erweiterte Binärbahnparameter mit vollständigen orbitalen Elementen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOrbit {
    /// Vollständige orbitale Elemente
    pub orbital_elements: OrbitalElements,
    /// Barycenter Position (Bruchteil vom Primärstern)
    pub barycenter_position: f64,
    /// S-Type Stabilitätsgrenze für Planeten (primary, secondary)
    pub s_type_stability: (Distance, Distance),
    /// P-Type Stabilitätsgrenze für Planeten
    pub p_type_stability: Distance,
    /// Lagrange-System für das Binärsystem
    pub lagrange_system: Option<LagrangeSystem>,
    /// Gegenseitige Hill-Sphären
    pub mutual_hill_sphere: Distance,
}

impl BinaryOrbit {
    /// Erstellt neue Binärbahn mit vollständigen orbitalen Elementen
    pub fn new(
        primary: &StellarProperties,
        secondary: &StellarProperties,
        separation: Distance,
        eccentricity: f64,
        inclination: f64,
        longitude_of_ascending_node: f64,
        argument_of_periapsis: f64,
    ) -> Self {
        let total_mass = Mass::kilograms(primary.mass.in_kg() + secondary.mass.in_kg());

        let orbital_elements = OrbitalElements::new(
            separation.clone(),
            eccentricity,
            inclination,
            longitude_of_ascending_node,
            argument_of_periapsis,
            0.0,
        );

        let barycenter_position = secondary.mass.in_kg() / total_mass.in_kg();

        let mu_primary = secondary.mass.in_kg() / total_mass.in_kg();
        let mu_secondary = primary.mass.in_kg() / total_mass.in_kg();

        let s_type_primary_limit =
            separation.value * (0.464 - 0.380 * mu_primary - 0.631 * eccentricity);
        let s_type_secondary_limit =
            separation.value * (0.464 - 0.380 * mu_secondary - 0.631 * eccentricity);

        let s_type_stability = (
            Distance::new(s_type_primary_limit.max(0.0), separation.system),
            Distance::new(s_type_secondary_limit.max(0.0), separation.system),
        );

        let mu_min = primary.mass.in_kg().min(secondary.mass.in_kg()) / total_mass.in_kg();
        let p_type_limit = separation.value * (1.60 + 4.12 * mu_min + 4.27 * eccentricity);
        let p_type_stability = Distance::new(p_type_limit, separation.system);

        let mut lagrange_system_opt = if primary.mass.in_kg() / secondary.mass.in_kg()
            >= MIN_LAGRANGE_MASS_RATIO
            || secondary.mass.in_kg() / primary.mass.in_kg() >= MIN_LAGRANGE_MASS_RATIO
        {
            Some(LagrangeSystem::new(primary, secondary, separation.clone()))
        } else {
            None
        };

        if let Some(ref mut lag_sys) = lagrange_system_opt {
            if lag_sys.l4_l5_stable {
                // Versuche, einen kleinen Test-Trojaner zu generieren
                let trojan_mass_val = primary.mass.value_in_system_base() * 0.000001; // Sehr kleine Masse
                let trojan_mass = Mass::new(trojan_mass_val, primary.mass.units());
                match lag_sys.generate_trojan(4, trojan_mass, &primary.mass, &secondary.mass) {
                    Ok(trojan) => {
                        if lag_sys.add_trojan(trojan).is_ok() {
                            // Erfolgreich hinzugefügt (für Debugging)
                            // println!("Test trojan added to L4");
                        }
                    }
                    Err(_) => {} // Fehler ignorieren für diese Demo
                }
            }
        }
        let smaller_mass = if primary.mass.in_kg() < secondary.mass.in_kg() {
            &primary.mass
        } else {
            &secondary.mass
        };
        let mutual_hill_sphere = orbital_elements.hill_radius(smaller_mass, &total_mass);

        Self {
            orbital_elements,
            barycenter_position,
            s_type_stability,
            p_type_stability,
            lagrange_system: lagrange_system_opt,
            mutual_hill_sphere,
        }
    }

    pub fn distance_range(&self) -> (Distance, Distance) {
        let periapsis = self.orbital_elements.periapsis();
        let apoapsis = self.orbital_elements.apoapsis();
        (periapsis, apoapsis)
    }

    pub fn s_type_primary_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() < self.s_type_stability.0.in_meters()
    }

    pub fn s_type_secondary_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() < self.s_type_stability.1.in_meters()
    }

    pub fn p_type_possible(&self, planet_distance: &Distance) -> bool {
        planet_distance.in_meters() > self.p_type_stability.in_meters()
    }

    pub fn combined_habitable_zone(
        &self,
        primary: &StellarProperties,
        secondary: &StellarProperties,
    ) -> crate::stellar_objects::bodies::habitability::HabitableZone {
        let combined_luminosity = primary.luminosity + secondary.luminosity;
        let sqrt_l_combined = combined_luminosity.sqrt();

        crate::stellar_objects::bodies::habitability::HabitableZone {
            inner_edge: Distance::new(0.95 * sqrt_l_combined, self.orbital_elements.units),
            outer_edge: Distance::new(1.37 * sqrt_l_combined, self.orbital_elements.units),
            optimistic_inner: Distance::new(
                0.84 * sqrt_l_combined,
                self.orbital_elements.units,
            ),
            optimistic_outer: Distance::new(
                1.67 * sqrt_l_combined,
                self.orbital_elements.units,
            ),
        }
    }
}
