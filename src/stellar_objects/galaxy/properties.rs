use crate::stellar_objects::universe::{CosmicEra, CosmicTime};
/// Galaxientypen mit unterschiedlichen Eigenschaften für Bewohnbarkeit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GalaxyType {
    /// Spiralgalaxie (wie die Milchstraße)
    Spiral,
    /// Elliptische Galaxie
    Elliptical,
    /// Irreguläre Galaxie
    Irregular,
    /// Zwerggalaxie
    Dwarf,
}

/// Position innerhalb einer Galaxie
#[derive(Debug, Clone, Copy)]
pub struct GalacticPosition {
    /// Entfernung vom galaktischen Zentrum in Kiloparsec
    pub distance_from_center_kpc: f64,
    /// Azimutaler Winkel in Radianten
    pub azimuth: f64,
}

/// Eigenschaften einer Galaxie
#[derive(Debug, Clone)]
pub struct Galaxy {
    pub galaxy_type: GalaxyType,
    pub age_gyr: f64,
    pub mass_solar_masses: f64,
    pub metallicity: f64,         // Z/Z_sun (Metallizität relativ zur Sonne)
    pub star_formation_rate: f64, // M_sun/Jahr
    pub has_active_nucleus: bool,
}

impl Galaxy {
    /// Galaktisch bewohnbare Zone für diese Galaxie
    pub fn habitable_zone_kpc(&self) -> (f64, f64) {
        match self.galaxy_type {
            GalaxyType::Spiral => (4.0, 10.0), // Typische Werte für Spiralgalaxien
            GalaxyType::Elliptical => (0.0, 50.0), // Elliptische Galaxien sind generell sicherer
            GalaxyType::Irregular => (1.0, 5.0), // Kleinere, weniger stabile Zonen
            GalaxyType::Dwarf => (0.0, 2.0),   // Sehr kleine Zonen
        }
    }

    /// Berechnet die Bewohnbarkeit an einer bestimmten Position
    pub fn habitability_at_position(
        &self,
        position: &GalacticPosition,
        cosmic_time: &CosmicTime,
    ) -> f64 {
        let (inner, outer) = self.habitable_zone_kpc();
        let distance = position.distance_from_center_kpc;

        // Grundlegende Zonenbewertung
        let zone_factor = if distance >= inner && distance <= outer {
            1.0
        } else if distance < inner {
            // Zu nah am Zentrum - Strahlung
            (distance / inner).powf(2.0)
        } else {
            // Zu weit außen - niedrige Metallizität
            (outer / distance).powf(0.5)
        };

        // Metallizitätsfaktor
        let metallicity_factor = if self.metallicity > 0.1 {
            (self.metallicity / 0.1).min(1.0)
        } else {
            0.0
        };

        // AGN-Faktor
        let agn_factor = if self.has_active_nucleus && distance < 1.0 {
            0.1 // Stark reduzierte Bewohnbarkeit nahe AGN
        } else {
            1.0
        };

        // Zeitfaktor basierend auf kosmischer Epoche
        let era_factor = match cosmic_time.era() {
            CosmicEra::Recombination | CosmicEra::DarkAge => 0.0,
            CosmicEra::FirstStars => 0.1,
            CosmicEra::GalaxyFormation => 0.3,
            CosmicEra::StellarPeak => 0.8,
            CosmicEra::Modern => 1.0,
            CosmicEra::Stelliferous => 0.7,
            CosmicEra::Degenerate => 0.1,
            CosmicEra::BlackHole => 0.0,
        };

        zone_factor * metallicity_factor * agn_factor * era_factor
    }
}

/// Ein Strahlungsereignis im Raum-Zeit-Kontinuum
#[derive(Debug, Clone)]
pub struct RadiationEvent {
    pub source_type: RadiationSource,
    pub position: GalacticPosition,
    pub time: CosmicTime,
    pub intensity: f64, // Relative Intensität
    pub duration_years: f64,
    pub effective_range_kpc: f64,
}

impl RadiationEvent {
    /// Berechnet den Schaden an einer Position
    pub fn damage_at_position(&self, target: &GalacticPosition) -> f64 {
        let distance = self.distance_to(target);

        if distance > self.effective_range_kpc {
            return 0.0;
        }

        // Inverse-square law für Intensitätsabfall
        let intensity_factor = (self.effective_range_kpc / distance.max(0.1)).powf(2.0);

        // Basisfaktor basierend auf Quelltyp
        let base_damage = match self.source_type {
            RadiationSource::GammaRayBurst => 1.0,
            RadiationSource::Supernova => 0.7,
            RadiationSource::ActiveGalacticNucleus => 0.5,
            RadiationSource::TidalDisruptionEvent => 0.3,
        };

        (base_damage * self.intensity * intensity_factor).min(1.0)
    }

    fn distance_to(&self, target: &GalacticPosition) -> f64 {
        let dx = self.position.distance_from_center_kpc * self.position.azimuth.cos()
            - target.distance_from_center_kpc * target.azimuth.cos();
        let dy = self.position.distance_from_center_kpc * self.position.azimuth.sin()
            - target.distance_from_center_kpc * target.azimuth.sin();
        (dx * dx + dy * dy).sqrt()
    }
}

/// Typen kosmischer Strahlungsquellen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadiationSource {
    /// Gammastrahlenausbruch
    GammaRayBurst,
    /// Supernova
    Supernova,
    /// Aktiver galaktischer Kern
    ActiveGalacticNucleus,
    /// Gezeitenstörungsereignis
    TidalDisruptionEvent,
}
