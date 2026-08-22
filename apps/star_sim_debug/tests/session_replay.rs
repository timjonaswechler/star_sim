use bug_hunter::host::recording::{Event, Recording};
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

static SESSION_REPLAY: Mutex<()> = Mutex::new(());

#[test]
fn logical_recording_replays_without_a_provider_and_can_record_the_replay() {
    let _guard = SESSION_REPLAY
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let artifact_dir = temporary_artifact_dir("logical");
    let source = artifact_dir.join("source/session.jsonl");
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../sessions/museum.json");
    let recorded = debug_command(&artifact_dir)
        .args(["--record", "source/session.jsonl", "run"])
        .arg(script)
        .output()
        .unwrap();
    assert_success(&recorded);
    let source_before = fs::read(&source).unwrap();

    let replayed = debug_command(&artifact_dir)
        .env("HTTP_PROXY", "http://127.0.0.1:1")
        .env("HTTPS_PROXY", "http://127.0.0.1:1")
        .env(
            "OPENAI_API_KEY",
            "replay-must-not-read-provider-credentials",
        )
        .args(["--record", "replays/session.jsonl", "replay"])
        .arg(&source)
        .output()
        .unwrap();

    assert_success(&replayed);
    assert!(String::from_utf8_lossy(&replayed.stdout).contains("Session Replay passed"));
    assert_eq!(fs::read(&source).unwrap(), source_before);
    let source_recording = Recording::parse_path(&source).unwrap();
    let target = Recording::parse_path(artifact_dir.join("replays/session.jsonl")).unwrap();
    assert!(target.entries.iter().any(|entry| matches!(
        entry.event,
        Event::ControllerAction { ref controller, .. } if controller.origin == "replay"
    )));
    let recorded_actions = |recording: &Recording| {
        recording
            .entries
            .iter()
            .filter_map(|entry| match &entry.event {
                Event::ControllerAction { action, .. } => Some(action.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        recorded_actions(&target),
        recorded_actions(&source_recording)
    );
    let result: Value =
        serde_json::from_slice(&fs::read(artifact_dir.join("replay-result.json")).unwrap())
            .unwrap();
    assert_eq!(result["outcome"], "passed");

    let report = debug_command(&artifact_dir)
        .args(["report", "--json"])
        .arg(&artifact_dir)
        .output()
        .unwrap();
    assert_success(&report);
    let report: Value = serde_json::from_slice(&report.stdout).unwrap();
    assert_eq!(report["replay"]["outcome"], "passed");
    for controller in ["script", "replay"] {
        assert!(
            report["sessions"]
                .as_array()
                .unwrap()
                .iter()
                .any(|session| {
                    session["controllers"]
                        .as_array()
                        .is_some_and(|controllers| {
                            controllers.iter().any(|actual| actual == controller)
                        })
                })
        );
    }
    fs::remove_dir_all(artifact_dir).ok();
}

#[test]
fn changed_stable_observation_reports_the_recording_sequence() {
    let _guard = SESSION_REPLAY
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let artifact_dir = temporary_artifact_dir("mismatch");
    let source = artifact_dir.join("source/session.jsonl");
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../sessions/museum.json");
    let recorded = debug_command(&artifact_dir)
        .args(["--record", "source/session.jsonl", "run"])
        .arg(script)
        .output()
        .unwrap();
    assert_success(&recorded);

    let mut changed = false;
    let modified = fs::read_to_string(&source)
        .unwrap()
        .lines()
        .map(|line| {
            let mut entry: Value = serde_json::from_str(line).unwrap();
            if !changed && entry["type"] == "observation" {
                entry["result"]["recorded_only_change"] = Value::Bool(true);
                changed = true;
            }
            serde_json::to_string(&entry).unwrap()
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let changed_source = artifact_dir.join("source/changed.jsonl");
    fs::write(&changed_source, modified).unwrap();

    let replayed = debug_command(&artifact_dir)
        .arg("replay")
        .arg(changed_source)
        .output()
        .unwrap();

    assert_eq!(replayed.status.code(), Some(6));
    let stderr = String::from_utf8_lossy(&replayed.stderr);
    assert!(stderr.contains("Replay result differs"), "{stderr}");
    assert!(stderr.contains("recording sequence:"), "{stderr}");
    assert!(stderr.contains("expected:"), "{stderr}");
    assert!(stderr.contains("actual:"), "{stderr}");

    let unsupported_source = artifact_dir.join("source/unsupported.jsonl");
    fs::write(
        &unsupported_source,
        "{\"version\":99,\"sequence\":1,\"type\":\"future_event\"}\n",
    )
    .unwrap();
    let unsupported = debug_command(&artifact_dir)
        .arg("replay")
        .arg(&unsupported_source)
        .output()
        .unwrap();
    assert_eq!(unsupported.status.code(), Some(6));
    let result: Value =
        serde_json::from_slice(&fs::read(artifact_dir.join("replay-result.json")).unwrap())
            .unwrap();
    assert_eq!(
        result["source"],
        unsupported_source.to_string_lossy().as_ref()
    );
    assert_eq!(result["outcome"], "failed");
    assert!(
        result["error"]
            .as_str()
            .unwrap()
            .contains("unsupported recording version")
    );
    fs::remove_dir_all(artifact_dir).ok();
}

fn debug_command(artifact_dir: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_star_sim_debug"));
    command
        .arg("--artifact-dir")
        .arg(artifact_dir)
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET");
    command
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "command failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn temporary_artifact_dir(name: &str) -> PathBuf {
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "star-sim-debug-replay-{name}-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ))
}
