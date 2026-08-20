mod scenario;

use automation_control::driver::{COMMAND_LINE_USAGE, CommandLine, Config, github};
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    if let Err(error) = execute_command() {
        eprintln!("star_sim_debug: {error}");
        std::process::exit(1);
    }
}

fn execute_command() -> Result<(), String> {
    let command = CommandLine::parse(env::args_os().skip(1)).map_err(|error| {
        let message = error.to_string();
        if message.starts_with("usage:") {
            message
        } else {
            format!("{message}; {COMMAND_LINE_USAGE}")
        }
    })?;
    match command {
        CommandLine::Report(options) => {
            let config = load_config(options.config_path.as_deref())?;
            let report = github::Report::prepare(&options.artifact_dir, &config.report)
                .map_err(|error| error.to_string())?;
            let outcome = if options.create {
                report.publish().map_err(|error| error.to_string())?
            } else {
                report.draft()
            };
            println!(
                "{}",
                serde_json::to_string(&outcome).map_err(|error| error.to_string())?
            );
            Ok(())
        }
        CommandLine::Run(options) => {
            let config = load_config(options.config_path.as_deref())?;
            scenario::execute(options, config)
        }
    }
}

fn load_config(config_path: Option<&Path>) -> Result<Config, String> {
    let config_path = config_path
        .map(Path::to_path_buf)
        .unwrap_or_else(default_config_path);
    Config::load(&config_path)
        .map_err(|error| format!("{}; pass --config PATH to select another file", error))
}

fn default_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/automation/debug.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_in_configuration_loads_and_points_at_the_application_target() {
        let config = Config::load(default_config_path()).unwrap();
        assert_eq!(config.application.package, "bevy_test_apps");
        assert_eq!(config.application.target, "context_menu");
        assert_eq!(config.application.features, ["automation"]);
        assert_eq!(
            config.report.generated_by.as_deref(),
            Some("star_sim_debug report")
        );
    }
}
