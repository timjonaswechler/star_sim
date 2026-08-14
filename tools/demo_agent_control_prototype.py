#!/usr/bin/env python3
"""Run the issue #35 optional agent-control camera/screenshot demonstration."""
from __future__ import annotations
import json, shutil, struct, subprocess, sys, tempfile, time
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def read_json_line(process, timeout=30.0):
    deadline=time.monotonic()+timeout
    while time.monotonic()<deadline:
        line=process.stdout.readline()
        if line:
            try: return json.loads(line)
            except json.JSONDecodeError as error: raise AssertionError(f"stdout was not JSONL: {line!r}") from error
        if process.poll() is not None: raise AssertionError(f"demonstration exited early with {process.returncode}")
    raise TimeoutError("timed out waiting for JSONL response")

def send(process, identifier, command):
    process.stdin.write(json.dumps({"version":1,"id":identifier,"command":command},separators=(",",":"))+"\n"); process.stdin.flush()
    return read_json_line(process)

def png_size(path):
    data=path.read_bytes(); assert data[:8]==b"\x89PNG\r\n\x1a\n" and data[12:16]==b"IHDR"
    return struct.unpack(">II",data[16:24])

def completed(response): assert response["status"]=="completed", response; return response

def main():
    artifact=Path(tempfile.mkdtemp(prefix="star-sim-agent-35-"))
    process=subprocess.Popen(["cargo","run","-q","-p","bevy_viewer","--example","agent_control_prototype","--features","agent-control","--","--agent","--artifact-dir",str(artifact)],cwd=ROOT,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,bufsize=1)
    assert process.stdin and process.stdout and process.stderr
    try:
        ready=read_json_line(process,60); assert ready["type"]=="ready"
        inspected=completed(send(process,"inspect",{"type":"inspect_ui"})); assert any(x["id"]=="toolbar.generate" for x in inspected["result"]["elements"])
        completed(send(process,"click",{"type":"click","target":"toolbar.generate"}))
        # Nonzero duration returns only after five deterministic 50ms adapter steps.
        completed(send(process,"focus",{"type":"camera_focus","camera":"camera.main","target":"scene.prototype_star"}))
        completed(send(process,"orbit",{"type":"camera_orbit","camera":"camera.main","mode":"relative","yaw_deg":20,"pitch_deg":-10,"duration_ms":250}))
        completed(send(process,"pan",{"type":"camera_pan","camera":"camera.main","mode":"relative","offset":{"space":"viewport_normalized","x":0.55,"y":0.45},"duration_ms":0}))
        completed(send(process,"zoom",{"type":"camera_zoom","camera":"camera.main","mode":"absolute","value":4.0,"duration_ms":0}))
        invalid=send(process,"escape",{"type":"screenshot","source":{"type":"window","target":"window.primary"},"path":"../escape.png"}); assert invalid["error"]["code"]=="invalid_artifact_path"
        window=completed(send(process,"window-shot",{"type":"screenshot","source":{"type":"window","target":"window.primary"},"path":"screenshots/window.png"}))
        camera=completed(send(process,"camera-shot",{"type":"screenshot","source":{"type":"camera","target":"camera.main"},"path":"screenshots/camera.png"}))
        window_path=Path(window["result"]["path"]); camera_path=Path(camera["result"]["path"])
        assert window_path.is_absolute() and window_path.is_relative_to(artifact.resolve()) and png_size(window_path)==(640,360)
        assert camera_path.is_absolute() and camera_path.is_relative_to(artifact.resolve()) and png_size(camera_path)==(320,180)
        overwrite=send(process,"overwrite",{"type":"screenshot","source":{"type":"window","target":"window.primary"},"path":"screenshots/window.png"}); assert overwrite["error"]["code"]=="invalid_artifact_path"
        completed(send(process,"shutdown",{"type":"shutdown"})); assert process.wait(timeout=20)==0
        print(json.dumps({"status":"passed","window_png":str(window_path),"window_size":[640,360],"camera_png":str(camera_path),"camera_size":[320,180]})); return 0
    finally:
        if process.poll() is None: process.terminate(); process.wait(timeout=5)
        stderr=process.stderr.read()
        if stderr: print(stderr,end="",file=sys.stderr)
        if "--keep-artifacts" not in sys.argv: shutil.rmtree(artifact,ignore_errors=True)
if __name__=="__main__": raise SystemExit(main())
