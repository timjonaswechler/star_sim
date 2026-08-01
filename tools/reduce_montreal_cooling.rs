//! Reduce the official Bédard et al. (2020) Montréal thick-H cooling sequences.
//!
//! Usage: `reduce_montreal_cooling INPUT_DIR OUTPUT.ron`

use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

const SOLAR_RADIUS_CM: f64 = 6.957e10;
const SOLAR_LUMINOSITY_ERG_PER_S: f64 = 3.828e33;
const YEARS_PER_GYR: f64 = 1.0e9;

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args().skip(1);
    let input = PathBuf::from(arguments.next().ok_or("missing input directory")?);
    let output = PathBuf::from(arguments.next().ok_or("missing output path")?);
    if arguments.next().is_some() {
        return Err("expected INPUT_DIR OUTPUT.ron".into());
    }

    let mut paths: Vec<_> = fs::read_dir(input)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("seq_") && name.ends_with("_thick.txt"))
        })
        .collect();
    paths.sort();
    if paths.len() != 23 {
        return Err(format!("expected 23 thick-H sequences, found {}", paths.len()).into());
    }

    let mut writer = BufWriter::new(File::create(output)?);
    writeln!(
        writer,
        "// Reduced official Montréal white-dwarf cooling grid."
    )?;
    writeln!(
        writer,
        "// Bédard et al. (2020), homogeneous C/O core, q_He=1e-2, q_H=1e-4."
    )?;
    writeln!(
        writer,
        "// See docs/scientific_sources/white_dwarf_cooling.md."
    )?;
    writeln!(writer, "(")?;
    writeln!(
        writer,
        "    model_version: MontrealBedard2020ThickHydrogenV1,"
    )?;
    writeln!(writer, "    sequences: [")?;
    for path in paths {
        write_sequence(&mut writer, &path)?;
    }
    writeln!(writer, "    ],")?;
    writeln!(writer, ")")?;
    Ok(())
}

fn write_sequence(writer: &mut impl Write, path: &Path) -> Result<(), Box<dyn Error>> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("non-UTF-8 sequence filename")?;
    let encoded_mass = name
        .strip_prefix("seq_")
        .and_then(|name| name.strip_suffix("_thick.txt"))
        .ok_or("unexpected sequence filename")?;
    let mass_msun = encoded_mass.parse::<u64>()? as f64 / 100.0;
    let lines: Vec<_> = BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<_, _>>()?;

    writeln!(writer, "        (")?;
    writeln!(writer, "            mass_msun: {mass_msun:.8},")?;
    writeln!(writer, "            points: [")?;
    for line in lines.iter().skip(5).step_by(3) {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 6 {
            return Err(format!("unexpected data row in {}: {line}", path.display()).into());
        }
        let effective_temperature_k = fields[1].parse::<f64>()?;
        let surface_gravity_log10_cgs = fields[2].parse::<f64>()?;
        let radius_rsun = fields[3].parse::<f64>()? / SOLAR_RADIUS_CM;
        let cooling_age_gyr = fields[4].parse::<f64>()? / YEARS_PER_GYR;
        let luminosity_lsun = fields[5].parse::<f64>()? / SOLAR_LUMINOSITY_ERG_PER_S;
        writeln!(
            writer,
            "                (cooling_age_gyr: {cooling_age_gyr:.15e}, luminosity_lsun: {luminosity_lsun:.15e}, radius_rsun: {radius_rsun:.15e}, effective_temperature_k: {effective_temperature_k:.15e}, surface_gravity_log10_cgs: {surface_gravity_log10_cgs:.15e}),"
        )?;
    }
    writeln!(writer, "            ],")?;
    writeln!(writer, "        ),")?;
    Ok(())
}
