//! Loading and assembly of the repository's versioned scientific model data.
//!
//! [`crate::core`] owns the model types and algorithms. This module owns the
//! RON representation and the location of bundled or local model data.

use crate::core::{
    ExplicitPlanetModel, GalaxyModel, PlanetOccurrenceModel, PlanetaryStabilityModel,
    PopulationHistoryModel, StellarBirthMassModel, StellarCatalogGenerator,
    StellarCatalogModelError, StellarEvolutionModel, StellarOrbitalHierarchyModel,
    SystemMultiplicityModel, WhiteDwarfCoolingModel,
};
use serde::de::DeserializeOwned;
use std::{fs, io, path::PathBuf};
use thiserror::Error;

const LOCAL_WHITE_DWARF_COOLING_FILE: &str = "white_dwarf_cooling.local.ron";

/// One coherent set of inputs used by the stellar-catalog generator and labs.
#[derive(Debug, Clone)]
pub struct SimulationModels {
    pub galaxy: GalaxyModel,
    pub nearby_m_dwarf_multiplicity: SystemMultiplicityModel,
    pub stellar_birth_masses: StellarBirthMassModel,
    pub stellar_population_history: PopulationHistoryModel,
    pub stellar_evolution: StellarEvolutionModel,
    pub stellar_orbital_hierarchy: StellarOrbitalHierarchyModel,
    pub planet_occurrence: PlanetOccurrenceModel,
    pub planetary_stability: PlanetaryStabilityModel,
    pub explicit_planets: ExplicitPlanetModel,
}

impl SimulationModels {
    /// Parses the versioned RON files embedded in this crate at compile time.
    pub fn bundled() -> Result<Self, ModelLoadError> {
        let models = Self {
            galaxy: parse_bundled(
                "milky_way.ron",
                include_str!("../../../assets/scientific_models/milky_way.ron"),
            )?,
            nearby_m_dwarf_multiplicity: parse_bundled(
                "nearby_m_dwarf_multiplicity.ron",
                include_str!("../../../assets/scientific_models/nearby_m_dwarf_multiplicity.ron"),
            )?,
            stellar_birth_masses: parse_bundled(
                "stellar_birth_masses.ron",
                include_str!("../../../assets/scientific_models/stellar_birth_masses.ron"),
            )?,
            stellar_population_history: parse_bundled(
                "stellar_population_history.ron",
                include_str!("../../../assets/scientific_models/stellar_population_history.ron"),
            )?,
            stellar_evolution: parse_bundled(
                "stellar_evolution.ron",
                include_str!("../../../assets/scientific_models/stellar_evolution.ron"),
            )?,
            stellar_orbital_hierarchy: parse_bundled(
                "stellar_orbital_hierarchy.ron",
                include_str!("../../../assets/scientific_models/stellar_orbital_hierarchy.ron"),
            )?,
            planet_occurrence: parse_bundled(
                "planet_occurrence.ron",
                include_str!("../../../assets/scientific_models/planet_occurrence.ron"),
            )?,
            planetary_stability: parse_bundled(
                "planetary_stability.ron",
                include_str!("../../../assets/scientific_models/planetary_stability.ron"),
            )?,
            explicit_planets: parse_bundled(
                "explicit_planets.ron",
                include_str!("../../../assets/scientific_models/explicit_planets.ron"),
            )?,
        };
        models.galaxy.validate()?;
        Ok(models)
    }

    /// Builds the catalog generator while keeping storage and parsing concerns out of callers.
    pub fn catalog_generator(&self) -> Result<StellarCatalogGenerator, StellarCatalogModelError> {
        StellarCatalogGenerator::new(
            self.stellar_birth_masses.clone(),
            self.stellar_population_history,
            self.stellar_evolution.clone(),
            self.planet_occurrence,
            self.stellar_orbital_hierarchy,
            self.planetary_stability,
            self.explicit_planets.clone(),
        )
    }
}

/// A locally generated cooling grid which cannot be redistributed with the repository.
#[derive(Debug)]
pub struct LocalWhiteDwarfCooling {
    pub path: PathBuf,
    pub model: WhiteDwarfCoolingModel,
}

/// Loads the ignored Montréal cooling model when it exists next to the bundled assets.
pub fn load_local_white_dwarf_cooling() -> Result<Option<LocalWhiteDwarfCooling>, ModelLoadError> {
    let path = local_white_dwarf_cooling_path();
    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(source) => return Err(ModelLoadError::ReadLocal { path, source }),
    };
    let model = ron::from_str(&source).map_err(|source| ModelLoadError::ParseLocal {
        path: path.clone(),
        source,
    })?;
    Ok(Some(LocalWhiteDwarfCooling { path, model }))
}

/// Expected path of the optional, locally generated Montréal cooling model.
pub fn local_white_dwarf_cooling_path() -> PathBuf {
    scientific_models_directory().join(LOCAL_WHITE_DWARF_COOLING_FILE)
}

/// Repository-local asset location used only for non-redistributable generated data.
pub fn scientific_models_directory() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/scientific_models")
}

#[derive(Debug, Error)]
pub enum ModelLoadError {
    #[error("failed to parse bundled scientific model `{name}`")]
    ParseBundled {
        name: &'static str,
        #[source]
        source: ron::error::SpannedError,
    },
    #[error("failed to read local scientific model `{}`", path.display())]
    ReadLocal {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse local scientific model `{}`", path.display())]
    ParseLocal {
        path: PathBuf,
        #[source]
        source: ron::error::SpannedError,
    },
    #[error(transparent)]
    InvalidGalaxy(#[from] crate::core::GalaxyModelError),
}

fn parse_bundled<T: DeserializeOwned>(
    name: &'static str,
    source: &'static str,
) -> Result<T, ModelLoadError> {
    ron::from_str(source).map_err(|source| ModelLoadError::ParseBundled { name, source })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_models_parse_and_construct_the_catalog_generator() {
        SimulationModels::bundled()
            .unwrap()
            .catalog_generator()
            .unwrap();
    }
}
