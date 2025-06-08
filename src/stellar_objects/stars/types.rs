use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpectralType {
    // Hauptreihe O, B, A, F, G, K, M mit Subklassen 0-9
    O(u8),
    B(u8),
    A(u8),
    F(u8),
    G(u8),
    K(u8),
    M(u8),
    // Braune Zwerge
    L(u8),
    T(u8),
    Y(u8),
    // Post-Main Sequence
    WolfRayet,  // W-type
    WhiteDwarf, // D-type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LuminosityClass {
    /// Hypergiants
    Zero,
    /// Bright supergiants
    Ia,
    /// Supergiants
    Ib,
    /// Bright giants
    II,
    /// Giants
    III,
    /// Subgiants
    IV,
    /// Main sequence (dwarfs)
    V,
    /// Subdwarfs
    VI,
    /// White dwarfs
    VII,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionaryStage {
    /// Pre-Main Sequence (Contraction phase)
    PreMainSequence {
        age: f64, // In Jahren
    },
    /// Zero Age Main Sequence
    ZAMS,
    /// Main Sequence
    MainSequence {
        fraction_complete: f64,
    },
    /// Terminal Age Main Sequence
    TAMS,
    /// Post-Main Sequence stages
    RedGiant,
    HorizontalBranch,
    AsymptoticGiantBranch,
    BlueDwarf,
    WhiteDwarf {
        cooling_age: f64, // In Jahren
    },
    NeutronStar,
    BlackHole,
}
