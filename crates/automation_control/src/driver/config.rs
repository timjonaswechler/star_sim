//! Versioned TOML configuration for Debug Host tools.
//!
//! All configuration structs reject unknown fields when deserialized.

use super::launch::LaunchSpec;
use serde::Deserialize;
use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

/// Supported Debug Host configuration schema version.
pub const CONFIG_VERSION: u32 = 1;
/// Default wall-clock response timeout used when `session.timeout_seconds` is omitted.
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 60;

/// Top-level versioned automation TOML configuration.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Schema version, which must equal [`CONFIG_VERSION`].
    pub version: u32,
    /// Cargo target used to launch the Controlled Session.
    pub application: LaunchSpec,
    /// Driver timeout settings.
    #[serde(default)]
    pub session: SessionConfig,
    /// Failure-report attribution settings.
    #[serde(default)]
    pub report: ReportConfig,
}

/// Configuration for generated issue reports.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportConfig {
    /// Optional tool attribution appended to the generated report.
    pub generated_by: Option<String>,
}

/// Wall-clock driver settings for one Controlled Session.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionConfig {
    /// Wall-clock response/shutdown timeout, not controlled simulation time.
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
        }
    }
}

impl Config {
    /// Reads, parses, and validates TOML from `path`.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|error| ConfigError::Read {
            path: path.to_path_buf(),
            error,
        })?;
        let config = toml::from_str::<Self>(&contents).map_err(|error| ConfigError::Parse {
            path: path.to_path_buf(),
            error: error.to_string(),
        })?;
        config.validate().map_err(ConfigError::Invalid)?;
        Ok(config)
    }

    /// Validates the version, required strings, and positive timeout.
    pub fn validate(&self) -> Result<(), String> {
        if self.version != CONFIG_VERSION {
            return Err(format!(
                "unsupported configuration version {}; expected {CONFIG_VERSION}",
                self.version
            ));
        }
        if self.session.timeout_seconds == 0 {
            return Err("session.timeout_seconds must be positive".into());
        }
        validate_launch("application", &self.application)?;
        if let Some(generated_by) = &self.report.generated_by {
            require_value("report.generated_by", generated_by)?;
        }
        Ok(())
    }
}

/// Configuration load, TOML parse, or semantic validation failure.
#[derive(Debug)]
pub enum ConfigError {
    /// File reading failed.
    Read {
        /// Configuration path.
        path: PathBuf,
        /// Underlying I/O error.
        error: std::io::Error,
    },
    /// TOML deserialization failed, including unknown fields.
    Parse {
        /// Configuration path.
        path: PathBuf,
        /// Parser diagnostic.
        error: String,
    },
    /// Parsed configuration violated a semantic invariant.
    Invalid(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, error } => {
                write!(
                    formatter,
                    "failed to read configuration {}: {error}",
                    path.display()
                )
            }
            Self::Parse { path, error } => {
                write!(formatter, "invalid TOML in {}: {error}", path.display())
            }
            Self::Invalid(message) => {
                write!(formatter, "invalid automation configuration: {message}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

fn validate_launch(name: &str, launch: &LaunchSpec) -> Result<(), String> {
    require_value(&format!("{name}.package"), &launch.package)?;
    require_value(&format!("{name}.target"), &launch.target)
}

fn require_value(name: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{name} must not be empty"))
    } else {
        Ok(())
    }
}

fn default_timeout_seconds() -> u64 {
    DEFAULT_TIMEOUT_SECONDS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_config() -> Config {
        toml::from_str(
            r#"
            version = 1

            [application]
            package = "app"
            kind = "binary"
            target = "app"
            features = ["automation-control"]
            arguments = ["--automation"]

            [session]
            timeout_seconds = 45

            [report]
            generated_by = "star_sim_debug report"
            "#,
        )
        .unwrap()
    }

    #[test]
    fn validates_version_and_required_values() {
        let mut config = valid_config();
        assert!(config.validate().is_ok());
        config.version = 2;
        assert!(
            config
                .validate()
                .unwrap_err()
                .contains("unsupported configuration version")
        );

        let mut config = valid_config();
        config.session.timeout_seconds = 0;
        assert!(
            config
                .validate()
                .unwrap_err()
                .contains("session.timeout_seconds")
        );
    }

    #[test]
    fn loads_generic_application_configuration_from_toml() {
        let config = valid_config();
        assert_eq!(config.application.package, "app");
        assert_eq!(config.application.target, "app");
        assert_eq!(config.application.features, ["automation-control"]);
        assert_eq!(config.session.timeout_seconds, 45);
        assert_eq!(
            config.report.generated_by.as_deref(),
            Some("star_sim_debug report")
        );
    }

    #[test]
    fn session_policy_defaults_when_omitted() {
        let config: Config = toml::from_str(
            r#"
            version = 1
            [application]
            package = "app"
            target = "app"
            "#,
        )
        .unwrap();
        assert_eq!(config.session.timeout_seconds, DEFAULT_TIMEOUT_SECONDS);
        assert!(config.report.generated_by.is_none());
    }

    #[test]
    fn rejects_empty_report_generator() {
        let mut config = valid_config();
        config.report.generated_by = Some("  ".into());
        assert!(
            config
                .validate()
                .unwrap_err()
                .contains("report.generated_by")
        );
    }

    #[test]
    fn rejects_unknown_configuration_fields() {
        let source = r#"
            version = 1
            unexpected = true

            [application]
            package = "app"
            target = "app"
        "#;
        assert!(toml::from_str::<Config>(source).is_err());
    }

    #[test]
    fn rejects_removed_scenario_configuration_fields() {
        let sources = [
            r#"
                version = 1
                [application]
                package = "app"
                target = "app"
                [logical]
                generate_target = "toolbar.generate"
            "#,
            r#"
                version = 1
                [application]
                package = "app"
                target = "app"
                [session]
                recent_log_capacity = 50
            "#,
            r#"
                version = 1
                [application]
                package = "app"
                target = "app"
                [visual]
                window_path = "window.png"
                window_size = [640, 360]
            "#,
        ];

        for source in sources {
            assert!(
                toml::from_str::<Config>(source).is_err(),
                "removed configuration field was accepted: {source}"
            );
        }
    }
}
