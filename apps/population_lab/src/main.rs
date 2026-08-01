//! PROTOTYPE: compare horizontal and vertical density sections before the
//! galactic population model is treated as validated simulation code.

use plotters::{
    coord::{Shift, types::RangedCoordf64},
    prelude::*,
};
use simulation_core::{
    EvolutionaryState, GalacticLocationSampler, GalacticPosition, GalacticSamplingVolume,
    GalaxyModel, GeneratedStellarCatalog, PlanetOccurrenceError, PlanetOccurrenceModel,
    PopulationHistoryModel, PopulationHistorySampler, SmallPlanetOccurrence, StellarBirthMassModel,
    StellarBirthMassSampler, StellarCatalogGenerator, StellarCatalogMember, StellarChemistry,
    StellarEvolutionError, StellarEvolutionEvaluator, StellarEvolutionModel,
    StellarEvolutionQualityFlag, StellarEvolutionSnapshot, StellarPopulation,
    StellarPopulationHistory, WhiteDwarfCoolingModel,
};
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

const MAP_HALF_WIDTH_PC: f64 = 20_000.0;
const MAP_HALF_HEIGHT_PC: f64 = 10_000.0;
const GRID_WIDTH: usize = 360;
const GRID_HEIGHT: usize = 180;
const LOG_DENSITY_MIN: f64 = -4.0;
const LOG_DENSITY_MAX: f64 = 1.5;

#[derive(Clone, Copy)]
enum DensityField {
    ThinDisk,
    ThickDisk,
    StellarHalo,
    BulgeMass,
}

impl DensityField {
    const ALL: [Self; 4] = [
        Self::ThinDisk,
        Self::ThickDisk,
        Self::StellarHalo,
        Self::BulgeMass,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::ThinDisk => "Thin disk — stars / pc³",
            Self::ThickDisk => "Thick disk — stars / pc³",
            Self::StellarHalo => "Stellar halo — stars / pc³",
            Self::BulgeMass => "Bulge shape — solar masses / pc³",
        }
    }

    fn value(self, galaxy: &GalaxyModel, position: GalacticPosition) -> f64 {
        match self {
            Self::ThinDisk => galaxy
                .stellar_number_density_at(position)
                .for_population(StellarPopulation::ThinDisk),
            Self::ThickDisk => galaxy
                .stellar_number_density_at(position)
                .for_population(StellarPopulation::ThickDisk),
            Self::StellarHalo => galaxy
                .stellar_number_density_at(position)
                .for_population(StellarPopulation::Halo),
            Self::BulgeMass => galaxy.bulge_mass_density_at(position),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let seed = parse_seed()?;
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/milky_way.ron");
    let galaxy: GalaxyModel = ron::de::from_reader(BufReader::new(File::open(&config_path)?))?;
    galaxy.validate()?;
    let birth_mass_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/stellar_birth_masses.ron");
    let birth_mass_model: StellarBirthMassModel =
        ron::de::from_reader(BufReader::new(File::open(&birth_mass_path)?))?;
    let birth_mass_sampler = StellarBirthMassSampler::new(birth_mass_model.clone())?;
    let history_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/stellar_population_history.ron");
    let history_model: PopulationHistoryModel =
        ron::de::from_reader(BufReader::new(File::open(&history_path)?))?;
    let history_sampler = PopulationHistorySampler::new(history_model)?;
    let evolution_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/stellar_evolution.ron");
    let evolution_model: StellarEvolutionModel =
        ron::de::from_reader(BufReader::new(File::open(&evolution_path)?))?;
    let evolution_evaluator = StellarEvolutionEvaluator::new(evolution_model.clone())?;
    let planet_occurrence_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../config/planet_occurrence.ron");
    let planet_occurrence_model: PlanetOccurrenceModel =
        ron::de::from_reader(BufReader::new(File::open(&planet_occurrence_path)?))?;
    let sampler = GalacticLocationSampler::new(galaxy, GalacticSamplingVolume::default())?;
    let sampled = sampler.sample(seed);
    let selected = sampled.position;
    let cooling_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../config/white_dwarf_cooling.local.ron");
    let catalog_generator = StellarCatalogGenerator::new(
        birth_mass_model,
        history_model,
        evolution_model,
        planet_occurrence_model,
    )?;
    let (catalog_generator, cooling_model_loaded) = if cooling_path.is_file() {
        let cooling_model: WhiteDwarfCoolingModel =
            ron::de::from_reader(BufReader::new(File::open(&cooling_path)?))?;
        (
            catalog_generator.with_white_dwarf_cooling(cooling_model)?,
            true,
        )
    } else {
        (catalog_generator, false)
    };
    let catalog = catalog_generator.generate(seed, sampled)?;
    let evolved_members: Vec<_> = catalog
        .systems
        .iter()
        .flat_map(|system| &system.members)
        .collect();

    let output_dir = "output/population_lab";
    fs::create_dir_all(output_dir)?;
    let total_path = format!("{output_dir}/galactic-density-sections.png");
    let components_path = format!("{output_dir}/population-density-components.png");
    let profiles_path = format!("{output_dir}/density-profiles.png");
    let region_path = format!("{output_dir}/local-stellar-region.png");
    let history_plot_path = format!("{output_dir}/population-history.png");
    let radial_metallicity_path = format!("{output_dir}/radial-metallicity-gradient.png");
    let chemistry_plot_path = format!("{output_dir}/stellar-chemistry.png");
    let birth_mass_plot_path = format!("{output_dir}/stellar-birth-masses.png");
    let evolution_plot_path = format!("{output_dir}/stellar-evolution.png");
    render_density_sections(&total_path, &galaxy, selected)?;
    render_population_components(&components_path, &galaxy, selected)?;
    render_density_profiles(&profiles_path, &galaxy, selected)?;
    render_local_region(&region_path, &catalog)?;
    render_population_history(
        &history_plot_path,
        history_sampler,
        seed,
        selected,
        &catalog,
    )?;
    render_radial_metallicity_gradient(&radial_metallicity_path, history_sampler, seed, selected)?;
    render_stellar_chemistry(&chemistry_plot_path, history_sampler, seed, selected)?;
    render_stellar_birth_masses(&birth_mass_plot_path, &birth_mass_sampler, seed, &catalog)?;
    render_stellar_evolution(&evolution_plot_path, &evolved_members, &evolution_evaluator)?;

    let density = galaxy.stellar_number_density_at(selected);
    println!("Loaded {}", config_path.display());
    println!("Loaded {}", planet_occurrence_path.display());
    if cooling_model_loaded {
        println!("Loaded {}", cooling_path.display());
    } else {
        println!(
            "White-dwarf cooling grid not loaded (optional local file: {})",
            cooling_path.display()
        );
    }
    println!("Wrote {total_path}");
    println!("Wrote {components_path}");
    println!("Wrote {profiles_path}");
    println!("Wrote {region_path}");
    println!("Wrote {history_plot_path}");
    println!("Wrote {radial_metallicity_path}");
    println!("Wrote {chemistry_plot_path}");
    println!("Wrote {birth_mass_plot_path}");
    println!("Wrote {evolution_plot_path}");
    println!("Seed: {seed}");
    println!("Sampled population: {}", sampled.sampled_population.label());
    println!(
        "Selected position: R={:.0} pc, phi={:.1} deg, z={:.0} pc",
        selected.radius_pc,
        selected.azimuth_rad.to_degrees(),
        selected.height_pc
    );
    println!("Local density: {:.4} stars/pc^3", density.total());
    for population in StellarPopulation::ALL {
        println!(
            "  {:<12} {:>8.4} stars/pc^3 ({:>5.1}%)",
            population.label(),
            density.for_population(population),
            density.fraction(population) * 100.0
        );
    }
    println!(
        "10 pc region: expected {:.1} systems, generated {}",
        catalog.expected_system_count,
        catalog.systems.len()
    );
    println!(
        "Birth-mass model: expected {:.3} stellar members per system",
        birth_mass_sampler.expected_members_per_system()
    );
    for member_count in 1..=4 {
        let count = catalog
            .systems
            .iter()
            .filter(|system| system.members.len() == member_count as usize)
            .count();
        println!("  {member_count}-star systems: {count}");
    }
    println!(
        "Stellar evolution outcomes ({} members):",
        evolved_members.len()
    );
    for category in EvolutionOutcomeCategory::ALL {
        let count = evolved_members
            .iter()
            .filter(|member| category.matches(&member.evolution))
            .count();
        println!("  {:<25} {count}", category.label());
    }
    let alpha_projected = evolved_members
        .iter()
        .filter_map(|member| member.evolution.as_ref().ok())
        .filter(|snapshot| {
            snapshot
                .quality_flags
                .contains(&StellarEvolutionQualityFlag::AlphaProjectedToSolarScaled)
        })
        .count();
    println!("  alpha-enhanced chemistry projected onto solar-scaled tracks: {alpha_projected}");
    let binary_ignored = evolved_members
        .iter()
        .filter_map(|member| member.evolution.as_ref().ok())
        .filter(|snapshot| {
            snapshot
                .quality_flags
                .contains(&StellarEvolutionQualityFlag::BinaryInteractionIgnored)
        })
        .count();
    println!("  independently evolved multiple-system members: {binary_ignored}");
    let calibrated_small_planet_hosts = evolved_members
        .iter()
        .filter(|member| member.planet_population.small_planets.is_ok())
        .count();
    let drawn_small_planets: u32 = evolved_members
        .iter()
        .filter_map(|member| member.planet_population.small_planets.as_ref().ok())
        .map(|occurrence| match occurrence {
            SmallPlanetOccurrence::FgkWarm {
                warm_super_earth_count,
                warm_sub_neptune_count,
            } => warm_super_earth_count + warm_sub_neptune_count,
            SmallPlanetOccurrence::MDwarfAggregate { small_planet_count } => *small_planet_count,
        })
        .sum();
    let calibrated_giant_planet_hosts = evolved_members
        .iter()
        .filter(|member| member.planet_population.giant_planets.is_ok())
        .count();
    let drawn_giant_hosts = evolved_members
        .iter()
        .filter_map(|member| member.planet_population.giant_planets.as_ref().ok())
        .filter(|occurrence| occurrence.has_at_least_one_cps_giant)
        .count();
    let unknown_multiplicity = evolved_members
        .iter()
        .filter(|member| {
            matches!(
                member.planet_population.small_planets,
                Err(PlanetOccurrenceError::MultiplicitySeparationRequired)
            )
        })
        .count();
    println!("Planet occurrence summaries:");
    println!(
        "  calibrated small-planet hosts: {calibrated_small_planet_hosts}, drawn planets in calibrated domains: {drawn_small_planets}"
    );
    println!(
        "  calibrated giant-planet hosts: {calibrated_giant_planet_hosts}, hosts with a CPS-domain giant: {drawn_giant_hosts}"
    );
    println!("  members awaiting companion separation: {unknown_multiplicity}");
    for population in StellarPopulation::ALL {
        let matching: Vec<_> = catalog
            .systems
            .iter()
            .filter(|system| system.population == population)
            .map(|system| system.history)
            .collect();
        if !matching.is_empty() {
            let mean_age =
                matching.iter().map(|history| history.age_gyr).sum::<f64>() / matching.len() as f64;
            let mean_feh = matching
                .iter()
                .map(|history| history.chemistry.iron_abundance_feh)
                .sum::<f64>()
                / matching.len() as f64;
            let mean_alpha = matching
                .iter()
                .map(|history| history.chemistry.alpha_enhancement_alpha_fe)
                .sum::<f64>()
                / matching.len() as f64;
            println!(
                "  {:<12} {} systems, mean age {:.1} Gyr, mean [Fe/H] {:+.2}, mean [alpha/Fe] {:+.2}",
                population.label(),
                matching.len(),
                mean_age,
                mean_feh,
                mean_alpha,
            );
        }
    }
    Ok(())
}

fn parse_seed() -> Result<u64, Box<dyn Error>> {
    let mut arguments = env::args().skip(1);
    let mut seed = 42;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--seed" => {
                let value = arguments
                    .next()
                    .ok_or("--seed requires an unsigned integer")?;
                seed = value.parse()?;
            }
            _ => return Err(format!("unknown argument `{argument}`; use `--seed <u64>`").into()),
        }
    }
    Ok(seed)
}

#[derive(Clone, Copy)]
enum EvolutionOutcomeCategory {
    PreMainSequence,
    MainSequence,
    PostMainSequenceLuminous,
    WhiteDwarf,
    UnsupportedTerminal,
    OutsideGrid,
}

impl EvolutionOutcomeCategory {
    const ALL: [Self; 6] = [
        Self::PreMainSequence,
        Self::MainSequence,
        Self::PostMainSequenceLuminous,
        Self::WhiteDwarf,
        Self::UnsupportedTerminal,
        Self::OutsideGrid,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::PreMainSequence => "pre-main sequence",
            Self::MainSequence => "main sequence",
            Self::PostMainSequenceLuminous => "luminous post-main sequence",
            Self::WhiteDwarf => "white dwarf",
            Self::UnsupportedTerminal => "unsupported terminal path",
            Self::OutsideGrid => "outside grid",
        }
    }

    fn matches(self, result: &Result<StellarEvolutionSnapshot, StellarEvolutionError>) -> bool {
        match (self, result) {
            (Self::PreMainSequence, Ok(snapshot)) => {
                snapshot.state == EvolutionaryState::PreMainSequence
            }
            (Self::MainSequence, Ok(snapshot)) => snapshot.state == EvolutionaryState::MainSequence,
            (Self::PostMainSequenceLuminous, Ok(snapshot)) => {
                matches!(
                    snapshot.state,
                    EvolutionaryState::SubgiantAndRedGiantBranch
                        | EvolutionaryState::HeliumIgnitionTransition
                        | EvolutionaryState::CoreHeliumBurning
                        | EvolutionaryState::EarlyAsymptoticGiantBranch
                        | EvolutionaryState::ThermallyPulsingAsymptoticGiantBranch
                        | EvolutionaryState::AdvancedBurningTrackEnd
                        | EvolutionaryState::WolfRayet
                        | EvolutionaryState::PostAsymptoticGiantBranch
                )
            }
            (Self::WhiteDwarf, Ok(snapshot)) => snapshot.state == EvolutionaryState::WhiteDwarf,
            (Self::UnsupportedTerminal, Err(error)) => matches!(
                error,
                StellarEvolutionError::PostMainSequenceNotBundled { .. }
                    | StellarEvolutionError::UnsupportedCoreCollapse { .. }
                    | StellarEvolutionError::PostAgbTrackIncomplete { .. }
                    | StellarEvolutionError::TrackEndedBeforeExpectedEndpoint { .. }
            ),
            (Self::OutsideGrid, Err(error)) => matches!(
                error,
                StellarEvolutionError::InvalidModel
                    | StellarEvolutionError::InvalidInput { .. }
                    | StellarEvolutionError::OutsideMassGrid { .. }
                    | StellarEvolutionError::OutsideMetallicityGrid { .. }
                    | StellarEvolutionError::AgeBeforeTrack { .. }
            ),
            _ => false,
        }
    }
}

fn render_stellar_evolution(
    path: &str,
    members: &[&StellarCatalogMember],
    evaluator: &StellarEvolutionEvaluator,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (1800, 650)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let panels = root.split_evenly((1, 3));

    let supported: Vec<_> = members
        .iter()
        .filter_map(|member| member.evolution.as_ref().ok())
        .collect();
    let mut hr = ChartBuilder::on(&panels[0])
        .caption(
            "Present-day HR plane — bundled tracks",
            ("sans-serif", 22).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(48)
        .y_label_area_size(58)
        .build_cartesian_2d(-5.1_f64..-3.3_f64, -5.0_f64..6.5_f64)?;
    hr.configure_mesh()
        .x_label_formatter(&|value| format!("{:.1}", -value))
        .x_desc("log10 effective temperature [K] — hot to cool")
        .y_desc("log10 L / L☉")
        .axis_desc_style(("sans-serif", 20).into_font().color(&WHITE))
        .label_style(("sans-serif", 16).into_font().color(&WHITE))
        .bold_line_style(RGBAColor(255, 255, 255, 0.16))
        .light_line_style(RGBAColor(255, 255, 255, 0.06))
        .draw()?;
    let solar_reference = solar_mass_reference_track(evaluator);
    hr.draw_series(LineSeries::new(
        solar_reference.iter().filter_map(|snapshot| {
            Some((
                -snapshot.effective_temperature_k?.log10(),
                snapshot.luminosity_lsun?.log10(),
            ))
        }),
        WHITE.mix(0.32).stroke_width(2),
    ))?
    .label("1 M☉ solar-composition reference")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], WHITE.mix(0.32).stroke_width(2)));
    for state in [
        EvolutionaryState::PreMainSequence,
        EvolutionaryState::MainSequence,
        EvolutionaryState::SubgiantAndRedGiantBranch,
        EvolutionaryState::HeliumIgnitionTransition,
        EvolutionaryState::CoreHeliumBurning,
        EvolutionaryState::EarlyAsymptoticGiantBranch,
        EvolutionaryState::ThermallyPulsingAsymptoticGiantBranch,
        EvolutionaryState::AdvancedBurningTrackEnd,
        EvolutionaryState::WolfRayet,
        EvolutionaryState::PostAsymptoticGiantBranch,
        EvolutionaryState::WhiteDwarf,
    ]
    .into_iter()
    .filter(|state| {
        supported.iter().any(|snapshot| {
            snapshot.state == *state
                && snapshot.effective_temperature_k.is_some()
                && snapshot.luminosity_lsun.is_some()
        })
    }) {
        let color = evolution_state_color(state);
        hr.draw_series(
            supported
                .iter()
                .filter(|snapshot| {
                    snapshot.state == state
                        && snapshot.effective_temperature_k.is_some()
                        && snapshot.luminosity_lsun.is_some()
                })
                .map(|snapshot| {
                    Circle::new(
                        (
                            -snapshot.effective_temperature_k.unwrap().log10(),
                            snapshot.luminosity_lsun.unwrap().log10(),
                        ),
                        5,
                        color.filled(),
                    )
                }),
        )?
        .label(state.label())
        .legend(move |(x, y)| Circle::new((x + 10, y), 5, color.filled()));
    }
    hr.configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 15).into_font().color(&WHITE))
        .draw()?;

    let mut mass = ChartBuilder::on(&panels[1])
        .caption(
            "Birth mass → current track mass",
            ("sans-serif", 22).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(48)
        .y_label_area_size(58)
        .build_cartesian_2d(-1.1_f64..2.05_f64, -1.1_f64..2.05_f64)?;
    style_mesh(
        &mut mass,
        "log10 initial mass [M☉]",
        "log10 current mass [M☉]",
    )?;
    mass.draw_series(LineSeries::new(
        [(-1.1, -1.1), (2.05, 2.05)],
        WHITE.mix(0.25).stroke_width(2),
    ))?;
    mass.draw_series(supported.iter().map(|snapshot| {
        Circle::new(
            (
                snapshot.initial_mass_msun.log10(),
                snapshot.current_mass_msun.log10(),
            ),
            5,
            evolution_state_color(snapshot.state).filled(),
        )
    }))?;

    let maximum_count = EvolutionOutcomeCategory::ALL
        .iter()
        .map(|category| {
            members
                .iter()
                .filter(|member| category.matches(&member.evolution))
                .count()
        })
        .max()
        .unwrap_or(1)
        .max(1) as f64;
    let mut outcomes = ChartBuilder::on(&panels[2])
        .caption(
            "Model coverage and present state",
            ("sans-serif", 22).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(48)
        .y_label_area_size(58)
        .build_cartesian_2d(-0.5_f64..5.5_f64, 0.0_f64..maximum_count * 1.2)?;
    outcomes
        .configure_mesh()
        .disable_x_mesh()
        .x_labels(6)
        .x_label_formatter(&|value| {
            let rounded = value.round();
            if (value - rounded).abs() > 1e-6 {
                return String::new();
            }
            let index = rounded as usize;
            EvolutionOutcomeCategory::ALL
                .get(index)
                .map(|category| match category {
                    EvolutionOutcomeCategory::PreMainSequence => "PMS",
                    EvolutionOutcomeCategory::MainSequence => "MS",
                    EvolutionOutcomeCategory::PostMainSequenceLuminous => "post-MS",
                    EvolutionOutcomeCategory::WhiteDwarf => "WD",
                    EvolutionOutcomeCategory::UnsupportedTerminal => "terminal?",
                    EvolutionOutcomeCategory::OutsideGrid => "outside",
                })
                .unwrap_or("")
                .to_string()
        })
        .x_desc("outcome")
        .y_desc("stellar members")
        .axis_desc_style(("sans-serif", 20).into_font().color(&WHITE))
        .label_style(("sans-serif", 15).into_font().color(&WHITE))
        .bold_line_style(RGBAColor(255, 255, 255, 0.16))
        .light_line_style(RGBAColor(255, 255, 255, 0.06))
        .draw()?;
    for (index, category) in EvolutionOutcomeCategory::ALL.into_iter().enumerate() {
        let count = members
            .iter()
            .filter(|member| category.matches(&member.evolution))
            .count();
        let color = match category {
            EvolutionOutcomeCategory::PreMainSequence => {
                evolution_state_color(EvolutionaryState::PreMainSequence)
            }
            EvolutionOutcomeCategory::MainSequence => {
                evolution_state_color(EvolutionaryState::MainSequence)
            }
            EvolutionOutcomeCategory::PostMainSequenceLuminous => {
                evolution_state_color(EvolutionaryState::SubgiantAndRedGiantBranch)
            }
            EvolutionOutcomeCategory::WhiteDwarf => {
                evolution_state_color(EvolutionaryState::WhiteDwarf)
            }
            EvolutionOutcomeCategory::UnsupportedTerminal => RGBColor(255, 110, 85),
            EvolutionOutcomeCategory::OutsideGrid => RGBColor(150, 155, 175),
        };
        outcomes.draw_series(std::iter::once(Rectangle::new(
            [
                (index as f64 - 0.38, 0.0),
                (index as f64 + 0.38, count as f64),
            ],
            color.filled(),
        )))?;
    }

    root.present()?;
    Ok(())
}

fn solar_mass_reference_track(
    evaluator: &StellarEvolutionEvaluator,
) -> Vec<StellarEvolutionSnapshot> {
    let chemistry = StellarChemistry {
        iron_abundance_feh: 0.0,
        alpha_enhancement_alpha_fe: 0.0,
        global_metallicity_mh: 0.0,
        hydrogen_mass_fraction_x: 0.7154,
        helium_mass_fraction_y: 0.2703,
        metal_mass_fraction_z: 0.0142,
    };
    [
        (0.000_002, 9.9, 120_usize),
        (9.9, 11.336, 120),
        (11.336, 11.4476, 120),
        (11.4476, 11.46293, 160),
        (11.46293, 11.46296, 120),
    ]
    .into_iter()
    .flat_map(|(start, end, count)| {
        (0..count).filter_map(move |index| {
            let fraction = index as f64 / (count - 1) as f64;
            evaluator
                .evaluate(1.0, start + (end - start) * fraction, chemistry)
                .ok()
        })
    })
    .collect()
}

fn render_population_history(
    path: &str,
    sampler: PopulationHistorySampler,
    seed: u64,
    position: GalacticPosition,
    catalog: &GeneratedStellarCatalog,
) -> Result<(), Box<dyn Error>> {
    let mut reference = Vec::with_capacity(9_000);
    for (population_index, population) in StellarPopulation::ALL.into_iter().enumerate() {
        for sample_index in 0..3_000_u64 {
            let id = population_index as u64 * 10_000 + sample_index;
            reference.push((population, sampler.sample(seed, id, population, position)));
        }
    }

    let root = BitMapBackend::new(path, (1800, 650)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let panels = root.split_evenly((1, 3));

    let mut scatter = ChartBuilder::on(&panels[0])
        .caption(
            "Age–chemistry prior — model samples",
            ("sans-serif", 23).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(48)
        .y_label_area_size(58)
        .build_cartesian_2d(0.0_f64..13.8_f64, -4.0_f64..0.75_f64)?;
    style_mesh(&mut scatter, "age [Gyr]", "[Fe/H] [dex]")?;
    for population in StellarPopulation::ALL {
        let color = population_color(population);
        scatter
            .draw_series(
                reference
                    .iter()
                    .filter(|(candidate, _)| *candidate == population)
                    .step_by(3)
                    .map(|(_, history)| {
                        Circle::new(
                            (history.age_gyr, history.chemistry.iron_abundance_feh),
                            2,
                            color.mix(0.28).filled(),
                        )
                    }),
            )?
            .label(population.label())
            .legend(move |(x, y)| Circle::new((x + 10, y), 4, color.filled()));
    }
    scatter
        .draw_series(catalog.systems.iter().map(|system| {
            let history = system.history;
            Cross::new(
                (history.age_gyr, history.chemistry.iron_abundance_feh),
                6,
                WHITE.stroke_width(2),
            )
        }))?
        .label("Current region")
        .legend(|(x, y)| Cross::new((x + 10, y), 5, WHITE.stroke_width(2)));
    scatter
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 14).into_font().color(&WHITE))
        .draw()?;

    render_history_histogram(
        &panels[1],
        "Age distribution",
        "age [Gyr]",
        0.0,
        13.8,
        &reference,
        |history| history.age_gyr,
    )?;
    render_history_histogram(
        &panels[2],
        "Metallicity distribution",
        "[Fe/H] [dex]",
        -4.0,
        0.75,
        &reference,
        |history| history.chemistry.iron_abundance_feh,
    )?;
    root.present()?;
    Ok(())
}

fn render_radial_metallicity_gradient(
    path: &str,
    sampler: PopulationHistorySampler,
    seed: u64,
    selected: GalacticPosition,
) -> Result<(), Box<dyn Error>> {
    const RADIAL_STEPS: usize = 80;
    const SAMPLES_PER_STEP: u64 = 500;
    let root = BitMapBackend::new(path, (1_200, 720)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Radial iron-abundance model (current-position proxy)",
            ("sans-serif", 25).into_font().color(&WHITE),
        )
        .margin(24)
        .x_label_area_size(58)
        .y_label_area_size(68)
        .build_cartesian_2d(0.0_f64..20.0_f64, -2.25_f64..0.75_f64)?;
    style_mesh(
        &mut chart,
        "galactocentric radius R [kpc]",
        "mean sampled [Fe/H] [dex]",
    )?;

    for (population_index, population) in StellarPopulation::ALL.into_iter().enumerate() {
        let points = (0..=RADIAL_STEPS).map(|step| {
            let radius_kpc = 20.0 * step as f64 / RADIAL_STEPS as f64;
            let position = GalacticPosition {
                radius_pc: radius_kpc * 1_000.0,
                azimuth_rad: selected.azimuth_rad,
                height_pc: selected.height_pc,
            };
            let mean = (0..SAMPLES_PER_STEP)
                .map(|sample_index| {
                    let id = 1_000_000
                        + population_index as u64 * 100_000
                        + step as u64 * SAMPLES_PER_STEP
                        + sample_index;
                    sampler
                        .sample(seed, id, population, position)
                        .chemistry
                        .iron_abundance_feh
                })
                .sum::<f64>()
                / SAMPLES_PER_STEP as f64;
            (radius_kpc, mean)
        });
        let color = population_color(population);
        chart
            .draw_series(LineSeries::new(points, color.stroke_width(4)))?
            .label(population.label())
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 24, y)], color.stroke_width(4))
            });
    }

    let selected_radius_kpc = selected.radius_pc / 1_000.0;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(selected_radius_kpc, -2.25), (selected_radius_kpc, 0.75)],
            WHITE.mix(0.75).stroke_width(2),
        )))?
        .label(format!("selected R = {selected_radius_kpc:.1} kpc"))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], WHITE.stroke_width(2)));
    chart
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 15).into_font().color(&WHITE))
        .draw()?;
    root.present()?;
    Ok(())
}

fn render_stellar_chemistry(
    path: &str,
    sampler: PopulationHistorySampler,
    seed: u64,
    position: GalacticPosition,
) -> Result<(), Box<dyn Error>> {
    let mut samples = Vec::with_capacity(9_000);
    for (population_index, population) in StellarPopulation::ALL.into_iter().enumerate() {
        for sample_index in 0..3_000_u64 {
            let id = 2_000_000 + population_index as u64 * 10_000 + sample_index;
            samples.push((population, sampler.sample(seed, id, population, position)));
        }
    }

    let root = BitMapBackend::new(path, (1_800, 650)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let panels = root.split_evenly((1, 3));

    let mut alpha_chart = ChartBuilder::on(&panels[0])
        .caption(
            "Composite alpha enhancement",
            ("sans-serif", 23).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(52)
        .y_label_area_size(60)
        .build_cartesian_2d(-4.0_f64..0.75_f64, -0.15_f64..0.65_f64)?;
    style_mesh(&mut alpha_chart, "[Fe/H] [dex]", "[alpha/Fe] [dex]")?;

    let mut global_chart = ChartBuilder::on(&panels[1])
        .caption(
            "Alpha-corrected global metallicity",
            ("sans-serif", 23).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(52)
        .y_label_area_size(60)
        .build_cartesian_2d(-4.0_f64..0.75_f64, -4.0_f64..1.0_f64)?;
    style_mesh(&mut global_chart, "[Fe/H] [dex]", "[M/H] [dex]")?;
    global_chart.draw_series(std::iter::once(PathElement::new(
        vec![(-4.0, -4.0), (0.75, 0.75)],
        WHITE.mix(0.35).stroke_width(2),
    )))?;

    let mut mass_chart = ChartBuilder::on(&panels[2])
        .caption(
            "Derived initial metal fraction",
            ("sans-serif", 23).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(52)
        .y_label_area_size(60)
        .build_cartesian_2d(-4.0_f64..1.0_f64, -6.0_f64..-0.8_f64)?;
    style_mesh(&mut mass_chart, "[M/H] [dex]", "log10(Z)")?;

    for population in StellarPopulation::ALL {
        let color = population_color(population);
        let population_samples: Vec<_> = samples
            .iter()
            .filter(|(candidate, _)| *candidate == population)
            .step_by(2)
            .collect();
        alpha_chart
            .draw_series(population_samples.iter().map(|(_, history)| {
                let chemistry = history.chemistry;
                Circle::new(
                    (
                        chemistry.iron_abundance_feh,
                        chemistry.alpha_enhancement_alpha_fe,
                    ),
                    2,
                    color.mix(0.3).filled(),
                )
            }))?
            .label(population.label())
            .legend(move |(x, y)| Circle::new((x + 10, y), 4, color.filled()));
        global_chart.draw_series(population_samples.iter().map(|(_, history)| {
            let chemistry = history.chemistry;
            Circle::new(
                (
                    chemistry.iron_abundance_feh,
                    chemistry.global_metallicity_mh,
                ),
                2,
                color.mix(0.3).filled(),
            )
        }))?;
        mass_chart.draw_series(population_samples.iter().map(|(_, history)| {
            let chemistry = history.chemistry;
            Circle::new(
                (
                    chemistry.global_metallicity_mh,
                    chemistry.metal_mass_fraction_z.log10(),
                ),
                2,
                color.mix(0.3).filled(),
            )
        }))?;
    }
    alpha_chart
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 14).into_font().color(&WHITE))
        .draw()?;
    root.present()?;
    Ok(())
}

fn render_history_histogram<F>(
    area: &DrawingArea<BitMapBackend<'_>, Shift>,
    title: &str,
    x_label: &str,
    minimum: f64,
    maximum: f64,
    samples: &[(StellarPopulation, StellarPopulationHistory)],
    value_of: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(StellarPopulationHistory) -> f64 + Copy,
{
    const BIN_COUNT: usize = 44;
    let bin_width = (maximum - minimum) / BIN_COUNT as f64;
    let mut distributions = Vec::new();
    let mut maximum_fraction = 0.0_f64;

    for population in StellarPopulation::ALL {
        let mut bins = vec![0_usize; BIN_COUNT];
        let mut count = 0_usize;
        for (_, history) in samples
            .iter()
            .filter(|(candidate, _)| *candidate == population)
        {
            let index = (((value_of(*history) - minimum) / bin_width).floor() as isize)
                .clamp(0, BIN_COUNT as isize - 1) as usize;
            bins[index] += 1;
            count += 1;
        }
        let points: Vec<_> = bins
            .into_iter()
            .enumerate()
            .map(|(index, bin_count)| {
                let fraction = bin_count as f64 / count as f64;
                maximum_fraction = maximum_fraction.max(fraction);
                (minimum + (index as f64 + 0.5) * bin_width, fraction)
            })
            .collect();
        distributions.push((population, points));
    }

    let mut chart = ChartBuilder::on(area)
        .caption(title, ("sans-serif", 23).into_font().color(&WHITE))
        .margin(18)
        .x_label_area_size(48)
        .y_label_area_size(58)
        .build_cartesian_2d(minimum..maximum, 0.0..maximum_fraction * 1.15)?;
    style_mesh(&mut chart, x_label, "fraction per bin")?;
    for (population, points) in distributions {
        let color = population_color(population);
        chart
            .draw_series(LineSeries::new(points, color.stroke_width(3)))?
            .label(population.label())
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 22, y)], color.stroke_width(3))
            });
    }
    chart
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 14).into_font().color(&WHITE))
        .draw()?;
    Ok(())
}

fn render_stellar_birth_masses(
    path: &str,
    sampler: &StellarBirthMassSampler,
    seed: u64,
    region: &GeneratedStellarCatalog,
) -> Result<(), Box<dyn Error>> {
    const SAMPLE_COUNT: u64 = 5_000;
    const MASS_BINS: usize = 48;
    const MASS_RATIO_BINS: usize = 40;
    let log_mass_minimum = 0.08_f64.log10();
    let log_mass_maximum = 100.0_f64.log10();
    let log_mass_width = (log_mass_maximum - log_mass_minimum) / MASS_BINS as f64;
    let mut primary_counts = vec![0_usize; MASS_BINS];
    let mut component_counts = vec![0_usize; MASS_BINS];
    let mut component_count = 0_usize;
    let mut mass_ratio_counts = vec![0_usize; MASS_RATIO_BINS];
    let mut companion_count = 0_usize;

    for sample_index in 0..SAMPLE_COUNT {
        let birth = sampler.sample(seed, 3_000_000 + sample_index);
        let primary = birth.members[0].initial_mass_msun;
        let mass_bin = (((primary.log10() - log_mass_minimum) / log_mass_width).floor() as isize)
            .clamp(0, MASS_BINS as isize - 1) as usize;
        primary_counts[mass_bin] += 1;
        for member in &birth.members {
            let component_bin = (((member.initial_mass_msun.log10() - log_mass_minimum)
                / log_mass_width)
                .floor() as isize)
                .clamp(0, MASS_BINS as isize - 1) as usize;
            component_counts[component_bin] += 1;
            component_count += 1;
        }
        for companion in &birth.members[1..] {
            let mass_ratio = companion.mass_ratio_to_primary.unwrap();
            let ratio_bin =
                ((mass_ratio * MASS_RATIO_BINS as f64).floor() as usize).min(MASS_RATIO_BINS - 1);
            mass_ratio_counts[ratio_bin] += 1;
            companion_count += 1;
        }
    }

    let primary_points: Vec<_> = primary_counts
        .iter()
        .enumerate()
        .map(|(index, count)| {
            (
                log_mass_minimum + (index as f64 + 0.5) * log_mass_width,
                *count as f64 / SAMPLE_COUNT as f64,
            )
        })
        .collect();
    let primary_maximum = primary_points
        .iter()
        .map(|(_, fraction)| *fraction)
        .fold(0.0_f64, f64::max);
    let component_points = component_counts.iter().enumerate().map(|(index, count)| {
        (
            log_mass_minimum + (index as f64 + 0.5) * log_mass_width,
            *count as f64 / component_count as f64,
        )
    });
    let component_maximum = component_counts
        .iter()
        .map(|count| *count as f64 / component_count as f64)
        .fold(0.0_f64, f64::max);
    let mass_distribution_maximum = primary_maximum.max(component_maximum);
    let multiplicity_points = (0..=400).map(|index| {
        let log_mass =
            log_mass_minimum + index as f64 / 400.0 * (log_mass_maximum - log_mass_minimum);
        let primary_mass = 10_f64.powf(log_mass).clamp(0.08, 100.0);
        (
            log_mass,
            sampler
                .multiplicity_fraction_for_primary_mass(primary_mass)
                .expect("plot range follows validated IMF bounds"),
        )
    });
    let mass_ratio_points = mass_ratio_counts.iter().enumerate().map(|(index, count)| {
        (
            (index as f64 + 0.5) / MASS_RATIO_BINS as f64,
            *count as f64 / companion_count as f64,
        )
    });

    let root = BitMapBackend::new(path, (1_800, 650)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let panels = root.split_evenly((1, 3));
    let mut imf_chart = ChartBuilder::on(&panels[0])
        .caption(
            "Primary-mass proxy from Kroupa IMF",
            ("sans-serif", 23).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(54)
        .y_label_area_size(58)
        .build_cartesian_2d(
            log_mass_minimum..log_mass_maximum,
            0.0..mass_distribution_maximum * 1.12,
        )?;
    style_mesh(
        &mut imf_chart,
        "log10(initial primary mass / solar mass)",
        "fraction per log-mass bin",
    )?;
    imf_chart
        .draw_series(LineSeries::new(
            primary_points,
            RGBColor(50, 205, 255).stroke_width(3),
        ))?
        .label("Primary proxy")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 22, y)],
                RGBColor(50, 205, 255).stroke_width(3),
            )
        });
    imf_chart
        .draw_series(LineSeries::new(
            component_points,
            RGBColor(185, 105, 255).stroke_width(3),
        ))?
        .label("All components")
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 22, y)],
                RGBColor(185, 105, 255).stroke_width(3),
            )
        });
    imf_chart.draw_series(region.systems.iter().map(|system| {
        let mass = system.members[0].birth.initial_mass_msun.log10();
        PathElement::new(
            vec![(mass, 0.0), (mass, mass_distribution_maximum * 0.09)],
            WHITE.mix(0.65).stroke_width(2),
        )
    }))?;
    imf_chart
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 14).into_font().color(&WHITE))
        .draw()?;

    let mut multiplicity_chart = ChartBuilder::on(&panels[1])
        .caption(
            "Multiplicity depends on primary mass",
            ("sans-serif", 23).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(54)
        .y_label_area_size(58)
        .build_cartesian_2d(log_mass_minimum..log_mass_maximum, 0.0..1.0)?;
    style_mesh(
        &mut multiplicity_chart,
        "log10(initial primary mass / solar mass)",
        "multiple-system fraction",
    )?;
    multiplicity_chart.draw_series(LineSeries::new(
        multiplicity_points,
        RGBColor(255, 178, 36).stroke_width(4),
    ))?;

    let mass_ratio_maximum = mass_ratio_counts
        .iter()
        .map(|count| *count as f64 / companion_count as f64)
        .fold(0.0_f64, f64::max);
    let mut ratio_chart = ChartBuilder::on(&panels[2])
        .caption(
            "Companion mass ratios",
            ("sans-serif", 23).into_font().color(&WHITE),
        )
        .margin(18)
        .x_label_area_size(54)
        .y_label_area_size(58)
        .build_cartesian_2d(0.0_f64..1.0_f64, 0.0..mass_ratio_maximum * 1.12)?;
    style_mesh(
        &mut ratio_chart,
        "q = companion mass / primary mass",
        "fraction per q bin",
    )?;
    ratio_chart.draw_series(LineSeries::new(
        mass_ratio_points,
        RGBColor(185, 105, 255).stroke_width(3),
    ))?;
    root.present()?;
    Ok(())
}

fn render_local_region(path: &str, region: &GeneratedStellarCatalog) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (1800, 650)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let panels = root.split_evenly((1, 3));
    render_region_projection(
        &panels[0],
        "Local region — x/y",
        "x [pc]",
        "y [pc]",
        region,
        |p| (p[0], p[1]),
    )?;
    render_region_projection(
        &panels[1],
        "Local region — x/z",
        "x [pc]",
        "z [pc]",
        region,
        |p| (p[0], p[2]),
    )?;
    render_region_projection(
        &panels[2],
        "Local region — y/z",
        "y [pc]",
        "z [pc]",
        region,
        |p| (p[1], p[2]),
    )?;
    root.present()?;
    Ok(())
}

fn render_region_projection<F>(
    area: &DrawingArea<BitMapBackend<'_>, Shift>,
    title: &str,
    x_label: &str,
    y_label: &str,
    region: &GeneratedStellarCatalog,
    project: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn([f64; 3]) -> (f64, f64) + Copy,
{
    let radius = region.radius_pc;
    let mut chart = ChartBuilder::on(area)
        .caption(title, ("sans-serif", 24).into_font().color(&WHITE))
        .margin(18)
        .x_label_area_size(48)
        .y_label_area_size(52)
        .build_cartesian_2d(-radius..radius, -radius..radius)?;
    style_mesh(&mut chart, x_label, y_label)?;

    chart.draw_series(LineSeries::new(
        (0..=240).map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / 240.0;
            (radius * angle.cos(), radius * angle.sin())
        }),
        RGBAColor(255, 255, 255, 0.35).stroke_width(2),
    ))?;

    for member_count in 1..=4 {
        let color = member_color(member_count);
        chart
            .draw_series(
                region
                    .systems
                    .iter()
                    .filter(|system| system.members.len() == member_count as usize)
                    .map(|system| {
                        Circle::new(
                            project(system.offset_pc),
                            2 + i32::from(member_count),
                            color.filled(),
                        )
                    }),
            )?
            .label(format!("{member_count} star"))
            .legend(move |(x, y)| Circle::new((x + 10, y), 3, color.filled()));
    }
    chart
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 15).into_font().color(&WHITE))
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;
    Ok(())
}

fn member_color(member_count: u8) -> RGBColor {
    match member_count {
        1 => RGBColor(50, 210, 255),
        2 => RGBColor(255, 205, 55),
        3 => RGBColor(205, 105, 255),
        _ => RGBColor(255, 80, 85),
    }
}

fn render_population_components(
    path: &str,
    galaxy: &GalaxyModel,
    selected: GalacticPosition,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (1800, 1200)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let panels = root.split_evenly((2, 2));

    for (area, field) in panels.iter().zip(DensityField::ALL) {
        let mut chart = ChartBuilder::on(area)
            .caption(field.label(), ("sans-serif", 25).into_font().color(&WHITE))
            .margin(18)
            .x_label_area_size(48)
            .y_label_area_size(58)
            .build_cartesian_2d(
                -MAP_HALF_WIDTH_PC / 1_000.0..MAP_HALF_WIDTH_PC / 1_000.0,
                -MAP_HALF_HEIGHT_PC / 1_000.0..MAP_HALF_HEIGHT_PC / 1_000.0,
            )?;
        style_mesh(&mut chart, "signed R [kpc]", "z [kpc]")?;
        draw_vertical_density_grid(&mut chart, galaxy, selected, Some(field))?;
        draw_marker(
            &mut chart,
            selected.radius_pc / 1_000.0,
            selected.height_pc / 1_000.0,
        )?;
    }
    root.present()?;
    Ok(())
}

fn render_density_sections(
    path: &str,
    galaxy: &GalaxyModel,
    selected: GalacticPosition,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (2000, 900)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let (maps, legend) = root.split_horizontally(1800);
    let panels = maps.split_evenly((1, 2));

    render_horizontal(&panels[0], galaxy, selected)?;
    render_vertical(&panels[1], galaxy, selected)?;
    render_density_legend(&legend)?;
    root.present()?;
    Ok(())
}

fn render_horizontal(
    area: &DrawingArea<BitMapBackend<'_>, Shift>,
    galaxy: &GalaxyModel,
    selected: GalacticPosition,
) -> Result<(), Box<dyn Error>> {
    let mut chart = ChartBuilder::on(area)
        .caption(
            "Stellar-number density — horizontal section",
            ("sans-serif", 28).into_font().color(&WHITE),
        )
        .margin(22)
        .x_label_area_size(52)
        .y_label_area_size(65)
        .build_cartesian_2d(
            -MAP_HALF_WIDTH_PC / 1_000.0..MAP_HALF_WIDTH_PC / 1_000.0,
            -MAP_HALF_WIDTH_PC / 1_000.0..MAP_HALF_WIDTH_PC / 1_000.0,
        )?;

    style_mesh(&mut chart, "x [kpc]", "y [kpc]")?;
    let dx = 2.0 * MAP_HALF_WIDTH_PC / GRID_WIDTH as f64;
    let dy = 2.0 * MAP_HALF_WIDTH_PC / GRID_HEIGHT as f64;

    chart.draw_series((0..GRID_WIDTH).flat_map(|ix| {
        (0..GRID_HEIGHT).map(move |iy| {
            let x0 = -MAP_HALF_WIDTH_PC + ix as f64 * dx;
            let y0 = -MAP_HALF_WIDTH_PC + iy as f64 * dy;
            let x = x0 + dx * 0.5;
            let y = y0 + dy * 0.5;
            let position = GalacticPosition {
                radius_pc: x.hypot(y),
                azimuth_rad: y.atan2(x),
                height_pc: selected.height_pc,
            };
            density_cell(
                x0 / 1_000.0,
                y0 / 1_000.0,
                (x0 + dx) / 1_000.0,
                (y0 + dy) / 1_000.0,
                galaxy.stellar_number_density_at(position).total(),
            )
        })
    }))?;

    let selected_x = selected.radius_pc * selected.azimuth_rad.cos() / 1_000.0;
    let selected_y = selected.radius_pc * selected.azimuth_rad.sin() / 1_000.0;
    draw_marker(&mut chart, selected_x, selected_y)?;
    Ok(())
}

fn render_vertical(
    area: &DrawingArea<BitMapBackend<'_>, Shift>,
    galaxy: &GalaxyModel,
    selected: GalacticPosition,
) -> Result<(), Box<dyn Error>> {
    let mut chart = ChartBuilder::on(area)
        .caption(
            "Stellar-number density — vertical section",
            ("sans-serif", 28).into_font().color(&WHITE),
        )
        .margin(22)
        .x_label_area_size(52)
        .y_label_area_size(65)
        .build_cartesian_2d(
            -MAP_HALF_WIDTH_PC / 1_000.0..MAP_HALF_WIDTH_PC / 1_000.0,
            -MAP_HALF_HEIGHT_PC / 1_000.0..MAP_HALF_HEIGHT_PC / 1_000.0,
        )?;

    style_mesh(&mut chart, "signed R [kpc]", "z [kpc]")?;
    draw_vertical_density_grid(&mut chart, galaxy, selected, None)?;

    draw_marker(
        &mut chart,
        selected.radius_pc / 1_000.0,
        selected.height_pc / 1_000.0,
    )?;
    Ok(())
}

fn draw_vertical_density_grid<DB: DrawingBackend>(
    chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    galaxy: &GalaxyModel,
    selected: GalacticPosition,
    field: Option<DensityField>,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    let dx = 2.0 * MAP_HALF_WIDTH_PC / GRID_WIDTH as f64;
    let dz = 2.0 * MAP_HALF_HEIGHT_PC / GRID_HEIGHT as f64;

    chart.draw_series((0..GRID_WIDTH).flat_map(|ix| {
        (0..GRID_HEIGHT).map(move |iz| {
            let x0 = -MAP_HALF_WIDTH_PC + ix as f64 * dx;
            let z0 = -MAP_HALF_HEIGHT_PC + iz as f64 * dz;
            let position = GalacticPosition {
                radius_pc: (x0 + dx * 0.5).abs(),
                azimuth_rad: selected.azimuth_rad,
                height_pc: z0 + dz * 0.5,
            };
            let value = field
                .map(|field| field.value(galaxy, position))
                .unwrap_or_else(|| galaxy.stellar_number_density_at(position).total());
            density_cell(
                x0 / 1_000.0,
                z0 / 1_000.0,
                (x0 + dx) / 1_000.0,
                (z0 + dz) / 1_000.0,
                value,
            )
        })
    }))?;
    Ok(())
}

fn render_density_profiles(
    path: &str,
    galaxy: &GalaxyModel,
    selected: GalacticPosition,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (1800, 800)).into_drawing_area();
    root.fill(&RGBColor(12, 16, 28))?;
    let panels = root.split_evenly((1, 2));

    let mut radial = ChartBuilder::on(&panels[0])
        .caption(
            format!("Radial profile at z = {:.0} pc", selected.height_pc),
            ("sans-serif", 26).into_font().color(&WHITE),
        )
        .margin(22)
        .x_label_area_size(52)
        .y_label_area_size(70)
        .build_cartesian_2d(
            0.1_f64..MAP_HALF_WIDTH_PC / 1_000.0,
            LOG_DENSITY_MIN..LOG_DENSITY_MAX,
        )?;
    style_mesh(&mut radial, "R [kpc]", "log10 stars / pc³")?;
    draw_profile_series(
        &mut radial,
        |coordinate| GalacticPosition {
            radius_pc: coordinate * 1_000.0,
            azimuth_rad: selected.azimuth_rad,
            height_pc: selected.height_pc,
        },
        galaxy,
        0.1,
        MAP_HALF_WIDTH_PC / 1_000.0,
    )?;
    radial
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 17).into_font().color(&WHITE))
        .draw()?;

    let mut vertical = ChartBuilder::on(&panels[1])
        .caption(
            format!(
                "Vertical profile at R = {:.1} kpc",
                selected.radius_pc / 1_000.0
            ),
            ("sans-serif", 26).into_font().color(&WHITE),
        )
        .margin(22)
        .x_label_area_size(52)
        .y_label_area_size(70)
        .build_cartesian_2d(0.0_f64..3.0_f64, LOG_DENSITY_MIN..LOG_DENSITY_MAX)?;
    style_mesh(&mut vertical, "|z| [kpc]", "log10 stars / pc³")?;
    draw_profile_series(
        &mut vertical,
        |coordinate| GalacticPosition {
            radius_pc: selected.radius_pc,
            azimuth_rad: selected.azimuth_rad,
            height_pc: coordinate * 1_000.0,
        },
        galaxy,
        0.0,
        3.0,
    )?;
    vertical
        .configure_series_labels()
        .background_style(RGBAColor(12, 16, 28, 0.82))
        .border_style(WHITE)
        .label_font(("sans-serif", 17).into_font().color(&WHITE))
        .draw()?;

    root.present()?;
    Ok(())
}

fn draw_profile_series<DB, F>(
    chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    position_at: F,
    galaxy: &GalaxyModel,
    start: f64,
    end: f64,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
where
    DB: DrawingBackend,
    F: Fn(f64) -> GalacticPosition + Copy,
{
    let samples = 400;
    let coordinates =
        || (0..=samples).map(|index| start + (end - start) * index as f64 / samples as f64);

    chart
        .draw_series(LineSeries::new(
            coordinates().map(|coordinate| {
                (
                    coordinate,
                    galaxy
                        .stellar_number_density_at(position_at(coordinate))
                        .total()
                        .log10(),
                )
            }),
            WHITE.stroke_width(3),
        ))?
        .label("Total")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 24, y)], WHITE.stroke_width(3)));

    for population in StellarPopulation::ALL {
        let color = population_color(population);
        chart
            .draw_series(LineSeries::new(
                coordinates().map(|coordinate| {
                    let density = galaxy.stellar_number_density_at(position_at(coordinate));
                    (
                        coordinate,
                        density.for_population(population).max(1e-12).log10(),
                    )
                }),
                color.stroke_width(2),
            ))?
            .label(population.label())
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 24, y)], color.stroke_width(2))
            });
    }
    Ok(())
}

fn population_color(population: StellarPopulation) -> RGBColor {
    match population {
        StellarPopulation::ThinDisk => RGBColor(50, 210, 255),
        StellarPopulation::ThickDisk => RGBColor(255, 185, 45),
        StellarPopulation::Halo => RGBColor(190, 110, 255),
    }
}

fn evolution_state_color(state: EvolutionaryState) -> RGBColor {
    match state {
        EvolutionaryState::PreMainSequence => RGBColor(255, 165, 70),
        EvolutionaryState::MainSequence => RGBColor(75, 205, 255),
        EvolutionaryState::SubgiantAndRedGiantBranch => RGBColor(255, 95, 105),
        EvolutionaryState::HeliumIgnitionTransition => RGBColor(255, 180, 75),
        EvolutionaryState::CoreHeliumBurning => RGBColor(255, 215, 85),
        EvolutionaryState::EarlyAsymptoticGiantBranch => RGBColor(255, 145, 65),
        EvolutionaryState::ThermallyPulsingAsymptoticGiantBranch => RGBColor(255, 95, 150),
        EvolutionaryState::AdvancedBurningTrackEnd => RGBColor(225, 70, 70),
        EvolutionaryState::WolfRayet => RGBColor(140, 215, 255),
        EvolutionaryState::PostAsymptoticGiantBranch => RGBColor(170, 130, 255),
        EvolutionaryState::WhiteDwarf => RGBColor(225, 235, 255),
    }
}

fn style_mesh<DB: DrawingBackend>(
    chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    x_label: &str,
    y_label: &str,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .axis_desc_style(("sans-serif", 20).into_font().color(&WHITE))
        .label_style(("sans-serif", 16).into_font().color(&WHITE))
        .bold_line_style(RGBAColor(255, 255, 255, 0.16))
        .light_line_style(RGBAColor(255, 255, 255, 0.06))
        .draw()
}

fn density_cell(x0: f64, y0: f64, x1: f64, y1: f64, density: f64) -> Rectangle<(f64, f64)> {
    let normalized = ((density.max(1e-12).log10() - LOG_DENSITY_MIN)
        / (LOG_DENSITY_MAX - LOG_DENSITY_MIN))
        .clamp(0.0, 1.0);
    Rectangle::new([(x0, y0), (x1, y1)], density_color(normalized).filled())
}

fn density_color(normalized: f64) -> HSLColor {
    HSLColor(250.0 / 360.0 - normalized * 245.0 / 360.0, 0.9, 0.52)
}

fn render_density_legend(
    area: &DrawingArea<BitMapBackend<'_>, Shift>,
) -> Result<(), Box<dyn Error>> {
    let (_, height) = area.dim_in_pixel();
    let top = 100_i32;
    let bottom = height as i32 - 100;
    let steps = bottom - top;

    area.draw(&Text::new(
        "log10 density",
        (20, 45),
        ("sans-serif", 22).into_font().color(&WHITE),
    ))?;
    area.draw(&Text::new(
        "stars / pc³",
        (20, 72),
        ("sans-serif", 18).into_font().color(&WHITE),
    ))?;

    for step in 0..steps {
        let normalized = step as f64 / (steps - 1) as f64;
        let y1 = bottom - step;
        let y0 = y1 - 1;
        area.draw(&Rectangle::new(
            [(20, y0), (72, y1)],
            density_color(normalized).filled(),
        ))?;
    }

    for value in [-4.0_f64, -3.0, -2.0, -1.0, 0.0, 1.0, 1.5] {
        let normalized = (value - LOG_DENSITY_MIN) / (LOG_DENSITY_MAX - LOG_DENSITY_MIN);
        let y = bottom - ((bottom - top) as f64 * normalized) as i32;
        area.draw(&PathElement::new(vec![(72, y), (82, y)], WHITE))?;
        area.draw(&Text::new(
            format!("{value:.1}"),
            (90, y + 6),
            ("sans-serif", 17).into_font().color(&WHITE),
        ))?;
    }
    Ok(())
}

fn draw_marker<DB: DrawingBackend>(
    chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    x: f64,
    y: f64,
) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
    chart.draw_series(std::iter::once(Circle::new(
        (x, y),
        8,
        ShapeStyle::from(&WHITE).filled().stroke_width(2),
    )))?;
    chart.draw_series(std::iter::once(Cross::new(
        (x, y),
        14,
        ShapeStyle::from(&BLACK).stroke_width(3),
    )))?;
    Ok(())
}
