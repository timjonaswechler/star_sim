use bug_hunter::host::{
    FailureReport,
    recording::{Event, Recording, SessionOutcome},
};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

static LOGICAL_REPL: Mutex<()> = Mutex::new(());

#[test]
fn logical_repl_clicks_the_real_museum_tab_and_shuts_down_on_quit_and_eof() {
    let _guard = LOGICAL_REPL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let quit = run_repl(
        "pause\nstep 2\nresume\nclick menu.tab.museum\npointer click left\nhelp\nquit\n",
        "quit",
    );
    assert!(
        quit.status.success(),
        "quit REPL failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&quit.stdout),
        String::from_utf8_lossy(&quit.stderr)
    );
    let stdout = String::from_utf8(quit.stdout).unwrap();
    assert!(stdout.contains("instance=alpha mode=logical screen=gym paused=false"));
    assert!(stdout.contains("instance=alpha mode=logical screen=gym paused=true"));
    assert!(stdout.contains("last action: step 2"));
    assert!(stdout.contains("last action: resume"));
    assert!(stdout.contains("instance=alpha mode=logical screen=museum paused=false"));
    assert!(stdout.contains("last action: click menu.tab.museum"));
    assert!(stdout.contains("last action: pointer click left"));
    assert!(stdout.contains("keys use case-insensitive names"));
    assert_no_protocol_envelopes(&stdout);

    let eof = run_repl("status\n", "eof");
    assert!(
        eof.status.success(),
        "EOF REPL failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&eof.stdout),
        String::from_utf8_lossy(&eof.stderr)
    );
    let stdout = String::from_utf8(eof.stdout).unwrap();
    assert!(stdout.contains("instance=alpha mode=logical screen=gym paused=false"));
    assert_no_protocol_envelopes(&stdout);
}

#[test]
fn logical_repl_records_context_actions_observations_and_ordered_shutdown() {
    let _guard = LOGICAL_REPL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let artifact_dir = temporary_artifact_dir("recording");
    let mut command = Command::new(env!("CARGO_BIN_EXE_star_sim_debug"));
    command
        .args(["--mode", "logical", "--artifact-dir"])
        .arg(&artifact_dir)
        .args(["--record", "sessions/first.jsonl"])
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("star_sim_debug should start");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(
            b"record stop\npause\nrecord start sessions/second.jsonl\nprovider-secret-value\nresume\nclick menu.tab.museum\nobserve clock\nrecord stop\nrecord start\nobserve clock\nquit\n",
        )
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "recording REPL failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let first = Recording::parse_path(artifact_dir.join("sessions/first.jsonl")).unwrap();
    let second_path = artifact_dir.join("sessions/second.jsonl");
    let second_bytes = fs::read_to_string(&second_path).unwrap();
    assert!(!second_bytes.contains("provider-secret-value"));
    let second = Recording::parse_path(second_path).unwrap();
    let automatic_path = fs::read_dir(artifact_dir.join("recordings"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let automatic = Recording::parse_path(automatic_path).unwrap();

    let Event::SessionStarted { context } = &second.entries[0].event else {
        panic!("second segment must begin with session context")
    };
    assert_eq!(context.session_id, "alpha");
    assert_eq!(context.mode, bug_hunter::RunMode::Logical);
    assert_eq!(context.configuration["profile_id"], "star-sim-debug-v1");
    assert_eq!(context.configuration["mode"], "logical");
    assert_eq!(context.configuration["paused"], true);
    assert!(second.entries.iter().any(|entry| matches!(
        entry.event,
        Event::ControllerAction { ref controller, ref action }
            if controller.origin == "repl" && action["type"] == "pointer"
    )));
    assert!(
        second
            .entries
            .iter()
            .any(|entry| matches!(entry.event, Event::Observation { .. }))
    );
    assert!(matches!(
        second.entries.last().unwrap().event,
        Event::RecordingStopped
    ));
    assert!(automatic.entries[0].sequence > second.entries.last().unwrap().sequence);
    assert!(matches!(
        automatic.entries.last().unwrap().event,
        Event::SessionEnded {
            outcome: SessionOutcome::Completed
        }
    ));
    assert!(second.entries[0].sequence > first.entries.last().unwrap().sequence);
    fs::remove_dir_all(artifact_dir).ok();
}

#[test]
fn explicit_recording_path_is_retained_in_failure_metadata() {
    let _guard = LOGICAL_REPL
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let artifact_dir = temporary_artifact_dir("recording-failure");
    let relative = PathBuf::from("sessions/existing.jsonl");
    fs::create_dir_all(artifact_dir.join("sessions")).unwrap();
    fs::write(artifact_dir.join(&relative), "already here").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_star_sim_debug"))
        .args(["--mode", "logical", "--artifact-dir"])
        .arg(&artifact_dir)
        .arg("--record")
        .arg(&relative)
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let failure = FailureReport::load(artifact_dir.join("failure.json")).unwrap();
    assert_eq!(failure.record_path, Some(artifact_dir.join(relative)));
    fs::remove_dir_all(artifact_dir).ok();
}

fn run_repl(input: &str, name: &str) -> std::process::Output {
    let artifact_dir = temporary_artifact_dir(name);
    let mut command = Command::new(env!("CARGO_BIN_EXE_star_sim_debug"));
    command
        .args(["--mode", "logical", "--artifact-dir"])
        .arg(&artifact_dir)
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("star_sim_debug should start");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .expect("REPL input should be written");
    let output = child.wait_with_output().expect("REPL should finish");
    fs::remove_dir_all(artifact_dir).ok();
    output
}

fn assert_no_protocol_envelopes(stdout: &str) {
    for fragment in [
        "\"sequence\"",
        "\"version\"",
        "\"type\":\"ready\"",
        "\"status\":\"completed\"",
    ] {
        assert!(
            !stdout.contains(fragment),
            "stdout leaked protocol fragment {fragment:?}:\n{stdout}"
        );
    }
}

fn temporary_artifact_dir(name: &str) -> PathBuf {
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "star-sim-debug-{name}-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ))
}
