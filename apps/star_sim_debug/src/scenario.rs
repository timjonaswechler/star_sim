use automation_control::{
    Command, RunMode, ScreenshotSource, WaitCondition,
    driver::{
        Config, LaunchSpec, RecentLogs, RunOptions, Session, SessionConfig, SessionOptions,
        response_path, validate_png,
    },
};
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

const DEFAULT_ARTIFACT_DIR: &str = "artifacts/debug-ci";
const LOGICAL_REQUIRED_CAPABILITIES: &[&str] =
    &["inspect_ui", "click", "wait_until", "inspect_run"];
const LOGICAL_TAB_TARGET: &str = "menu.tab.museum";
const LOGICAL_SCREEN: &str = "museum";
const VISUAL_REQUIRED_CAPABILITIES: &[&str] = &["screenshot"];
const VISUAL_WINDOW_TARGET: &str = "window.primary";
const VISUAL_WINDOW_PATH: &str = "window.png";
const VISUAL_WINDOW_SIZE: [u32; 2] = [640, 360];

/// Runs a star-sim scenario against the configured application.
///
/// The launch configuration is generic, while target IDs, protocol commands, artifact paths,
/// image dimensions, and result assertions remain owned by this application-specific module.
pub fn execute(options: RunOptions, config: Config) -> Result<(), String> {
    let RunOptions {
        scenario,
        artifact_dir,
        record,
        ..
    } = options;
    let artifact_dir = artifact_dir.unwrap_or_else(|| PathBuf::from(DEFAULT_ARTIFACT_DIR));
    let recent_logs = RecentLogs::default();
    let result = match scenario.as_str() {
        "logical" => run_logical(
            &config.application,
            &config.session,
            artifact_dir.clone(),
            record.clone(),
            recent_logs.clone(),
        ),
        "visual" => run_visual(
            &config.application,
            &config.session,
            artifact_dir.clone(),
            record.clone(),
            recent_logs.clone(),
        ),
        scenario => {
            return Err(format!(
                "unknown scenario {scenario:?}; expected logical or visual"
            ));
        }
    };
    let cli_error = result.as_ref().err().map(String::as_str);
    if cli_error.is_some() || recent_logs.failure().is_some() {
        match recent_logs.persist_failure_artifacts(&artifact_dir, cli_error, record.as_deref()) {
            Ok(artifacts) => {
                eprintln!("recent log: {}", artifacts.recent_log.display());
                eprintln!("failure metadata: {}", artifacts.failure_report.display());
            }
            Err(error) => eprintln!("warning: could not save failure artifacts: {error}"),
        }
    }
    result
}

fn run_logical(
    application: &LaunchSpec,
    session_config: &SessionConfig,
    artifact_dir: PathBuf,
    record: Option<PathBuf>,
    recent_logs: RecentLogs,
) -> Result<(), String> {
    let mut client = Session::spawn(
        application,
        SessionOptions::from_config(session_config, record, recent_logs, artifact_dir),
    )
    .map_err(|error| error.to_string())?;
    let ready = client
        .ready(LOGICAL_REQUIRED_CAPABILITIES)
        .map_err(|error| error.to_string())?;
    client
        .request("ui", Command::InspectUi)
        .map_err(|error| error.to_string())?;
    client
        .request(
            "click-museum",
            Command::Click {
                target: LOGICAL_TAB_TARGET.into(),
            },
        )
        .map_err(|error| error.to_string())?;
    client
        .request(
            "wait-screen",
            Command::WaitUntil {
                condition: WaitCondition::ActiveScreen {
                    screen: LOGICAL_SCREEN.into(),
                },
                timeout_frames: 10,
            },
        )
        .map_err(|error| error.to_string())?;
    let state = client
        .request("state", Command::InspectRun)
        .map_err(|error| error.to_string())?;
    client.shutdown().map_err(|error| error.to_string())?;
    println!(
        "{}",
        json!({
            "status": "passed",
            "mode": "logical",
            "ready": ready,
            "state": state.result.unwrap_or(Value::Null),
        })
    );
    Ok(())
}

fn run_visual(
    application: &LaunchSpec,
    session_config: &SessionConfig,
    artifact_dir: PathBuf,
    record: Option<PathBuf>,
    recent_logs: RecentLogs,
) -> Result<(), String> {
    fs::create_dir_all(&artifact_dir).map_err(|error| error.to_string())?;
    let artifact_dir = artifact_dir
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let mut client = Session::spawn(
        application,
        SessionOptions::from_config(session_config, record, recent_logs, artifact_dir),
    )
    .map_err(|error| error.to_string())?;
    let ready = client
        .ready(VISUAL_REQUIRED_CAPABILITIES)
        .map_err(|error| error.to_string())?;
    if ready.mode != RunMode::Rendered {
        return Err(format!(
            "visual scenario requires rendered mode, got {:?}",
            ready.mode
        ));
    }
    let window = client
        .request(
            "window",
            Command::Screenshot {
                source: ScreenshotSource::Window {
                    target: VISUAL_WINDOW_TARGET.into(),
                },
                path: Some(VISUAL_WINDOW_PATH.into()),
                overwrite: false,
            },
        )
        .map_err(|error| error.to_string())?;
    client.shutdown().map_err(|error| error.to_string())?;
    let window_path = response_path(&window)?;
    validate_png(&window_path, VISUAL_WINDOW_SIZE).map_err(|error| error.to_string())?;
    println!(
        "{}",
        json!({
            "status": "passed",
            "mode": "visual",
            "window_png": window_path,
        })
    );
    Ok(())
}
