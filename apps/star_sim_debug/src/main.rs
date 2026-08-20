mod controller;
mod repl;

use automation_control::driver::{RecentLogs, ReportConfig, github};
use clap::{Parser, Subcommand, ValueEnum};
use controller::{ControllerSession, Mode, SurfaceSize};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

const DEFAULT_ARTIFACT_DIR: &str = "artifacts/star-sim-debug";
const SURFACE_WIDTH: u32 = 640;
const SURFACE_HEIGHT: u32 = 360;

#[derive(Debug, Parser)]
#[command(
    name = "star_sim_debug",
    about = "Start and control one isolated Star Sim session"
)]
struct Cli {
    /// Controlled Session execution mode.
    #[arg(long, value_enum, default_value_t = ModeArgument::Rendered)]
    mode: ModeArgument,

    /// Root for session diagnostics and artifacts.
    #[arg(long, default_value = DEFAULT_ARTIFACT_DIR)]
    artifact_dir: PathBuf,

    #[command(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum ModeArgument {
    Logical,
    Rendered,
}

impl From<ModeArgument> for Mode {
    fn from(value: ModeArgument) -> Self {
        match value {
            ModeArgument::Logical => Self::Logical,
            ModeArgument::Rendered => Self::Rendered,
        }
    }
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    /// Draft or publish a report from an existing failure artifact directory.
    Report {
        artifact_dir: PathBuf,
        #[arg(long)]
        create: bool,
    },
}

fn main() {
    if let Err(error) = execute(Cli::parse()) {
        eprintln!("star_sim_debug: {error}");
        std::process::exit(1);
    }
}

fn execute(cli: Cli) -> Result<(), String> {
    match cli.command {
        Some(CliCommand::Report {
            artifact_dir,
            create,
        }) => report(artifact_dir, create),
        None => controlled_repl(cli.mode.into(), cli.artifact_dir),
    }
}

fn controlled_repl(mode: Mode, artifact_dir: PathBuf) -> Result<(), String> {
    let recent_logs = RecentLogs::default();
    let interrupted = Arc::new(AtomicBool::new(false));
    let signal = Arc::clone(&interrupted);
    ctrlc::set_handler(move || signal.store(true, Ordering::SeqCst))
        .map_err(|error| format!("could not install Ctrl-C handler: {error}"))?;

    let result = ControllerSession::start(
        mode,
        SurfaceSize::new(SURFACE_WIDTH, SURFACE_HEIGHT),
        artifact_dir.clone(),
        recent_logs.clone(),
    )
    .and_then(|session| repl::run(session, interrupted))
    .map_err(|error| error.to_string());

    let cli_error = result.as_ref().err().map(String::as_str);
    if (cli_error.is_some() || recent_logs.failure().is_some())
        && let Err(error) = recent_logs.persist_failure_artifacts(&artifact_dir, cli_error, None)
    {
        eprintln!("warning: could not save failure artifacts: {error}");
    }
    result
}

fn report(artifact_dir: PathBuf, create: bool) -> Result<(), String> {
    let config = ReportConfig {
        generated_by: Some("star_sim_debug report".into()),
    };
    let report =
        github::Report::prepare(&artifact_dir, &config).map_err(|error| error.to_string())?;
    let outcome = if create {
        report.publish().map_err(|error| error.to_string())?
    } else {
        report.draft()
    };
    println!(
        "{}",
        serde_json::to_string(&outcome).map_err(|error| error.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_subcommand_selects_the_rendered_repl() {
        let cli = Cli::try_parse_from(["star_sim_debug"]).unwrap();
        assert_eq!(cli.mode, ModeArgument::Rendered);
        assert!(cli.command.is_none());
        assert_eq!(cli.artifact_dir, PathBuf::from(DEFAULT_ARTIFACT_DIR));
    }

    #[test]
    fn logical_mode_is_a_host_option_not_a_scenario() {
        let cli = Cli::try_parse_from(["star_sim_debug", "--mode", "logical"]).unwrap();
        assert_eq!(cli.mode, ModeArgument::Logical);
        assert!(cli.command.is_none());
        assert!(Cli::try_parse_from(["star_sim_debug", "logical"]).is_err());
    }

    #[test]
    fn report_remains_an_explicit_subcommand() {
        let cli =
            Cli::try_parse_from(["star_sim_debug", "report", "artifacts/failure", "--create"])
                .unwrap();
        assert!(matches!(
            cli.command,
            Some(CliCommand::Report {
                artifact_dir,
                create: true,
            }) if artifact_dir == std::path::Path::new("artifacts/failure")
        ));
    }
}
