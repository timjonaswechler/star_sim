use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fmt,
    fs::{self, File},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

pub const DEFAULT_RECENT_LOG_CAPACITY: usize = 50;
pub const FAILURE_REPORT_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FailureHeadline {
    pub kind: &'static str,
    pub message: String,
}

/// Versioned metadata describing why an automation run failed.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FailureReport {
    pub version: u32,
    pub kind: String,
    pub message: String,
    #[serde(default)]
    pub cli_error: Option<String>,
    #[serde(default)]
    pub record_path: Option<PathBuf>,
}

impl FailureReport {
    /// Builds failure metadata from the retained application headline and controller error.
    pub fn new(
        failure: Option<&FailureHeadline>,
        cli_error: Option<&str>,
        record_path: Option<&Path>,
    ) -> Self {
        let (kind, message) = failure
            .map(|failure| (failure.kind.to_owned(), failure.message.clone()))
            .unwrap_or_else(|| {
                (
                    "cli_error".into(),
                    cli_error.unwrap_or("automation run failed").into(),
                )
            });
        Self {
            version: FAILURE_REPORT_VERSION,
            kind,
            message,
            cli_error: cli_error.map(str::to_owned),
            record_path: record_path.map(Path::to_path_buf),
        }
    }

    /// Loads and validates a failure report from its JSON artifact.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, DiagnosticsError> {
        let path = path.as_ref();
        let data = fs::read(path).map_err(|error| DiagnosticsError::io("read", path, error))?;
        let report = serde_json::from_slice::<Self>(&data)
            .map_err(|error| DiagnosticsError::json("parse", path, error.to_string()))?;
        report
            .validate()
            .map_err(|message| DiagnosticsError::invalid(path, message))?;
        Ok(report)
    }

    /// Validates the version and required values of this report format.
    pub fn validate(&self) -> Result<(), String> {
        if self.version != FAILURE_REPORT_VERSION {
            return Err(format!(
                "unsupported failure report version {}; expected {FAILURE_REPORT_VERSION}",
                self.version
            ));
        }
        require_value("kind", &self.kind)?;
        require_value("message", &self.message)?;
        if self
            .record_path
            .as_ref()
            .is_some_and(|path| path.as_os_str().is_empty())
        {
            return Err("record_path must not be empty when supplied".into());
        }
        Ok(())
    }

    /// Writes this report as a validated, pretty-printed JSON artifact.
    pub fn write_to(&self, path: impl AsRef<Path>) -> Result<(), DiagnosticsError> {
        let path = path.as_ref();
        self.validate()
            .map_err(|message| DiagnosticsError::invalid(path, message))?;
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent)
                .map_err(|error| DiagnosticsError::io("create parent directory", parent, error))?;
        }
        let file =
            File::create(path).map_err(|error| DiagnosticsError::io("create", path, error))?;
        serde_json::to_writer_pretty(file, self)
            .map_err(|error| DiagnosticsError::json("write", path, error.to_string()))?;
        Ok(())
    }
}

fn require_value(name: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{name} must not be empty"))
    } else {
        Ok(())
    }
}

/// Paths produced together when a failed run's diagnostics are persisted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticArtifacts {
    pub recent_log: PathBuf,
    pub failure_report: PathBuf,
}

#[derive(Debug)]
pub enum DiagnosticsError {
    Io {
        operation: &'static str,
        path: PathBuf,
        error: io::Error,
    },
    Json {
        operation: &'static str,
        path: PathBuf,
        error: String,
    },
    Invalid {
        path: PathBuf,
        message: String,
    },
}

impl DiagnosticsError {
    fn io(operation: &'static str, path: &Path, error: io::Error) -> Self {
        Self::Io {
            operation,
            path: path.to_path_buf(),
            error,
        }
    }

    fn json(operation: &'static str, path: &Path, error: String) -> Self {
        Self::Json {
            operation,
            path: path.to_path_buf(),
            error,
        }
    }

    fn invalid(path: &Path, message: String) -> Self {
        Self::Invalid {
            path: path.to_path_buf(),
            message,
        }
    }
}

impl fmt::Display for DiagnosticsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io {
                operation,
                path,
                error,
            } => write!(
                formatter,
                "failed to {operation} {}: {error}",
                path.display()
            ),
            Self::Json {
                operation,
                path,
                error,
            } => write!(
                formatter,
                "failed to {operation} JSON in {}: {error}",
                path.display()
            ),
            Self::Invalid { path, message } => {
                write!(
                    formatter,
                    "invalid failure report {}: {message}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for DiagnosticsError {}

#[derive(Default)]
struct RecentLogState {
    capacity: usize,
    lines: VecDeque<String>,
    failure: Option<FailureHeadline>,
    pending_panic_location: Option<String>,
}

#[derive(Clone)]
pub struct RecentLogs(Arc<Mutex<RecentLogState>>);

impl Default for RecentLogs {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_RECENT_LOG_CAPACITY)
    }
}

impl RecentLogs {
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "recent log capacity must be positive");
        Self(Arc::new(Mutex::new(RecentLogState {
            capacity,
            ..RecentLogState::default()
        })))
    }

    pub fn push(&self, line: String) {
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
        if state.lines.len() == state.capacity {
            state.lines.pop_front();
        }
        state.lines.push_back(line);
    }

    pub fn snapshot(&self) -> Vec<String> {
        self.0
            .lock()
            .expect("recent log mutex poisoned")
            .lines
            .iter()
            .cloned()
            .collect()
    }

    pub fn failure(&self) -> Option<FailureHeadline> {
        self.0
            .lock()
            .expect("recent log mutex poisoned")
            .failure
            .clone()
    }

    /// Persists the rolling log and typed failure metadata as one diagnostic artifact set.
    pub fn persist_failure_artifacts(
        &self,
        artifact_dir: impl AsRef<Path>,
        cli_error: Option<&str>,
        record_path: Option<&Path>,
    ) -> Result<DiagnosticArtifacts, DiagnosticsError> {
        let artifact_dir = artifact_dir.as_ref();
        fs::create_dir_all(artifact_dir).map_err(|error| {
            DiagnosticsError::io("create artifact directory", artifact_dir, error)
        })?;
        let recent_log = artifact_dir.join("recent.log");
        self.write_recent_log(&recent_log)?;
        let failure_report = artifact_dir.join("failure.json");
        FailureReport::new(self.failure().as_ref(), cli_error, record_path)
            .write_to(&failure_report)?;
        Ok(DiagnosticArtifacts {
            recent_log,
            failure_report,
        })
    }

    fn write_recent_log(&self, path: &Path) -> Result<(), DiagnosticsError> {
        let mut file =
            File::create(path).map_err(|error| DiagnosticsError::io("create", path, error))?;
        for line in self.snapshot() {
            writeln!(file, "{line}").map_err(|error| DiagnosticsError::io("write", path, error))?;
        }
        Ok(())
    }
}

pub fn stream_stderr(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "automation-control-{name}-{}-{nonce}",
            std::process::id()
        ))
    }

    #[test]
    fn stderr_is_teed_and_retained_with_configured_capacity() {
        let logs = RecentLogs::with_capacity(2);
        let mut terminal = Vec::new();
        stream_stderr(
            std::io::Cursor::new("first\nsecond\nthird\n"),
            &mut terminal,
            logs.clone(),
        )
        .unwrap();
        assert_eq!(
            String::from_utf8(terminal).unwrap(),
            "first\nsecond\nthird\n"
        );
        assert_eq!(logs.snapshot(), ["second", "third"]);
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
        assert_eq!(logs.snapshot().len(), DEFAULT_RECENT_LOG_CAPACITY);
    }

    #[test]
    fn bevy_error_message_becomes_the_failure_headline() {
        let logs = RecentLogs::default();
        logs.push("2026-08-14T12:00:00Z ERROR star_sim::automation: automation-control initialization failed".into());
        let failure = logs.failure().expect("error should be detected");
        assert_eq!(failure.kind, "error");
        assert_eq!(failure.message, "automation-control initialization failed");
    }

    #[test]
    fn failure_report_round_trips_through_json() {
        let path = temporary_path("failure-report.json");
        let report = FailureReport {
            version: FAILURE_REPORT_VERSION,
            kind: "panic".into(),
            message: "stellar catalog invariant violated".into(),
            cli_error: Some("child exited with status 101".into()),
            record_path: Some(PathBuf::from("artifacts/custom-session.jsonl")),
        };
        report.write_to(&path).unwrap();
        assert_eq!(FailureReport::load(&path).unwrap(), report);
        fs::remove_file(path).ok();
    }

    #[test]
    fn failure_report_rejects_missing_and_malformed_input_with_path() {
        let missing = temporary_path("missing-failure.json");
        let error = FailureReport::load(&missing).unwrap_err();
        assert!(error.to_string().contains(&missing.display().to_string()));

        let malformed = temporary_path("malformed-failure.json");
        fs::write(&malformed, "not-json").unwrap();
        let error = FailureReport::load(&malformed).unwrap_err();
        assert!(error.to_string().contains(&malformed.display().to_string()));
        assert!(error.to_string().contains("parse JSON"));
        fs::remove_file(malformed).ok();
    }

    #[test]
    fn failure_report_rejects_unsupported_version_and_empty_message() {
        let mut report = FailureReport {
            version: FAILURE_REPORT_VERSION + 1,
            kind: "panic".into(),
            message: "failure".into(),
            cli_error: None,
            record_path: None,
        };
        assert!(
            report
                .validate()
                .unwrap_err()
                .contains("unsupported failure report version")
        );
        report.version = FAILURE_REPORT_VERSION;
        report.message.clear();
        assert!(report.validate().unwrap_err().contains("message"));
    }

    #[test]
    fn persistence_writes_recent_log_and_custom_record_path_together() {
        let artifact_dir = temporary_path("diagnostic-artifacts");
        let record = artifact_dir.join("custom-session.jsonl");
        let logs = RecentLogs::default();
        logs.push("child produced no ready message".into());
        let artifacts = logs
            .persist_failure_artifacts(&artifact_dir, Some("child failed"), Some(&record))
            .unwrap();

        assert_eq!(artifacts.recent_log, artifact_dir.join("recent.log"));
        assert_eq!(artifacts.failure_report, artifact_dir.join("failure.json"));
        assert_eq!(
            fs::read_to_string(&artifacts.recent_log).unwrap(),
            "child produced no ready message\n"
        );
        let report = FailureReport::load(&artifacts.failure_report).unwrap();
        assert_eq!(report.kind, "cli_error");
        assert_eq!(report.message, "child failed");
        assert_eq!(report.record_path, Some(record));
        fs::remove_dir_all(artifact_dir).ok();
    }

    #[test]
    fn detected_headline_takes_precedence_over_cli_error() {
        let artifact_dir = temporary_path("headline-artifacts");
        let logs = RecentLogs::default();
        logs.push("thread 'main' panicked at src/main.rs:42:9:".into());
        logs.push("stellar catalog invariant violated".into());
        let artifacts = logs
            .persist_failure_artifacts(&artifact_dir, Some("child exited with status 101"), None)
            .unwrap();
        let report = FailureReport::load(&artifacts.failure_report).unwrap();
        assert_eq!(report.kind, "panic");
        assert!(
            report
                .message
                .contains("stellar catalog invariant violated")
        );
        assert_eq!(
            report.cli_error.as_deref(),
            Some("child exited with status 101")
        );
        fs::remove_dir_all(artifact_dir).ok();
    }
}
