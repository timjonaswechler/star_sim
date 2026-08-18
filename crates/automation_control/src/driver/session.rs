use super::{
    config::SessionConfig,
    diagnostics::{RecentLogs, stream_stderr},
    launch::LaunchSpec,
};
use crate::{
    AUTOMATION_CONTROL_ARTIFACT_DIR,
    protocol::{Command, PROTOCOL_VERSION, Ready, Request, Response, ResponseStatus},
};
use serde_json::Value;
use std::{
    fmt,
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, ChildStdout, Command as ProcessCommand, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::os::unix::process::CommandExt;

#[derive(Debug)]
pub enum DriverError {
    Launch(String),
    Io(String),
    Protocol(String),
    Timeout(String),
    Child(String),
    RequestFailed(Response),
}

impl fmt::Display for DriverError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Launch(message) => write!(formatter, "failed to start child: {message}"),
            Self::Io(message) => formatter.write_str(message),
            Self::Protocol(message) => write!(formatter, "protocol error: {message}"),
            Self::Timeout(message) => write!(formatter, "timed out waiting for child: {message}"),
            Self::Child(message) => formatter.write_str(message),
            Self::RequestFailed(response) => write!(formatter, "request failed: {response:?}"),
        }
    }
}

impl std::error::Error for DriverError {}

pub struct SessionOptions {
    pub timeout: Duration,
    pub record: Option<PathBuf>,
    pub recent_logs: RecentLogs,
    pub artifact_dir: Option<PathBuf>,
}

impl SessionOptions {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            record: None,
            recent_logs: RecentLogs::default(),
            artifact_dir: None,
        }
    }

    /// Builds session options from the reusable timeout policy and consumer-selected paths.
    pub fn from_config(
        config: &SessionConfig,
        record: Option<PathBuf>,
        recent_logs: RecentLogs,
        artifact_dir: PathBuf,
    ) -> Self {
        Self::new(Duration::from_secs(config.timeout_seconds))
            .with_record(record)
            .with_recent_logs(recent_logs)
            .with_artifact_dir(artifact_dir)
    }

    pub fn with_record(mut self, path: Option<PathBuf>) -> Self {
        self.record = path;
        self
    }

    pub fn with_recent_logs(mut self, recent_logs: RecentLogs) -> Self {
        self.recent_logs = recent_logs;
        self
    }

    /// Supplies the artifact root to the child through the standard automation environment
    /// variable. Applications remain responsible for constructing their own confined
    /// `ArtifactRoot` when handling screenshot requests.
    pub fn with_artifact_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.artifact_dir = Some(path.into());
        self
    }
}

struct SessionRecorder {
    file: File,
    sequence: u64,
}

impl SessionRecorder {
    fn create(path: &Path) -> Result<Self, DriverError> {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                DriverError::Io(format!(
                    "failed to create recording directory {}: {error}",
                    parent.display()
                ))
            })?;
        }
        let file = File::create(path).map_err(|error| {
            DriverError::Io(format!(
                "failed to create recording {}: {error}",
                path.display()
            ))
        })?;
        Ok(Self { file, sequence: 0 })
    }

    fn record(&mut self, direction: &str, message: &Value) -> Result<(), DriverError> {
        serde_json::to_writer(
            &mut self.file,
            &serde_json::json!({
                "sequence": self.sequence,
                "direction": direction,
                "message": message,
            }),
        )
        .map_err(|error| {
            DriverError::Io(format!("failed to serialize session recording: {error}"))
        })?;
        writeln!(self.file)
            .and_then(|_| self.file.flush())
            .map_err(|error| {
                DriverError::Io(format!("failed to write session recording: {error}"))
            })?;
        self.sequence += 1;
        Ok(())
    }
}

pub struct Session {
    child: Child,
    stdin: Option<ChildStdin>,
    receiver: mpsc::Receiver<Result<Value, String>>,
    recorder: Option<SessionRecorder>,
    stderr_thread: Option<thread::JoinHandle<()>>,
    timeout: Duration,
    clean_shutdown: bool,
}

impl Session {
    pub fn spawn(spec: &LaunchSpec, options: SessionOptions) -> Result<Self, DriverError> {
        Self::spawn_command(spec.command(), options)
    }

    pub fn spawn_command(
        mut command: ProcessCommand,
        options: SessionOptions,
    ) -> Result<Self, DriverError> {
        if let Some(artifact_dir) = options.artifact_dir.as_deref() {
            command.env(AUTOMATION_CONTROL_ARTIFACT_DIR, artifact_dir);
        }
        let recorder = options
            .record
            .as_deref()
            .map(SessionRecorder::create)
            .transpose()?;
        #[cfg(unix)]
        command.process_group(0);
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| DriverError::Launch(error.to_string()))?;
        let Some(stdin) = child.stdin.take() else {
            terminate_child(&mut child);
            return Err(DriverError::Launch("child has no stdin".into()));
        };
        let Some(stdout) = child.stdout.take() else {
            terminate_child(&mut child);
            return Err(DriverError::Launch("child has no stdout".into()));
        };
        let Some(stderr) = child.stderr.take() else {
            terminate_child(&mut child);
            return Err(DriverError::Launch("child has no stderr".into()));
        };
        let receiver = json_reader(stdout);
        let recent_logs = options.recent_logs;
        let stderr_thread = thread::spawn(move || {
            if let Err(error) = stream_stderr(BufReader::new(stderr), io::stderr(), recent_logs) {
                eprintln!("automation-control: failed to read child stderr: {error}");
            }
        });
        Ok(Self {
            child,
            stdin: Some(stdin),
            receiver,
            recorder,
            stderr_thread: Some(stderr_thread),
            timeout: options.timeout,
            clean_shutdown: false,
        })
    }

    pub fn ready(&mut self, required: &[&str]) -> Result<Ready, DriverError> {
        let value = self.receive()?;
        self.record("from_app", &value)?;
        let ready: Ready = serde_json::from_value(value)
            .map_err(|error| DriverError::Protocol(format!("invalid ready message: {error}")))?;
        if ready.kind != "ready" || ready.version != PROTOCOL_VERSION {
            return Err(DriverError::Protocol(format!(
                "invalid ready message: {ready:?}"
            )));
        }
        for capability in required {
            if !ready.capabilities.iter().any(|value| value == capability) {
                return Err(DriverError::Protocol(format!(
                    "child lacks capability {capability}"
                )));
            }
        }
        Ok(ready)
    }

    pub fn request(&mut self, id: &str, command: Command) -> Result<Response, DriverError> {
        let request = Request {
            version: PROTOCOL_VERSION,
            id: id.into(),
            command,
        };
        let request_value = serde_json::to_value(&request).map_err(|error| {
            DriverError::Protocol(format!("failed to serialize request {id}: {error}"))
        })?;
        self.record("to_app", &request_value)?;
        let serialized = serde_json::to_string(&request).map_err(|error| {
            DriverError::Protocol(format!("failed to serialize request {id}: {error}"))
        })?;
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| DriverError::Io("child stdin is closed".into()))?;
        writeln!(stdin, "{serialized}")
            .and_then(|_| stdin.flush())
            .map_err(|error| DriverError::Io(format!("failed to send {id}: {error}")))?;
        let response_value = self.receive()?;
        self.record("from_app", &response_value)?;
        let response: Response = serde_json::from_value(response_value).map_err(|error| {
            DriverError::Protocol(format!("invalid response for {id}: {error}"))
        })?;
        if response.id.as_deref() != Some(id) {
            return Err(DriverError::Protocol(format!(
                "expected response {id}, got {response:?}"
            )));
        }
        if response.version != PROTOCOL_VERSION {
            return Err(DriverError::Protocol(format!(
                "unsupported response version {}; expected {PROTOCOL_VERSION}",
                response.version
            )));
        }
        if response.status != ResponseStatus::Completed {
            return Err(DriverError::RequestFailed(response));
        }
        Ok(response)
    }

    pub fn shutdown(mut self) -> Result<(), DriverError> {
        self.request("shutdown", Command::Shutdown)?;
        self.stdin.take();
        let deadline = Instant::now() + self.timeout;
        loop {
            if let Some(status) = self
                .child
                .try_wait()
                .map_err(|error| DriverError::Child(error.to_string()))?
            {
                if status.success() {
                    self.clean_shutdown = true;
                    return Ok(());
                }
                return Err(DriverError::Child(format!("child exited {status}")));
            }
            if Instant::now() >= deadline {
                terminate_child(&mut self.child);
                return Err(DriverError::Timeout(
                    "child did not exit after shutdown".into(),
                ));
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn receive(&self) -> Result<Value, DriverError> {
        let value = self
            .receiver
            .recv_timeout(self.timeout)
            .map_err(|error| match error {
                mpsc::RecvTimeoutError::Timeout => {
                    DriverError::Timeout(format!("{} seconds", self.timeout.as_secs()))
                }
                mpsc::RecvTimeoutError::Disconnected => {
                    DriverError::Protocol("child stdout closed".into())
                }
            })?;
        value.map_err(DriverError::Protocol)
    }

    fn record(&mut self, direction: &str, message: &Value) -> Result<(), DriverError> {
        if let Some(recorder) = &mut self.recorder {
            recorder.record(direction, message)?;
        }
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        self.stdin.take();
        if self.child.try_wait().ok().flatten().is_none() {
            terminate_child(&mut self.child);
        }
        if self.clean_shutdown {
            if let Some(thread) = self.stderr_thread.take() {
                let _ = thread.join();
            }
        } else {
            // A failed or timed-out Cargo wrapper may have left a descendant holding the pipe.
            // Detach the reader rather than allowing diagnostics to make process cleanup hang.
            self.stderr_thread.take();
        }
    }
}

fn terminate_child(child: &mut Child) {
    if child.try_wait().ok().flatten().is_some() {
        return;
    }
    #[cfg(unix)]
    {
        let process_group = format!("-{}", child.id());
        let _ = ProcessCommand::new("kill")
            .args(["-KILL", &process_group])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::Command;
    use std::process::Command as ProcessCommand;

    #[test]
    fn builds_options_from_session_configuration() {
        let config = SessionConfig { timeout_seconds: 7 };
        let record = PathBuf::from("artifacts/session.jsonl");
        let artifact_dir = PathBuf::from("artifacts/run");
        let options = SessionOptions::from_config(
            &config,
            Some(record.clone()),
            RecentLogs::default(),
            artifact_dir.clone(),
        );
        assert_eq!(options.timeout, Duration::from_secs(7));
        assert_eq!(options.record, Some(record));
        assert_eq!(options.artifact_dir, Some(artifact_dir));
    }

    #[test]
    fn records_typed_requests_as_protocol_json() {
        let record = std::env::temp_dir().join(format!(
            "automation-control-session-{}.jsonl",
            std::process::id()
        ));
        let mut command = ProcessCommand::new("sh");
        command.args(["-c", "printf '%s\\n' '{\"version\":1,\"type\":\"ready\",\"capabilities\":[],\"mode\":\"logical\",\"seed\":42,\"fixed_step_ms\":50}' ; while read line; do printf '%s\\n' '{\"version\":1,\"id\":\"shutdown\",\"status\":\"completed\",\"result\":{}}'; exit 0; done"]);
        let logs = RecentLogs::default();
        let options = SessionOptions::new(Duration::from_secs(2))
            .with_record(Some(record.clone()))
            .with_recent_logs(logs);
        let mut session = Session::spawn_command(command, options).unwrap();
        session.ready(&[]).unwrap();
        session.request("shutdown", Command::Shutdown).unwrap();
        let data = fs::read_to_string(&record).unwrap();
        assert!(data.contains("\"direction\":\"to_app\""));
        fs::remove_file(record).ok();
    }

    #[test]
    fn propagates_artifact_root_to_child_environment() {
        let artifact_dir = std::env::temp_dir().join(format!(
            "automation-control-artifact-env-{}",
            std::process::id()
        ));
        let mut command = ProcessCommand::new("sh");
        command.args([
            "-c",
            "printf '%s\\n' '{\"version\":1,\"type\":\"ready\",\"capabilities\":[],\"mode\":\"logical\",\"seed\":42,\"fixed_step_ms\":50}'; read line; test \"$AUTOMATION_CONTROL_ARTIFACT_DIR\" = \"$EXPECTED_ARTIFACT_DIR\" || exit 42; printf '%s\\n' '{\"version\":1,\"id\":\"shutdown\",\"status\":\"completed\",\"result\":{}}'",
        ]);
        command.env("EXPECTED_ARTIFACT_DIR", &artifact_dir);
        let options =
            SessionOptions::new(Duration::from_secs(2)).with_artifact_dir(artifact_dir.clone());
        let mut session = Session::spawn_command(command, options).unwrap();
        session.ready(&[]).unwrap();
        session.shutdown().unwrap();
    }

    #[test]
    fn invalid_stdout_is_reported_as_a_protocol_error() {
        let mut command = ProcessCommand::new("sh");
        command.args(["-c", "printf 'not-json\\n'"]);
        let mut session =
            Session::spawn_command(command, SessionOptions::new(Duration::from_secs(2))).unwrap();
        let error = session.ready(&[]).unwrap_err().to_string();
        assert!(error.contains("invalid ready message") || error.contains("non-JSON"));
    }

    #[test]
    fn rejects_a_completed_response_with_an_unsupported_version() {
        let mut command = ProcessCommand::new("sh");
        command.args([
            "-c",
            "printf '%s\\n' '{\"version\":1,\"type\":\"ready\",\"capabilities\":[],\"mode\":\"logical\",\"seed\":42,\"fixed_step_ms\":50}'; read line; printf '%s\\n' '{\"version\":2,\"id\":\"state\",\"status\":\"completed\",\"result\":{}}'; sleep 1",
        ]);
        let mut session =
            Session::spawn_command(command, SessionOptions::new(Duration::from_secs(2))).unwrap();
        session.ready(&[]).unwrap();
        let error = session.request("state", Command::InspectRun).unwrap_err();
        assert!(error.to_string().contains("unsupported response version"));
    }

    #[test]
    fn shutdown_is_bounded_by_the_session_timeout() {
        let mut command = ProcessCommand::new("sh");
        command.args([
            "-c",
            "printf '%s\\n' '{\"version\":1,\"type\":\"ready\",\"capabilities\":[],\"mode\":\"logical\",\"seed\":42,\"fixed_step_ms\":50}'; read line; printf '%s\\n' '{\"version\":1,\"id\":\"shutdown\",\"status\":\"completed\",\"result\":{}}'; sleep 5",
        ]);
        let mut session =
            Session::spawn_command(command, SessionOptions::new(Duration::from_millis(50)))
                .unwrap();
        session.ready(&[]).unwrap();
        let error = session.shutdown().unwrap_err();
        assert!(error.to_string().contains("did not exit after shutdown"));
    }
}
