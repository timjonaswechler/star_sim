use serde_json::{Value, json};
use std::{
    env,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

const TIMEOUT: Duration = Duration::from_secs(60);

fn main() {
    if let Err(error) = run() {
        eprintln!("star_sim_agent: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mode = args
        .next()
        .ok_or("usage: star_sim_agent <logical|visual> [--artifact-dir PATH]")?;
    let artifact_dir = parse_artifact_dir(args.collect())?;
    match mode.as_str() {
        "logical" => run_logical(),
        "visual" => run_visual(artifact_dir),
        _ => Err(format!("unknown mode {mode:?}; expected logical or visual")),
    }
}

fn parse_artifact_dir(args: Vec<String>) -> Result<PathBuf, String> {
    if args.is_empty() {
        return Ok(PathBuf::from("artifacts/agent-ci"));
    }
    if args.len() == 2 && args[0] == "--artifact-dir" {
        return Ok(PathBuf::from(&args[1]));
    }
    Err("expected optional --artifact-dir PATH".into())
}

struct Client {
    child: Child,
    stdin: ChildStdin,
    receiver: mpsc::Receiver<Result<Value, String>>,
}

impl Client {
    fn spawn(mut command: Command) -> Result<Self, String> {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|error| format!("failed to start child: {error}"))?;
        let stdin = child.stdin.take().ok_or("child has no stdin")?;
        let stdout = child.stdout.take().ok_or("child has no stdout")?;
        let receiver = json_reader(stdout);
        Ok(Self {
            child,
            stdin,
            receiver,
        })
    }

    fn ready(&self, required: &[&str]) -> Result<Value, String> {
        let value = self.receive()?;
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
        writeln!(
            self.stdin,
            "{}",
            json!({"version": 1, "id": id, "command": command})
        )
        .and_then(|_| self.stdin.flush())
        .map_err(|error| format!("failed to send {id}: {error}"))?;
        let response = self.receive()?;
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
    }
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

fn run_logical() -> Result<(), String> {
    let mut command = cargo_example("agent_control", "agent_control_headless", &[]);
    command.args(["--agent", "--seed", "42"]);
    let mut client = Client::spawn(command)?;
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

fn run_visual(artifact_dir: PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(&artifact_dir).map_err(|error| error.to_string())?;
    let artifact_dir = artifact_dir
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let mut command = cargo_example("bevy_viewer", "agent_control_prototype", &["agent-control"]);
    command.args(["--agent", "--artifact-dir"]);
    command.arg(&artifact_dir);
    let mut client = Client::spawn(command)?;
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
    fn rejects_unknown_mode_and_bad_options() {
        assert!(parse_artifact_dir(vec!["--bad".into()]).is_err());
        let mut command = Command::new("sh");
        command.args(["-c", "printf 'not-json\\n'"]);
        let client = Client::spawn(command).unwrap();
        assert!(client.receive().unwrap_err().contains("non-JSON stdout"));
    }
}
