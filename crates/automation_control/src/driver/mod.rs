mod command_line;
mod config;
mod diagnostics;
pub mod github;
mod launch;
mod png;
mod report;
mod session;

use crate::protocol::Response;
use std::path::PathBuf;

/// Extracts the normalized path returned by a completed screenshot response.
pub fn response_path(response: &Response) -> Result<PathBuf, String> {
    response
        .result
        .as_ref()
        .and_then(|result| result.get("path"))
        .and_then(serde_json::Value::as_str)
        .map(PathBuf::from)
        .ok_or_else(|| "screenshot response path missing".into())
}

pub use command_line::{
    CommandLine, CommandLineError, ReportOptions, RunOptions, USAGE as COMMAND_LINE_USAGE,
};
pub use config::{CONFIG_VERSION, Config, ConfigError, ReportConfig, SessionConfig};
pub use diagnostics::{
    DEFAULT_RECENT_LOG_CAPACITY, DiagnosticArtifacts, DiagnosticsError, FAILURE_REPORT_VERSION,
    FailureHeadline, FailureReport, RecentLogs,
};
pub use launch::{LaunchSpec, LaunchTargetKind};
pub use png::{PngError, validate_png};
pub use report::{IssueDraft, ReportError};
pub use session::{DriverError, Session, SessionOptions};
