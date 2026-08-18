use automation_control::{
    Command, ScreenshotSource,
    driver::{DriverError, LaunchSpec, LaunchTargetKind, RecentLogs, Session, SessionOptions},
};
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::Command as ProcessCommand,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[test]
#[ignore = "requires a display and a rendered apps/app process"]
fn checked_in_app_logical_scenario_uses_real_menu_semantics() {
    let artifact_dir = unique_temp_dir("logical");
    let record_path = artifact_dir.join("session.jsonl");
    let output = ProcessCommand::new(env!("CARGO_BIN_EXE_star_sim_debug"))
        .args(["logical", "--artifact-dir"])
        .arg(&artifact_dir)
        .args(["--record"])
        .arg(&record_path)
        .output()
        .expect("star_sim_debug should start");

    assert_success(&output);
    let result = last_json(&output.stdout);
    let entries: Vec<Value> = fs::read_to_string(&record_path)
        .expect("logical app recording should exist")
        .lines()
        .map(|line| serde_json::from_str(line).expect("recording line should be JSON"))
        .collect();
    let ready = &entries[0]["message"];
    assert!(
        ready["capabilities"]
            .as_array()
            .is_some_and(|values| values.iter().any(|value| value == "screenshot"))
    );
    assert!(
        !ready["capabilities"]
            .as_array()
            .is_some_and(|values| { values.iter().any(|value| value == "camera_focus") })
    );
    assert!(entries.iter().any(|entry| {
        entry["message"]["command"]["type"] == "click"
            && entry["message"]["command"]["target"] == "menu.tab.museum"
    }));
    assert_eq!(result["status"], "passed");
    assert_eq!(result["mode"], "logical");
    assert_eq!(result["state"]["active_screen"], "museum");

    fs::remove_dir_all(artifact_dir).ok();
}

#[test]
#[ignore = "requires a display and a rendered apps/app process"]
fn unsupported_screenshot_sources_do_not_create_artifacts() {
    let artifact_dir = unique_temp_dir("invalid-screenshot");
    let launch = LaunchSpec {
        package: "app".into(),
        kind: LaunchTargetKind::Binary,
        target: "app".into(),
        features: vec!["automation-control".into()],
        arguments: vec!["--automation".into()],
    };
    let options = SessionOptions::new(Duration::from_secs(120))
        .with_recent_logs(RecentLogs::default())
        .with_artifact_dir(artifact_dir.clone());
    let mut session = Session::spawn(&launch, options).expect("apps/app should start");
    session
        .ready(&["screenshot"])
        .expect("apps/app should advertise window screenshots");

    let camera_error = session
        .request(
            "camera-error",
            Command::Screenshot {
                source: ScreenshotSource::Camera {
                    target: "camera.main".into(),
                },
                path: Some("camera.png".into()),
                overwrite: false,
            },
        )
        .expect_err("camera screenshots should be rejected");
    assert_request_error(camera_error, "unsupported_action");
    assert!(
        !artifact_dir.join("camera.png").exists(),
        "rejected camera screenshot must not reserve an artifact"
    );

    let window_error = session
        .request(
            "window-error",
            Command::Screenshot {
                source: ScreenshotSource::Window {
                    target: "window.unknown".into(),
                },
                path: Some("unknown.png".into()),
                overwrite: false,
            },
        )
        .expect_err("unknown window targets should be rejected");
    assert_request_error(window_error, "unknown_target");
    assert!(
        !artifact_dir.join("unknown.png").exists(),
        "rejected window screenshot must not reserve an artifact"
    );

    session
        .shutdown()
        .expect("apps/app should shut down after rejected screenshots");
    fs::remove_dir_all(artifact_dir).ok();
}

#[test]
#[ignore = "requires a display and a rendered apps/app process"]
fn checked_in_app_visual_scenario_writes_primary_window_png() {
    let artifact_dir = unique_temp_dir("visual");
    let output = ProcessCommand::new(env!("CARGO_BIN_EXE_star_sim_debug"))
        .args(["visual", "--artifact-dir"])
        .arg(&artifact_dir)
        .output()
        .expect("star_sim_debug should start");

    assert_success(&output);
    let result = last_json(&output.stdout);
    assert_eq!(result["status"], "passed");
    assert_eq!(result["mode"], "visual");
    let expected_artifact_dir = artifact_dir
        .canonicalize()
        .expect("artifact directory should be canonicalizable");
    assert_eq!(
        result["window_png"],
        expected_artifact_dir
            .join("window.png")
            .to_string_lossy()
            .as_ref()
    );

    let data =
        fs::read(expected_artifact_dir.join("window.png")).expect("window screenshot should exist");
    assert_eq!(&data[..8], b"\x89PNG\r\n\x1a\n");
    assert_eq!(u32::from_be_bytes(data[16..20].try_into().unwrap()), 640);
    assert_eq!(u32::from_be_bytes(data[20..24].try_into().unwrap()), 360);

    fs::remove_dir_all(artifact_dir).ok();
}

fn assert_request_error(error: DriverError, expected_code: &str) {
    match error {
        DriverError::RequestFailed(response) => assert_eq!(
            response.error.as_ref().map(|error| error.code.as_str()),
            Some(expected_code)
        ),
        other => panic!("expected a protocol error response, got {other:?}"),
    }
}

fn assert_success(output: &std::process::Output) {
    assert!(
        output.status.success(),
        "star_sim_debug failed:\nstdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn last_json(stdout: &[u8]) -> Value {
    stdout
        .split(|byte| *byte == b'\n')
        .filter_map(|line| serde_json::from_slice(line).ok())
        .next_back()
        .expect("scenario should print a JSON result")
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after the Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "star-sim-app-e2e-{name}-{}-{nonce}",
        std::process::id()
    ))
}
