use serde::{Deserialize, Serialize};

use super::types::SurfaceType;

/// Oberflächenkomposition eines Planeten
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceComposition {
    /// Primärer Oberflächentyp (größte Abdeckung)
    pub primary_surface: SurfaceType,
    /// Sekundäre Oberflächentypen mit Abdeckungsanteil
    pub secondary_surfaces: Vec<(SurfaceType, f64)>,
    /// Durchschnittliche Albedo der Gesamtoberfläche
    pub average_albedo: f64,
    /// Oberflächentemperatur-Bereich (min, max) in K
    pub temperature_range: (f64, f64),
    /// Geologische Aktivität (0.0 = tot, 1.0 = sehr aktiv)
    pub geological_activity: f64,
}
