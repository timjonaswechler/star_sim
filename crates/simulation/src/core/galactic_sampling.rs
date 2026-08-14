//! Seed-deterministic sampling of Galactic Positions.

use super::*;

/// Finite cylinder within which a seed may select a galactic position.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GalacticSamplingVolume {
    pub max_radius_pc: f64,
    pub max_abs_height_pc: f64,
    pub radial_bins: usize,
    pub vertical_bins: usize,
}

impl Default for GalacticSamplingVolume {
    fn default() -> Self {
        Self {
            max_radius_pc: 20_000.0,
            max_abs_height_pc: 10_000.0,
            radial_bins: 320,
            vertical_bins: 240,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SampledGalacticLocation {
    pub position: GalacticPosition,
    /// Population whose density contribution selected this position.
    pub sampled_population: StellarPopulation,
    pub local_density: PopulationDensity,
}

#[derive(Debug, Error)]
pub enum GalacticSamplingError {
    #[error(transparent)]
    InvalidGalaxyModel(#[from] GalaxyModelError),
    #[error("sampling volume must have positive finite dimensions and non-zero bin counts")]
    InvalidSamplingVolume,
    #[error("sampling volume contains no finite positive stellar density")]
    EmptySamplingVolume,
}

/// Precomputed density-weighted sampler for deterministic galactic positions.
pub struct GalacticLocationSampler {
    galaxy: GalaxyModel,
    volume: GalacticSamplingVolume,
    population_cdfs: [Vec<f64>; 3],
    population_totals: [f64; 3],
}

impl GalacticLocationSampler {
    pub fn new(
        galaxy: GalaxyModel,
        volume: GalacticSamplingVolume,
    ) -> Result<Self, GalacticSamplingError> {
        galaxy.validate()?;
        if !volume.max_radius_pc.is_finite()
            || volume.max_radius_pc <= 0.0
            || !volume.max_abs_height_pc.is_finite()
            || volume.max_abs_height_pc <= 0.0
            || volume.radial_bins == 0
            || volume.vertical_bins == 0
        {
            return Err(GalacticSamplingError::InvalidSamplingVolume);
        }

        let mut population_cdfs: [Vec<f64>; 3] =
            std::array::from_fn(|_| Vec::with_capacity(volume.radial_bins * volume.vertical_bins));
        let mut population_totals = [0.0; 3];
        let radial_step = volume.max_radius_pc / volume.radial_bins as f64;
        let vertical_step = 2.0 * volume.max_abs_height_pc / volume.vertical_bins as f64;

        for radial_index in 0..volume.radial_bins {
            let radius_inner = radial_index as f64 * radial_step;
            let radius_outer = radius_inner + radial_step;
            let radius_mid = (radius_inner + radius_outer) * 0.5;
            let annular_cell_volume = std::f64::consts::PI
                * (radius_outer.powi(2) - radius_inner.powi(2))
                * vertical_step;

            for vertical_index in 0..volume.vertical_bins {
                let height =
                    -volume.max_abs_height_pc + (vertical_index as f64 + 0.5) * vertical_step;
                let density = galaxy.stellar_number_density_at(GalacticPosition {
                    radius_pc: radius_mid,
                    azimuth_rad: 0.0,
                    height_pc: height,
                });

                for population in StellarPopulation::ALL {
                    let index = population_index(population);
                    population_totals[index] +=
                        density.for_population(population) * annular_cell_volume;
                    population_cdfs[index].push(population_totals[index]);
                }
            }
        }

        let total: f64 = population_totals.iter().sum();
        if !total.is_finite() || total <= 0.0 {
            return Err(GalacticSamplingError::EmptySamplingVolume);
        }

        Ok(Self {
            galaxy,
            volume,
            population_cdfs,
            population_totals,
        })
    }

    pub fn sample(&self, seed: u64) -> SampledGalacticLocation {
        let mut rng = position_rng(seed);
        let total: f64 = self.population_totals.iter().sum();
        let population_draw = rng.gen_range(0.0..total);
        let mut accumulated = 0.0;
        let mut sampled_population = StellarPopulation::Halo;
        for population in StellarPopulation::ALL {
            accumulated += self.population_totals[population_index(population)];
            if population_draw < accumulated {
                sampled_population = population;
                break;
            }
        }

        let population_index = population_index(sampled_population);
        let population_total = self.population_totals[population_index];
        let cell_draw = rng.gen_range(0.0..population_total);
        let cell_index = self.population_cdfs[population_index]
            .partition_point(|&cumulative| cumulative < cell_draw)
            .min(self.population_cdfs[population_index].len() - 1);
        let radial_index = cell_index / self.volume.vertical_bins;
        let vertical_index = cell_index % self.volume.vertical_bins;

        let radial_step = self.volume.max_radius_pc / self.volume.radial_bins as f64;
        let radius_inner = radial_index as f64 * radial_step;
        let radius_outer = radius_inner + radial_step;
        let radius_pc = rng
            .gen_range(radius_inner.powi(2)..radius_outer.powi(2))
            .sqrt();
        let vertical_step = 2.0 * self.volume.max_abs_height_pc / self.volume.vertical_bins as f64;
        let height_lower = -self.volume.max_abs_height_pc + vertical_index as f64 * vertical_step;
        let height_pc = rng.gen_range(height_lower..height_lower + vertical_step);
        let azimuth_rad = rng.gen_range(0.0..std::f64::consts::TAU);
        let position = GalacticPosition {
            radius_pc,
            azimuth_rad,
            height_pc,
        };

        SampledGalacticLocation {
            position,
            sampled_population,
            local_density: self.galaxy.stellar_number_density_at(position),
        }
    }

    pub fn population_probabilities(&self) -> [(StellarPopulation, f64); 3] {
        let total: f64 = self.population_totals.iter().sum();
        StellarPopulation::ALL.map(|population| {
            (
                population,
                self.population_totals[population_index(population)] / total,
            )
        })
    }
}
