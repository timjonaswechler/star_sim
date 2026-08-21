mod controller;
mod repl;
mod script;

use automation_control::driver::{RecentLogs, ReportConfig, github, recording};
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
const CANVAS: SurfaceSize = SurfaceSize::new(640, 360);

#[derive(Debug, Parser)]
#[command(
    name = "star_sim_debug",
    about = "Start and control one isolated Star Sim session"
)]
struct Cli {
    /// Controlled Session execution mode. Overrides a Session Script's configured mode.
    #[arg(long, value_enum)]
    mode: Option<ModeArgument>,

    /// Root for session diagnostics and artifacts.
    #[arg(long, default_value = DEFAULT_ARTIFACT_DIR)]
    artifact_dir: PathBuf,

    /// Start Session Recording at this artifact-root-relative JSONL path.
    #[arg(long)]
    record: Option<PathBuf>,

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
    /// Run a human-authored Session Script in a fresh Controlled Session.
    Run { script: PathBuf },

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
        std::process::exit(error.exit_code());
    }
}

#[derive(Debug)]
enum ExecuteError {
    General(String),
    Script(script::Error),
}

impl ExecuteError {
    const fn exit_code(&self) -> i32 {
        match self {
            Self::General(_) => 1,
            Self::Script(error) => error.exit_code(),
        }
    }
}

impl std::fmt::Display for ExecuteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::General(message) => formatter.write_str(message),
            Self::Script(error) => error.fmt(formatter),
        }
    }
}

fn execute(cli: Cli) -> Result<(), ExecuteError> {
    match cli.command {
        Some(CliCommand::Run { script }) => run_script(
            script,
            cli.mode.map(Into::into),
            cli.artifact_dir,
            cli.record,
        ),
        Some(CliCommand::Report {
            artifact_dir,
            create,
        }) => report(artifact_dir, create).map_err(ExecuteError::General),
        None => controlled_repl(
            cli.mode.unwrap_or(ModeArgument::Rendered).into(),
            cli.artifact_dir,
            cli.record,
        )
        .map_err(ExecuteError::General),
    }
}

fn controlled_repl(
    mode: Mode,
    artifact_dir: PathBuf,
    record: Option<PathBuf>,
) -> Result<(), String> {
    let recent_logs = RecentLogs::default();
    let interrupted = Arc::new(AtomicBool::new(false));
    let signal = Arc::clone(&interrupted);
    ctrlc::set_handler(move || signal.store(true, Ordering::SeqCst))
        .map_err(|error| format!("could not install Ctrl-C handler: {error}"))?;

    let diagnostic_record_path = record
        .as_ref()
        .and_then(|path| recording::path_below_artifact_root(&artifact_dir, path).ok());
    let result = ControllerSession::start(
        mode,
        CANVAS,
        artifact_dir.clone(),
        record,
        recent_logs.clone(),
        recording::Controller::new("repl"),
    )
    .and_then(|session| repl::run(session, interrupted))
    .map_err(|error| error.to_string());

    let cli_error = result.as_ref().err().map(String::as_str);
    persist_failure(
        &recent_logs,
        &artifact_dir,
        cli_error,
        diagnostic_record_path.as_deref(),
    );
    result
}

fn run_script(
    path: PathBuf,
    mode: Option<Mode>,
    artifact_dir: PathBuf,
    record: Option<PathBuf>,
) -> Result<(), ExecuteError> {
    let recent_logs = RecentLogs::default();
    let diagnostic_record_path = record
        .as_ref()
        .and_then(|path| recording::path_below_artifact_root(&artifact_dir, path).ok());
    let result = script::run(
        &path,
        mode,
        CANVAS,
        artifact_dir.clone(),
        record,
        recent_logs.clone(),
    );
    let cli_error = result.as_ref().err().map(ToString::to_string);
    persist_failure(
        &recent_logs,
        &artifact_dir,
        cli_error.as_deref(),
        diagnostic_record_path.as_deref(),
    );
    let summary = result.map_err(ExecuteError::Script)?;
    println!(
        "Session Script passed: {} completed, {} skipped, mode={}",
        summary.completed, summary.skipped, summary.mode
    );
    Ok(())
}

fn persist_failure(
    recent_logs: &RecentLogs,
    artifact_dir: &std::path::Path,
    cli_error: Option<&str>,
    record_path: Option<&std::path::Path>,
) {
    if (cli_error.is_some() || recent_logs.failure().is_some())
        && let Err(error) =
            recent_logs.persist_failure_artifacts(artifact_dir, cli_error, record_path)
    {
        eprintln!("warning: could not save failure artifacts: {error}");
    }
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
        assert_eq!(cli.mode, None);
        assert!(cli.command.is_none());
        assert_eq!(cli.artifact_dir, PathBuf::from(DEFAULT_ARTIFACT_DIR));
        assert_eq!(cli.record, None);
    }

    #[test]
    fn logical_mode_is_a_host_option_not_a_scenario() {
        let cli = Cli::try_parse_from(["star_sim_debug", "--mode", "logical"]).unwrap();
        assert_eq!(cli.mode, Some(ModeArgument::Logical));
        assert!(cli.command.is_none());
        assert!(Cli::try_parse_from(["star_sim_debug", "logical"]).is_err());
    }

    #[test]
    fn record_is_a_host_option() {
        let cli = Cli::try_parse_from([
            "star_sim_debug",
            "--mode",
            "logical",
            "--record",
            "records/logical.jsonl",
        ])
        .unwrap();
        assert_eq!(cli.record, Some(PathBuf::from("records/logical.jsonl")));
        assert!(cli.command.is_none());
    }

    #[test]
    fn run_accepts_a_session_script_path() {
        let cli = Cli::try_parse_from(["star_sim_debug", "run", "sessions/museum.json"]).unwrap();
        assert!(matches!(
            cli.command,
            Some(CliCommand::Run { script })
                if script == std::path::Path::new("sessions/museum.json")
        ));
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
