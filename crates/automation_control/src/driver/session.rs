use super::{
    config::SessionConfig,
    diagnostics::{RecentLogs, stream_stderr},
    launch::LaunchSpec,
};
use crate::{
    AUTOMATION_CONTROL_ARTIFACT_DIR, Command, PROTOCOL_VERSION, Ready, Request, Response,
    ResponseStatus, observation::Request as ObservationRequest, time::Command as TimeCommand,
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
    WaitLimitReached {
        frame_limit: u64,
        last_observation: Value,
    },
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
            Self::WaitLimitReached { frame_limit, .. } => write!(
                formatter,
                "observation condition was not met within {frame_limit} controlled frames"
            ),
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
    next_sequence: u64,
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
            next_sequence: 1,
            clean_shutdown: false,
        })
    }

    pub fn ready(&mut self) -> Result<Ready, DriverError> {
        let value = self.receive()?;
        self.record("from_app", &value)?;
        let ready: Ready = serde_json::from_value(value)
            .map_err(|error| DriverError::Protocol(format!("invalid ready message: {error}")))?;
        if ready.kind != "ready" || ready.version != PROTOCOL_VERSION {
            return Err(DriverError::Protocol(format!(
                "invalid ready message: {ready:?}"
            )));
        }
        Ok(ready)
    }

    /// Sends one command with a host-assigned sequence beginning at one.
    pub fn request(&mut self, command: Command) -> Result<Response, DriverError> {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        let request = Request { sequence, command };
        let request_value = serde_json::to_value(&request).map_err(|error| {
            DriverError::Protocol(format!("failed to serialize request {sequence}: {error}"))
        })?;
        self.record("to_app", &request_value)?;
        let serialized = serde_json::to_string(&request).map_err(|error| {
            DriverError::Protocol(format!("failed to serialize request {sequence}: {error}"))
        })?;
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| DriverError::Io("child stdin is closed".into()))?;
        writeln!(stdin, "{serialized}")
            .and_then(|_| stdin.flush())
            .map_err(|error| {
                DriverError::Io(format!("failed to send sequence {sequence}: {error}"))
            })?;
        let response_value = self.receive()?;
        self.record("from_app", &response_value)?;
        let response: Response = serde_json::from_value(response_value).map_err(|error| {
            DriverError::Protocol(format!("invalid response for sequence {sequence}: {error}"))
        })?;
        if response.sequence != sequence {
            return Err(DriverError::Protocol(format!(
                "expected response sequence {sequence}, got {response:?}"
            )));
        }
        if response.status != ResponseStatus::Completed {
            return Err(DriverError::RequestFailed(response));
        }
        Ok(response)
    }

    /// Repeats an observation and advances one controlled frame after each miss.
    ///
    /// The predicate runs first against the current state. At most `limit` controlled frames are
    /// then advanced, and every command remains subject to the session's wall-clock timeout.
    pub fn wait_for_observation<F>(
        &mut self,
        request: ObservationRequest,
        limit: super::wait::FrameLimit,
        mut predicate: F,
    ) -> Result<Response, DriverError>
    where
        F: FnMut(&Value) -> bool,
    {
        let mut response = self.request(Command::Observe(request.clone()))?;
        for advanced_frames in 0..=limit.frames {
            let observation = response.result.as_ref().ok_or_else(|| {
                DriverError::Protocol("completed observation has no result".into())
            })?;
            if predicate(observation) {
                return Ok(response);
            }
            if advanced_frames == limit.frames {
                return Err(DriverError::WaitLimitReached {
                    frame_limit: limit.frames,
                    last_observation: observation.clone(),
                });
            }
            self.request(Command::Time(TimeCommand::advance(
                1,
                limit.step_nanoseconds,
            )))?;
            response = self.request(Command::Observe(request.clone()))?;
        }
        unreachable!("the bounded wait loop always returns")
    }

    pub fn shutdown(mut self) -> Result<(), DriverError> {
        self.request(Command::Shutdown)?;
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
        self.receiver
            .recv_timeout(self.timeout)
            .map_err(|error| match error {
                mpsc::RecvTimeoutError::Timeout => {
                    DriverError::Timeout(format!("{} seconds", self.timeout.as_secs()))
                }
                mpsc::RecvTimeoutError::Disconnected => {
                    DriverError::Protocol("child stdout closed".into())
                }
            })?
            .map_err(DriverError::Protocol)
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
    use crate::{Command, RunMode};
    use std::process::Command as ProcessCommand;

    #[test]
    fn records_driver_sequences_starting_at_one_without_request_ids() {
        let record = std::env::temp_dir().join(format!(
            "automation-control-session-{}.jsonl",
            std::process::id()
        ));
        let mut command = ProcessCommand::new("sh");
        command.args([
            "-c",
            "printf '%s\\n' '{\"type\":\"ready\",\"version\":2,\"mode\":\"logical\",\"controls\":[\"pointer\"],\"observation_scopes\":[\"targets\"]}'; while read line; do printf '%s\\n' '{\"sequence\":1,\"status\":\"completed\",\"result\":{}}'; exit 0; done",
        ]);
        let mut session = Session::spawn_command(
            command,
            SessionOptions::new(Duration::from_secs(2)).with_record(Some(record.clone())),
        )
        .unwrap();
        assert_eq!(session.ready().unwrap().mode, RunMode::Logical);
        session.request(Command::Shutdown).unwrap();
        let data = fs::read_to_string(&record).unwrap();
        assert!(data.contains("\"sequence\":1"));
        assert!(!data.contains("\"id\""));
        fs::remove_file(record).ok();
    }

    #[test]
    fn rejects_non_json_child_stdout_as_a_protocol_error() {
        let mut command = ProcessCommand::new("sh");
        command.args(["-c", "printf 'human log on stdout\\n'"]);
        let mut session =
            Session::spawn_command(command, SessionOptions::new(Duration::from_secs(2))).unwrap();
        let error = session.ready().unwrap_err().to_string();
        assert!(
            error.contains("non-JSON stdout"),
            "unexpected error: {error}"
        );
    }
}
