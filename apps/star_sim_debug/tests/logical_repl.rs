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
        "pause\nstep 2\nresume\nclick menu.tab.museum\nquit\n",
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
