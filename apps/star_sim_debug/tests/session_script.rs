use automation_control::driver::recording::{Event, Recording, SessionOutcome};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

static SESSION_SCRIPT: Mutex<()> = Mutex::new(());

#[test]
fn museum_script_drives_a_fresh_logical_session_and_uses_session_recording() {
    let _guard = SESSION_SCRIPT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let artifact_dir = temporary_artifact_dir("museum");
    let output = run_script(
        &museum_script(),
        &artifact_dir,
        &["--record", "recordings/museum.jsonl"],
    );
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Session Script passed: 3 completed, 1 skipped, mode=logical"));

    let recording = Recording::parse_path(artifact_dir.join("recordings/museum.jsonl")).unwrap();
    assert!(recording.entries.iter().any(|entry| matches!(
        entry.event,
        Event::ControllerAction { ref controller, ref action }
            if controller.origin == "script" && action["type"] == "pointer"
    )));
    assert!(
        recording
            .entries
            .iter()
            .any(|entry| matches!(entry.event, Event::Observation { .. }))
    );
    assert!(matches!(
        recording.entries.last().unwrap().event,
        Event::SessionEnded {
            outcome: SessionOutcome::Completed
        }
    ));
    fs::remove_dir_all(artifact_dir).ok();
}

#[test]
fn script_failures_have_distinct_exit_codes_and_step_context() {
    let _guard = SESSION_SCRIPT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let invalid_dir = temporary_artifact_dir("invalid");
    fs::create_dir_all(&invalid_dir).unwrap();
    let invalid_path = invalid_dir.join("invalid.json");
    fs::write(
        &invalid_path,
        r#"{"version":99,"session":{"mode":"logical"},"steps":[{"type":"text","text":"hello"}]}"#,
    )
    .unwrap();
    let invalid = run_script(
        &invalid_path,
        &invalid_dir,
        &["--record", "recordings/invalid.jsonl"],
    );
    assert_eq!(invalid.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("$.version"));
    let invalid_recording =
        Recording::parse_path(invalid_dir.join("recordings/invalid.jsonl")).unwrap();
    assert!(invalid_recording.entries.iter().any(|entry| matches!(
        entry.event,
        Event::Error { ref kind, .. } if kind == "invalid_session_script"
    )));
    assert!(matches!(
        invalid_recording.entries.last().unwrap().event,
        Event::SessionEnded {
            outcome: SessionOutcome::Completed
        }
    ));

    let timeout_dir = temporary_artifact_dir("timeout");
    let timeout_path = write_script(
        &timeout_dir,
        "timeout.json",
        json!({
            "version": 1,
            "session": {"mode": "logical"},
            "steps": [{
                "type": "wait",
                "condition": {"type": "screen", "equals": "zoo"},
                "max_frames": 1
            }]
        }),
    );
    let timeout = run_script(&timeout_path, &timeout_dir, &[]);
    assert_eq!(timeout.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&timeout.stderr);
    assert!(stderr.contains("step 1 ($.steps[0])"));
    assert!(stderr.contains("last stable observation (screen)"));

    let action_dir = temporary_artifact_dir("action");
    let action_path = write_script(
        &action_dir,
        "action.json",
        json!({
            "version": 1,
            "session": {"mode": "logical"},
            "steps": [{"type": "click", "target": "menu.tab.space"}]
        }),
    );
    let action = run_script(&action_path, &action_dir, &[]);
    assert_eq!(action.status.code(), Some(4));
    assert!(String::from_utf8_lossy(&action.stderr).contains("step 1 ($.steps[0])"));

    let expectation_dir = temporary_artifact_dir("expectation");
    let expectation_path = write_script(
        &expectation_dir,
        "expectation.json",
        json!({
            "version": 1,
            "session": {"mode": "logical"},
            "steps": [{
                "type": "expect",
                "condition": {"type": "screen", "equals": "museum"}
            }]
        }),
    );
    let expectation = run_script(&expectation_path, &expectation_dir, &[]);
    assert_eq!(expectation.status.code(), Some(5));
    let stderr = String::from_utf8_lossy(&expectation.stderr);
    assert!(stderr.contains("expected: {\"active_screen\":\"museum\"}"));
    assert!(stderr.contains("actual: {\"active_screen\":\"gym\"}"));
    assert!(stderr.contains("step 1 ($.steps[0])"));

    for directory in [invalid_dir, timeout_dir, action_dir, expectation_dir] {
        fs::remove_dir_all(directory).ok();
    }
}

fn museum_script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../sessions/museum.json")
}

fn write_script(directory: &Path, name: &str, value: serde_json::Value) -> PathBuf {
    fs::create_dir_all(directory).unwrap();
    let path = directory.join(name);
    fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    path
}

fn run_script(path: &Path, artifact_dir: &Path, host_arguments: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_star_sim_debug"));
    command
        .arg("--artifact-dir")
        .arg(artifact_dir)
        .args(host_arguments)
        .arg("run")
        .arg(path)
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .output()
        .unwrap()
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "Session Script failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn temporary_artifact_dir(name: &str) -> PathBuf {
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "star-sim-debug-script-{name}-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ))
}
