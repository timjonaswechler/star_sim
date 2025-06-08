use crate::physics::units::Time;

/// Kosmische Epochen basierend auf der Entstehung und dem Schicksal des Universums
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CosmicEra {
    /// Big Bang bis Rekombination (0 - 380.000 Jahre)
    Recombination,
    /// Dunkles Zeitalter bis erste Sterne (380.000 - 180 Millionen Jahre)
    DarkAge,
    /// Erste Sterne und Reionisation (180 Millionen - 1 Milliarde Jahre)
    FirstStars,
    /// Galaxienbildung und frühe Metallizität (1 - 3 Milliarden Jahre)
    GalaxyFormation,
    /// Hochphase der Sternbildung (3 - 10 Milliarden Jahre)
    StellarPeak,
    /// Moderne Ära - stabile Verhältnisse (10 - 100 Milliarden Jahre)
    Modern,
    /// Stelliferous Era - langsamer Rückgang (100 Milliarden - 100 Billionen Jahre)
    Stelliferous,
    /// Degenerate Era - braune Zwerge und Weiße Zwerge (10^14 - 10^30 Jahre)
    Degenerate,
    /// Black Hole Era - nur noch schwarze Löcher (10^30 - 10^100 Jahre)
    BlackHole,
}

/// Zeitrechnung seit dem Big Bang in Jahren
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CosmicTime {
    pub years_since_big_bang: Time,
}

impl CosmicTime {
    pub fn new(years: f64) -> Self {
        Self {
            years_since_big_bang: Time::from_years(years),
        }
    }

    /// Bestimmt die kosmische Epoche basierend auf der Zeit
    pub fn era(&self) -> CosmicEra {
        match self.years_since_big_bang {
            t if t.value < 380_000.0 => CosmicEra::Recombination,
            t if t.value < 180_000_000.0 => CosmicEra::DarkAge,
            t if t.value < 1_000_000_000.0 => CosmicEra::FirstStars,
            t if t.value < 3_000_000_000.0 => CosmicEra::GalaxyFormation,
            t if t.value < 10_000_000_000.0 => CosmicEra::StellarPeak,
            t if t.value < 100_000_000_000.0 => CosmicEra::Modern,
            t if t.value < 1e14 => CosmicEra::Stelliferous,
            t if t.value < 1e30 => CosmicEra::Degenerate,
            _ => CosmicEra::BlackHole,
        }
    }

    /// Kosmische Mikrowellen-Hintergrundtemperatur bei dieser Zeit
    pub fn cmb_temperature(&self) -> f64 {
        // T(t) = 2.7 K * (13.8 Gyr / t)
        2.7 * (13.8e9 / self.years_since_big_bang.value)
    }

    /// Bewohnbare Epoche durch CMB (273-373 K für Wasser)
    pub fn is_cmb_habitable_epoch(&self) -> bool {
        let temp = self.cmb_temperature();
        temp >= 273.0 && temp <= 373.0
    }
}
