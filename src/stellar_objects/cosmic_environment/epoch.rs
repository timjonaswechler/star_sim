use serde::{Deserialize, Serialize};

/// Kosmische Epoche und Zeitrahmen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicEpoch {
    /// Alter des Universums in Milliarden Jahren
    pub age_universe: f64,
    /// Kosmische Ära (String-Beschreibung)
    pub era: String,
    /// Sternentstehungsrate relativ zu heute
    pub star_formation_rate: f64,
    /// Durchschnittliche Metallizität in dieser Epoche
    pub epoch_metallicity: f64,
    /// Rotverschiebung (z)
    pub redshift: f64,
    /// Hubble-Parameter H(z)
    pub hubble_parameter: f64,
}

impl CosmicEpoch {
    /// Erstellt eine kosmische Epoche für gegebenes Universumsalter
    pub fn from_age(age_gyr: f64) -> Self {
        let era = match age_gyr {
            age if age < 0.5 => "Primordial Era".to_string(),
            age if age < 2.0 => "Early Universe".to_string(),
            age if age < 6.0 => "Peak Star Formation".to_string(),
            age if age < 10.0 => "Stellar Era".to_string(),
            age if age < 13.0 => "Mature Universe".to_string(),
            _ => "Late Universe".to_string(),
        };

        // Vereinfachte Sternentstehungsrate (relativ zu heute)
        let star_formation_rate = match age_gyr {
            age if age < 1.0 => 0.1,
            age if age < 3.0 => 10.0, // Peak bei z~2-3
            age if age < 8.0 => 3.0,
            age if age < 11.0 => 1.0,
            _ => 0.3,
        };

        // Metallizität entwickelt sich mit der Zeit
        let epoch_metallicity = match age_gyr {
            age if age < 0.5 => -3.0, // Sehr metall-arm
            age if age < 2.0 => -1.5,
            age if age < 6.0 => -0.5,
            age if age < 10.0 => 0.0,
            _ => 0.2, // Leicht metall-reich
        };

        // Redshift approximation
        let redshift = ((13.8 / age_gyr) - 1.0).max(0.0);

        // Hubble parameter (vereinfacht)
        let hubble_parameter = 70.0 * (1.0 + redshift).sqrt(); // km/s/Mpc

        Self {
            age_universe: age_gyr,
            era,
            star_formation_rate,
            epoch_metallicity,
            redshift,
            hubble_parameter,
        }
    }

    /// Ist das Universum alt genug für komplexe Chemie?
    pub fn allows_complex_chemistry(&self) -> bool {
        self.age_universe > 1.0 && self.epoch_metallicity > -2.0
    }

    /// Ist das Universum alt genug für langlebige Sterne?
    pub fn allows_long_lived_stars(&self) -> bool {
        self.age_universe > 0.5
    }
}
