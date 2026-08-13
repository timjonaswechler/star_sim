use crate::{Observations, TargetRegistry, WaitCondition};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const DEFAULT_FIXED_STEP_MS: u32 = 50;

/// Generic deterministic run state. Hosts update semantic fields through the narrow setters.
#[derive(Clone, Debug, Resource, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunState {
    pub seed: u64,
    pub paused: bool,
    pub rendered_frames: u64,
    pub simulation_ms: u64,
    pub fixed_step_ms: u32,
    pub active_screen: Option<String>,
    pub selection: Option<String>,
    pub camera_motion_pending: bool,
    pub screenshot_pending: bool,
}

impl RunState {
    pub fn new(seed: u64, fixed_step_ms: u32) -> Self {
        assert!(fixed_step_ms > 0);
        Self {
            seed,
            paused: true,
            rendered_frames: 0,
            simulation_ms: 0,
            fixed_step_ms,
            active_screen: None,
            selection: None,
            camera_motion_pending: false,
            screenshot_pending: false,
        }
    }

    pub fn step_frames(&mut self, count: u32) {
        self.rendered_frames = self.rendered_frames.saturating_add(u64::from(count));
    }

    /// Advances fixed simulation steps, rounding duration up to the next complete fixed step.
    pub fn step_simulation(&mut self, duration_ms: u64) -> u64 {
        let step = u64::from(self.fixed_step_ms);
        let steps = duration_ms.div_ceil(step);
        let advanced = steps.saturating_mul(step);
        self.simulation_ms = self.simulation_ms.saturating_add(advanced);
        advanced
    }

    pub fn observation(&self) -> Value {
        json!({
            "seed": self.seed,
            "paused": self.paused,
            "rendered_frames": self.rendered_frames,
            "simulation_ms": self.simulation_ms,
            "fixed_step_ms": self.fixed_step_ms,
            "active_screen": self.active_screen,
            "selection": self.selection,
            "camera_motion_pending": self.camera_motion_pending,
            "screenshot_pending": self.screenshot_pending,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaitEvaluation {
    pub satisfied: bool,
    pub observation: Value,
}

pub fn evaluate_wait(
    condition: &WaitCondition,
    state: &RunState,
    registry: &TargetRegistry,
    observations: &Observations,
    baseline_frames: u64,
) -> WaitEvaluation {
    let target_observation = |target: &str| {
        observations
            .ui
            .iter()
            .chain(&observations.scene)
            .chain(&observations.selection)
            .chain(&observations.camera)
            .find(|value| value.id == target)
    };
    let (satisfied, observation) = match condition {
        WaitCondition::TargetExists { target } => {
            let exists = registry.contains(target);
            (exists, json!({"target": target, "exists": exists}))
        }
        WaitCondition::TargetVisible { target } => {
            let value = target_observation(target);
            (
                value.is_some_and(|value| value.visible),
                json!({"target": target, "observation": value}),
            )
        }
        WaitCondition::TargetEnabled { target } => {
            let value = target_observation(target);
            (
                value.is_some_and(|value| value.enabled),
                json!({"target": target, "observation": value}),
            )
        }
        WaitCondition::TargetAbsent { target } => {
            let absent = !registry.contains(target);
            (absent, json!({"target": target, "absent": absent}))
        }
        WaitCondition::ActiveScreen { screen } => (
            state.active_screen.as_deref() == Some(screen),
            json!({"expected": screen, "actual": state.active_screen}),
        ),
        WaitCondition::SelectionIs { target } => (
            state.selection.as_deref() == Some(target),
            json!({"expected": target, "actual": state.selection}),
        ),
        WaitCondition::CameraMotionComplete => (
            !state.camera_motion_pending,
            json!({"camera_motion_pending": state.camera_motion_pending}),
        ),
        WaitCondition::ScreenshotComplete => (
            !state.screenshot_pending,
            json!({"screenshot_pending": state.screenshot_pending}),
        ),
        WaitCondition::SimulationPaused => (state.paused, json!({"paused": state.paused})),
        WaitCondition::FramesElapsed { count } => {
            let elapsed = state.rendered_frames.saturating_sub(baseline_frames);
            (
                elapsed >= *count,
                json!({"expected": count, "elapsed": elapsed}),
            )
        }
    };
    WaitEvaluation {
        satisfied,
        observation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AgentTarget, TargetObservation};

    #[test]
    fn pause_and_steps_are_explicit_and_repeatable() {
        let mut first = RunState::new(42, 50);
        let mut second = RunState::new(42, 50);
        for state in [&mut first, &mut second] {
            assert!(state.paused);
            state.step_frames(3);
            assert_eq!(state.step_simulation(120), 150);
        }
        assert_eq!(first, second);
        assert_eq!(
            serde_json::to_vec(&first).unwrap(),
            serde_json::to_vec(&second).unwrap()
        );
    }

    #[test]
    fn closed_wait_conditions_return_relevant_receipts() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<TargetRegistry>();
        app.world_mut()
            .spawn(AgentTarget::new("target", "ui", "Target", ["click"]));
        app.add_systems(Update, crate::target::sync_registry);
        app.update();
        let mut observations = Observations::default();
        observations.ui.push(TargetObservation {
            id: "target".into(),
            role: "ui".into(),
            label: "Target".into(),
            visible: true,
            enabled: false,
            actions: vec!["click".into()],
            bounds: None,
        });
        let state = RunState::new(7, 50);
        let visible = evaluate_wait(
            &WaitCondition::TargetVisible {
                target: "target".into(),
            },
            &state,
            app.world().resource(),
            &observations,
            0,
        );
        assert!(visible.satisfied);
        let enabled = evaluate_wait(
            &WaitCondition::TargetEnabled {
                target: "target".into(),
            },
            &state,
            app.world().resource(),
            &observations,
            0,
        );
        assert!(!enabled.satisfied);
        assert_eq!(enabled.observation["observation"]["enabled"], false);
    }
}
