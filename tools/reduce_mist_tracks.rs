//! Reduce official MIST v1.2 `.track.eep` files to the RON subset consumed by star_sim.
//!
//! Usage:
//! `reduce_mist_tracks OUTPUT.ron -2.0=DIR -1.0=DIR 0.0=DIR 0.5=DIR`

use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

const RETAINED_EEPS: [u16; 12] = [1, 202, 353, 354, 355, 453, 454, 605, 631, 707, 808, 1409];

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args().skip(1);
    let output = PathBuf::from(arguments.next().ok_or("missing output path")?);
    let mut sources = Vec::new();
    for argument in arguments {
        let (metallicity, directory) = argument
            .split_once('=')
            .ok_or("sources must use METALLICITY=DIRECTORY")?;
        sources.push((metallicity.parse::<f64>()?, PathBuf::from(directory)));
    }
    if sources.is_empty() {
        return Err("at least one source directory is required".into());
    }
    sources.sort_by(|left, right| left.0.total_cmp(&right.0));

    let mut writer = BufWriter::new(File::create(output)?);
    writeln!(
        writer,
        "// Reduced official MIST v1.2 non-rotating, solar-scaled EEP grid."
    )?;
    writeln!(
        writer,
        "// Retains every tenth EEP plus exact PMS/MS boundary and solar regression nodes."
    )?;
    writeln!(
        writer,
        "// See docs/scientific_sources/stellar_evolution.md."
    )?;
    writeln!(writer, "(")?;
    writeln!(
        writer,
        "    model_version: MistV12NonRotatingSolarScaledThroughWhiteDwarfHandoffV2,"
    )?;
    writeln!(writer, "    tracks: [")?;
    for (metallicity, directory) in sources {
        let mut paths: Vec<_> = fs::read_dir(directory)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "eep"))
            .collect();
        paths.sort();
        for path in paths {
            write_track(&mut writer, metallicity, &path)?;
        }
    }
    writeln!(writer, "    ],")?;
    writeln!(writer, ")")?;
    Ok(())
}

fn write_track(
    writer: &mut impl Write,
    metallicity: f64,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("non-UTF-8 MIST filename")?;
    let encoded_mass = filename
        .strip_suffix("M.track.eep")
        .ok_or("unexpected MIST filename")?;
    let initial_mass_msun = encoded_mass.parse::<u64>()? as f64 / 10_000.0;
    let lines: Vec<String> = BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<_, _>>()?;
    let branch = if lines.iter().any(|line| {
        line.starts_with('#') && line.contains("YES") && line.trim_end().ends_with("high-mass")
    }) {
        "MassiveBurning"
    } else {
        "WhiteDwarfProgenitor"
    };
    let primary_eeps: Vec<u16> = lines
        .iter()
        .find_map(|line| line.strip_prefix("# EEPs:"))
        .ok_or("missing primary EEP header")?
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    writeln!(writer, "        (")?;
    writeln!(
        writer,
        "            initial_mass_msun: {initial_mass_msun:.8},"
    )?;
    writeln!(
        writer,
        "            global_metallicity_mh: {metallicity:.8},"
    )?;
    writeln!(writer, "            branch: {branch},")?;
    writeln!(writer, "            primary_eeps: {primary_eeps:?},")?;
    writeln!(writer, "            points: [")?;

    let mut eep = 0_u16;
    let mut written = 0_usize;
    let mut previous_phase = None;
    for line in lines {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        eep += 1;
        let columns: Vec<_> = line.split_whitespace().collect();
        if columns.len() < 16 {
            return Err(format!("too few columns in {}", path.display()).into());
        }
        let age_gyr = columns[0].parse::<f64>()? / 1.0e9;
        let current_mass_msun = columns[1].parse::<f64>()?;
        let carbon_oxygen_core_mass_msun = columns[4].parse::<f64>()?;
        let log_luminosity = columns[6].parse::<f64>()?;
        let log_temperature = columns[11].parse::<f64>()?;
        let log_radius = columns[13].parse::<f64>()?;
        let log_gravity = columns[14].parse::<f64>()?;
        let phase = columns.last().ok_or("missing MIST phase")?.parse::<f64>()? as i8;
        let phase_changed = previous_phase.is_none_or(|previous| previous != phase);
        previous_phase = Some(phase);
        if eep % 10 != 1 && !RETAINED_EEPS.contains(&eep) && phase != 6 && !phase_changed {
            continue;
        }
        writeln!(
            writer,
            "                (eep: {eep}, age_gyr: {age_gyr:.15e}, current_mass_msun: {current_mass_msun:.15e}, carbon_oxygen_core_mass_msun: {carbon_oxygen_core_mass_msun:.15e}, log10_luminosity_lsun: {log_luminosity:.15e}, log10_effective_temperature_k: {log_temperature:.15e}, log10_radius_rsun: {log_radius:.15e}, surface_gravity_log10_cgs: {log_gravity:.15e}, phase: {phase}),"
        )?;
        written += 1;
    }
    if eep < 454 || written < 2 {
        return Err(format!("{} does not reach TAMS EEP 454", path.display()).into());
    }
    writeln!(writer, "            ],")?;
    writeln!(writer, "        ),")?;
    Ok(())
}
