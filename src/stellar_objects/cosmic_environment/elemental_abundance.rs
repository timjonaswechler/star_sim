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
