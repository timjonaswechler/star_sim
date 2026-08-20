use automation_control::{
    Command, RunMode,
    driver::{Config, LaunchSpec, RecentLogs, RunOptions, Session, SessionConfig, SessionOptions},
    observation::{Projection, Request as ObservationRequest, Selector},
};
use serde_json::json;
use std::path::PathBuf;

const DEFAULT_ARTIFACT_DIR: &str = "artifacts/debug-ci";

/// Runs a small protocol-v2 compatibility probe against the configured application.
///
/// The public REPL, scripts, replay, and recording orchestration are follow-up work. This keeps
/// the existing Debug Host binary compiling while the Controlled Session interface is rebuilt.
pub fn execute(options: RunOptions, config: Config) -> Result<(), String> {
    let RunOptions {
        scenario,
        artifact_dir,
        record,
        ..
    } = options;
    let artifact_dir = artifact_dir.unwrap_or_else(|| PathBuf::from(DEFAULT_ARTIFACT_DIR));
    let recent_logs = RecentLogs::default();
    let result = run_probe(
        &config.application,
        &config.session,
        scenario.as_str(),
        artifact_dir.clone(),
        record.clone(),
        recent_logs.clone(),
    );
    let cli_error = result.as_ref().err().map(String::as_str);
    if cli_error.is_some() || recent_logs.failure().is_some() {
        if let Err(error) =
            recent_logs.persist_failure_artifacts(&artifact_dir, cli_error, record.as_deref())
        {
            eprintln!("warning: could not save failure artifacts: {error}");
        }
    }
    result
}

fn run_probe(
    application: &LaunchSpec,
    session_config: &SessionConfig,
    requested_mode: &str,
    artifact_dir: PathBuf,
    record: Option<PathBuf>,
    recent_logs: RecentLogs,
) -> Result<(), String> {
    let expected_mode = match requested_mode {
        "rendered" => RunMode::Rendered,
        other => {
            return Err(format!(
                "mode {other:?} is unavailable in this compatibility probe; expected rendered"
            ));
        }
    };
    let mut session = Session::spawn(
        application,
        SessionOptions::from_config(session_config, record, recent_logs, artifact_dir),
    )
    .map_err(|error| error.to_string())?;
    let ready = session.ready().map_err(|error| error.to_string())?;
    if ready.mode != expected_mode {
        return Err(format!(
            "requested {expected_mode:?}, child reported {:?}",
            ready.mode
        ));
    }
    let observation = session
        .request(Command::Observe(ObservationRequest::new(
            Selector::Targets,
            Projection::Summary,
        )))
        .map_err(|error| error.to_string())?;
    session.shutdown().map_err(|error| error.to_string())?;
    println!(
        "{}",
        json!({
            "status": "passed",
            "mode": ready.mode,
            "ready": ready,
            "targets": observation.result.unwrap_or_default(),
        })
    );
    Ok(())
}
