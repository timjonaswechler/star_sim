mod report;

use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

const TIMEOUT: Duration = Duration::from_secs(60);
const RECENT_LOG_CAPACITY: usize = 50;

#[derive(Clone, Debug, PartialEq, Eq)]
struct FailureHeadline {
    kind: &'static str,
    message: String,
}

#[derive(Default)]
struct RecentLogState {
    lines: VecDeque<String>,
    failure: Option<FailureHeadline>,
    pending_panic_location: Option<String>,
}

#[derive(Clone, Default)]
struct RecentLogs(Arc<Mutex<RecentLogState>>);

impl RecentLogs {
    fn push(&self, line: String) {
        let mut state = self.0.lock().expect("recent log mutex poisoned");
        if let Some(location) = state.pending_panic_location.take() {
            let message = line.trim();
            if !message.is_empty()
                && message != "stack backtrace:"
                && !message.starts_with("note: run with")
            {
                state.failure = Some(FailureHeadline {
                    kind: "panic",
                    message: format!("{message} ({location})"),
                });
            }
        }
        if line.contains("panicked at") {
            state.failure = Some(FailureHeadline {
                kind: "panic",
                message: line.clone(),
            });
            state.pending_panic_location = Some(line.clone());
        } else if state
            .failure
            .as_ref()
            .is_none_or(|failure| failure.kind != "panic")
            && let Some(message) = bevy_error_message(&line)
        {
            state.failure = Some(FailureHeadline {
                kind: "error",
                message: message.to_owned(),
            });
        }
        if state.lines.len() == RECENT_LOG_CAPACITY {
            state.lines.pop_front();
        }
        state.lines.push_back(line);
    }

    fn snapshot(&self) -> Vec<String> {
        self.0
            .lock()
            .expect("recent log mutex poisoned")
            .lines
            .iter()
            .cloned()
            .collect()
    }

    fn failure(&self) -> Option<FailureHeadline> {
        self.0
            .lock()
            .expect("recent log mutex poisoned")
            .failure
            .clone()
    }

    fn write_failure_to(
        &self,
        artifact_dir: &Path,
        cli_error: Option<&str>,
    ) -> Result<PathBuf, String> {
        fs::create_dir_all(artifact_dir).map_err(|error| {
            format!(
                "failed to create artifact directory {}: {error}",
                artifact_dir.display()
            )
        })?;
        let failure = self.failure();
        let kind = failure.as_ref().map_or("cli_error", |failure| failure.kind);
        let message = failure
            .as_ref()
            .map(|failure| failure.message.as_str())
            .or(cli_error)
            .unwrap_or("automation run failed");
        let path = artifact_dir.join("failure.json");
        let file = File::create(&path)
            .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
        serde_json::to_writer_pretty(
            file,
            &json!({
                "version": 1,
                "kind": kind,
                "message": message,
                "cli_error": cli_error,
            }),
        )
        .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        Ok(path)
    }

    fn write_to(&self, artifact_dir: &Path) -> Result<PathBuf, String> {
        fs::create_dir_all(artifact_dir).map_err(|error| {
            format!(
                "failed to create artifact directory {}: {error}",
                artifact_dir.display()
            )
        })?;
        let path = artifact_dir.join("recent.log");
        let mut file = File::create(&path)
            .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
        for line in self.snapshot() {
            writeln!(file, "{line}")
                .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        }
        Ok(path)
    }
}

fn bevy_error_message(line: &str) -> Option<&str> {
    let after_level = line
        .split_once(" ERROR ")
        .map(|(_, rest)| rest)
        .or_else(|| line.strip_prefix("ERROR "))?;
    after_level
        .split_once(": ")
        .map(|(_, message)| message.trim())
        .filter(|message| !message.is_empty())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("star_sim_debug: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mode = args.next().ok_or(
        "usage: star_sim_debug <logical|visual> [--artifact-dir PATH] [--record PATH] | report ARTIFACT_DIR [--create]",
    )?;
    if mode == "report" {
        return report::run(args.collect());
    }
    let options = parse_options(args.collect())?;
    let recent_logs = RecentLogs::default();
    let result = match mode.as_str() {
        "logical" => run_logical(options.record, recent_logs.clone()),
        "visual" => run_visual(
            options.artifact_dir.clone(),
            options.record,
            recent_logs.clone(),
        ),
        _ => return Err(format!("unknown mode {mode:?}; expected logical or visual")),
    };
    let cli_error = result.as_ref().err().map(String::as_str);
    if cli_error.is_some() || recent_logs.failure().is_some() {
        match recent_logs.write_to(&options.artifact_dir) {
            Ok(path) => eprintln!("recent log: {}", path.display()),
            Err(error) => eprintln!("warning: could not save recent log: {error}"),
        }
        match recent_logs.write_failure_to(&options.artifact_dir, cli_error) {
            Ok(path) => eprintln!("failure metadata: {}", path.display()),
            Err(error) => eprintln!("warning: could not save failure metadata: {error}"),
        }
    }
    result
}

struct Options {
    artifact_dir: PathBuf,
    record: Option<PathBuf>,
}

fn parse_options(args: Vec<String>) -> Result<Options, String> {
    let mut options = Options {
        artifact_dir: PathBuf::from("artifacts/debug-ci"),
        record: None,
    };
    let mut args = args.into_iter();
    while let Some(option) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("{option} requires a path"))?;
        match option.as_str() {
            "--artifact-dir" => options.artifact_dir = PathBuf::from(value),
            "--record" if options.record.is_none() => options.record = Some(PathBuf::from(value)),
            "--record" => return Err("--record may only be supplied once".into()),
            _ => return Err(format!("unknown option {option:?}")),
        }
    }
    Ok(options)
}

struct SessionRecorder {
    file: File,
    sequence: u64,
}

impl SessionRecorder {
    fn create(path: &Path) -> Result<Self, String> {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create recording directory {}: {error}",
                    parent.display()
                )
            })?;
        }
        let file = File::create(path)
            .map_err(|error| format!("failed to create recording {}: {error}", path.display()))?;
        Ok(Self { file, sequence: 0 })
    }

    fn record(&mut self, direction: &str, message: &Value) -> Result<(), String> {
        serde_json::to_writer(
            &mut self.file,
            &json!({
                "sequence": self.sequence,
                "direction": direction,
                "message": message,
            }),
        )
        .map_err(|error| format!("failed to serialize session recording: {error}"))?;
        writeln!(self.file)
            .and_then(|_| self.file.flush())
            .map_err(|error| format!("failed to write session recording: {error}"))?;
        self.sequence += 1;
        Ok(())
    }
}

struct Client {
    child: Child,
    stdin: ChildStdin,
    receiver: mpsc::Receiver<Result<Value, String>>,
    recorder: Option<SessionRecorder>,
    stderr_thread: Option<thread::JoinHandle<()>>,
}

impl Client {
    fn spawn(
        mut command: Command,
        record: Option<PathBuf>,
        recent_logs: RecentLogs,
    ) -> Result<Self, String> {
        let recorder = record.as_deref().map(SessionRecorder::create).transpose()?;
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("failed to start child: {error}"))?;
        let stdin = child.stdin.take().ok_or("child has no stdin")?;
        let stdout = child.stdout.take().ok_or("child has no stdout")?;
        let stderr = child.stderr.take().ok_or("child has no stderr")?;
        let receiver = json_reader(stdout);
        let stderr_thread = thread::spawn(move || {
            if let Err(error) = stream_stderr(BufReader::new(stderr), io::stderr(), recent_logs) {
                eprintln!("star_sim_debug: failed to read child stderr: {error}");
            }
        });
        Ok(Self {
            child,
            stdin,
            receiver,
            recorder,
            stderr_thread: Some(stderr_thread),
        })
    }

    fn ready(&mut self, required: &[&str]) -> Result<Value, String> {
        let value = self.receive()?;
        self.record("from_app", &value)?;
        if value["type"] != "ready" || value["version"] != 1 {
            return Err(format!("invalid ready message: {value}"));
        }
        for capability in required {
            if !value["capabilities"]
                .as_array()
                .is_some_and(|values| values.iter().any(|value| value == capability))
            {
                return Err(format!("child lacks capability {capability}"));
            }
        }
        Ok(value)
    }

    fn request(&mut self, id: &str, command: Value) -> Result<Value, String> {
        let request = json!({"version": 1, "id": id, "command": command});
        self.record("to_app", &request)?;
        writeln!(self.stdin, "{request}")
            .and_then(|_| self.stdin.flush())
            .map_err(|error| format!("failed to send {id}: {error}"))?;
        let response = self.receive()?;
        self.record("from_app", &response)?;
        if response["id"] != id {
            return Err(format!("expected response {id}, got {response}"));
        }
        if response["status"] != "completed" {
            return Err(format!("request {id} failed: {response}"));
        }
        Ok(response)
    }

    fn receive(&self) -> Result<Value, String> {
        self.receiver
            .recv_timeout(TIMEOUT)
            .map_err(|error| format!("timed out waiting for child: {error}"))?
    }

    fn record(&mut self, direction: &str, message: &Value) -> Result<(), String> {
        if let Some(recorder) = &mut self.recorder {
            recorder.record(direction, message)?;
        }
        Ok(())
    }

    fn shutdown(mut self) -> Result<(), String> {
        self.request("shutdown", json!({"type": "shutdown"}))?;
        let status = self.child.wait().map_err(|error| error.to_string())?;
        status
            .success()
            .then_some(())
            .ok_or_else(|| format!("child exited {status}"))
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
        if let Some(thread) = self.stderr_thread.take() {
            let _ = thread.join();
        }
    }
}

fn stream_stderr(
    reader: impl BufRead,
    mut terminal: impl Write,
    recent_logs: RecentLogs,
) -> io::Result<()> {
    for line in reader.lines() {
        let line = line?;
        recent_logs.push(line.clone());
        writeln!(terminal, "{line}")?;
        terminal.flush()?;
    }
    Ok(())
}

fn json_reader(stdout: ChildStdout) -> mpsc::Receiver<Result<Value, String>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let value = line.map_err(|error| error.to_string()).and_then(|line| {
                serde_json::from_str(&line)
                    .map_err(|error| format!("non-JSON stdout {line:?}: {error}"))
            });
            if sender.send(value).is_err() {
                return;
            }
        }
        let _ = sender.send(Err("child stdout closed".into()));
    });
    receiver
}

fn cargo_example(package: &str, name: &str, features: &[&str]) -> Command {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = Command::new(cargo);
    command.args(["run", "-q", "-p", package, "--example", name]);
    if !features.is_empty() {
        command.arg("--features").arg(features.join(","));
    }
    command.arg("--");
    command
}

fn run_logical(record: Option<PathBuf>, recent_logs: RecentLogs) -> Result<(), String> {
    let mut command = cargo_example("automation_control", "automation_control_headless", &[]);
    command.args(["--automation", "--seed", "42"]);
    let mut client = Client::spawn(command, record, recent_logs)?;
    let ready = client.ready(&["pause", "step_frames", "step_simulation", "wait_until"])?;
    client.request("pause", json!({"type": "pause"}))?;
    client.request("frames", json!({"type": "step_frames", "count": 3}))?;
    client.request(
        "simulation",
        json!({"type": "step_simulation", "duration_ms": 120}),
    )?;
    client.request(
        "click",
        json!({"type": "click", "target": "toolbar.generate"}),
    )?;
    client.request(
        "wait-selection",
        json!({
            "type": "wait_until",
            "condition": {"type": "selection_is", "target": "scene.prototype_star"},
            "timeout_frames": 5
        }),
    )?;
    let state = client.request("state", json!({"type": "inspect_run"}))?;
    client.shutdown()?;
    println!(
        "{}",
        json!({"status": "passed", "mode": "logical", "ready": ready, "state": state["result"]})
    );
    Ok(())
}

fn run_visual(
    artifact_dir: PathBuf,
    record: Option<PathBuf>,
    recent_logs: RecentLogs,
) -> Result<(), String> {
    std::fs::create_dir_all(&artifact_dir).map_err(|error| error.to_string())?;
    let artifact_dir = artifact_dir
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let mut command = cargo_example(
        "bevy_viewer",
        "automation_control_prototype",
        &["automation-control"],
    );
    command.args(["--automation", "--artifact-dir"]);
    command.arg(&artifact_dir);
    let mut client = Client::spawn(command, record, recent_logs)?;
    client.ready(&["camera_focus", "screenshot"])?;
    client.request("focus", json!({"type": "camera_focus", "camera": "camera.main", "target": "scene.prototype_star", "duration_ms": 0}))?;
    let window = client.request("window", json!({"type": "screenshot", "source": {"type": "window", "target": "window.primary"}, "path": "window.png"}))?;
    let camera = client.request("camera", json!({"type": "screenshot", "source": {"type": "camera", "target": "camera.main"}, "path": "camera.png"}))?;
    client.shutdown()?;
    let window_path = PathBuf::from(
        window["result"]["path"]
            .as_str()
            .ok_or("window path missing")?,
    );
    let camera_path = PathBuf::from(
        camera["result"]["path"]
            .as_str()
            .ok_or("camera path missing")?,
    );
    validate_png(&window_path, (640, 360))?;
    validate_png(&camera_path, (320, 180))?;
    println!(
        "{}",
        json!({"status": "passed", "mode": "visual", "window_png": window_path, "camera_png": camera_path})
    );
    Ok(())
}

fn validate_png(path: &PathBuf, expected: (u32, u32)) -> Result<(), String> {
    let data = std::fs::read(path).map_err(|error| error.to_string())?;
    if data.len() < 24 || &data[..8] != b"\x89PNG\r\n\x1a\n" || &data[12..16] != b"IHDR" {
        return Err(format!("{} is not a valid PNG", path.display()));
    }
    let width = u32::from_be_bytes(data[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(data[20..24].try_into().unwrap());
    (width, height)
        .eq(&expected)
        .then_some(())
        .ok_or_else(|| format!("unexpected PNG size {width}x{height}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stderr_is_teed_while_being_retained() {
        let logs = RecentLogs::default();
        let mut terminal = Vec::new();

        stream_stderr(
            std::io::Cursor::new("first\nsecond\n"),
            &mut terminal,
            logs.clone(),
        )
        .unwrap();

        assert_eq!(String::from_utf8(terminal).unwrap(), "first\nsecond\n");
        assert_eq!(logs.snapshot(), ["first", "second"]);
    }

    #[test]
    fn panic_headline_survives_when_backtrace_exceeds_recent_log() {
        let logs = RecentLogs::default();
        logs.push("thread 'main' panicked at src/main.rs:42:9:".into());
        logs.push("stellar catalog invariant violated".into());
        for index in 0..75 {
            logs.push(format!("backtrace frame {index}"));
        }

        let failure = logs.failure().expect("panic should be detected");
        assert_eq!(failure.kind, "panic");
        assert_eq!(
            failure.message,
            "stellar catalog invariant violated (thread 'main' panicked at src/main.rs:42:9:)"
        );
        assert_eq!(logs.snapshot().len(), 50);
    }

    #[test]
    fn bevy_error_message_becomes_the_failure_headline() {
        let logs = RecentLogs::default();
        logs.push(
            "2026-08-14T12:00:00Z ERROR star_sim::automation: automation-control initialization failed"
                .into(),
        );

        let failure = logs.failure().expect("error should be detected");
        assert_eq!(failure.kind, "error");
        assert_eq!(failure.message, "automation-control initialization failed");
    }

    #[test]
    fn recent_log_keeps_only_the_latest_fifty_lines() {
        let logs = RecentLogs::default();
        for index in 0..55 {
            logs.push(format!("log line {index}"));
        }

        let lines = logs.snapshot();
        assert_eq!(lines.len(), 50);
        assert_eq!(lines.first().unwrap(), "log line 5");
        assert_eq!(lines.last().unwrap(), "log line 54");
    }

    #[test]
    fn failed_child_error_is_available_for_the_failure_log() {
        let logs = RecentLogs::default();
        let mut command = Command::new("sh");
        command.args([
            "-c",
            "printf 'ERROR star_sim::automation: automation-control initialization failed\\n' >&2; exit 101",
        ]);
        let mut client = Client::spawn(command, None, logs.clone()).unwrap();

        let status = client.child.wait().unwrap();
        drop(client);

        assert_eq!(status.code(), Some(101));
        assert_eq!(
            logs.snapshot(),
            ["ERROR star_sim::automation: automation-control initialization failed"]
        );
    }

    #[test]
    fn failure_metadata_uses_the_detected_application_headline() {
        let artifact_dir = std::env::temp_dir().join(format!(
            "star-sim-failure-metadata-test-{}",
            std::process::id()
        ));
        fs::remove_dir_all(&artifact_dir).ok();
        let logs = RecentLogs::default();
        logs.push("thread 'main' panicked at src/main.rs:42:9:".into());
        logs.push("stellar catalog invariant violated".into());

        let path = logs
            .write_failure_to(&artifact_dir, Some("child exited with status 101"))
            .unwrap();
        let failure: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        assert_eq!(failure["version"], 1);
        assert_eq!(failure["kind"], "panic");
        assert_eq!(
            failure["message"],
            "stellar catalog invariant violated (thread 'main' panicked at src/main.rs:42:9:)"
        );
        assert_eq!(failure["cli_error"], "child exited with status 101");
        fs::remove_dir_all(artifact_dir).ok();
    }

    #[test]
    fn recent_log_is_written_to_the_artifact_directory() {
        let artifact_dir =
            std::env::temp_dir().join(format!("star-sim-recent-log-test-{}", std::process::id()));
        fs::remove_dir_all(&artifact_dir).ok();
        let logs = RecentLogs::default();
        logs.push("first error".into());
        logs.push("second error".into());

        let path = logs.write_to(&artifact_dir).unwrap();

        assert_eq!(path, artifact_dir.join("recent.log"));
        assert_eq!(
            fs::read_to_string(path).unwrap(),
            "first error\nsecond error\n"
        );
        fs::remove_dir_all(artifact_dir).ok();
    }

    #[test]
    fn rejects_unknown_mode_and_bad_options() {
        assert!(parse_options(vec!["--bad".into()]).is_err());
        let mut command = Command::new("sh");
        command.args(["-c", "printf 'not-json\\n'"]);
        let client = Client::spawn(command, None, RecentLogs::default()).unwrap();
        assert!(client.receive().unwrap_err().contains("non-JSON stdout"));
    }
}
