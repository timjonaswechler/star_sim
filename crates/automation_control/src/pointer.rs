use crate::entity::Handle;
use bevy::{
    camera::{NormalizedRenderTarget, RenderTarget},
    input::mouse::MouseScrollUnit,
    picking::pointer::{
        Location, PointerAction, PointerButton as BevyPointerButton, PointerId, PointerInput,
    },
    prelude::*,
    window::{PrimaryWindow, Window, WindowRef},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fmt};

/// A pointer-device button, not a semantic UI action.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Button {
    Primary,
    Secondary,
    Middle,
}

pub type PointerButton = Button;
pub type PointerCommand = Command;
pub type PointerState = State;

impl From<Button> for BevyPointerButton {
    fn from(value: Button) -> Self {
        match value {
            Button::Primary => Self::Primary,
            Button::Secondary => Self::Secondary,
            Button::Middle => Self::Middle,
        }
    }
}

/// Virtual pointer transitions accepted by a Controlled Session.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Move {
        surface: Option<Handle>,
        position: [f32; 2],
    },
    Press {
        button: Button,
    },
    Release {
        button: Button,
    },
    Scroll {
        delta: [f32; 2],
    },
}

impl Command {
    pub fn validate(&self) -> Result<(), PointerError> {
        let finite = |values: &[f32; 2]| {
            values
                .iter()
                .all(|value| value.is_finite())
                .then_some(())
                .ok_or(PointerError::NonFinitePosition)
        };
        match self {
            Self::Move { position, .. } => finite(position),
            Self::Scroll { delta } => finite(delta),
            Self::Press { .. } | Self::Release { .. } => Ok(()),
        }
    }
}

/// Session-local virtual pointer state. It is never shared between Controlled Sessions.
#[derive(Clone, Debug, Default, Resource)]
pub struct State {
    pub position: Option<[f32; 2]>,
    pub surface: Option<Handle>,
    pub pressed: BTreeSet<Button>,
}

impl State {
    pub fn is_pressed(&self, button: Button) -> bool {
        self.pressed.contains(&button)
    }

    pub fn observation(&self) -> serde_json::Value {
        serde_json::json!({
            "position": self.position,
            "surface": self.surface,
            "pressed": self.pressed.iter().copied().collect::<Vec<_>>(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PointerError {
    NonFinitePosition,
    NoPrimarySurface,
    AmbiguousPrimarySurface,
    SurfaceNotLive(Handle),
    SurfaceNotWindow(Handle),
    NoLocation,
    ButtonAlreadyPressed(Button),
    ButtonNotPressed(Button),
}

impl fmt::Display for PointerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinitePosition => formatter.write_str("pointer coordinates must be finite"),
            Self::NoPrimarySurface => formatter.write_str("no primary render surface is available"),
            Self::AmbiguousPrimarySurface => {
                formatter.write_str("more than one primary render surface is available")
            }
            Self::SurfaceNotLive(handle) => {
                write!(formatter, "surface handle {handle} is not live")
            }
            Self::SurfaceNotWindow(handle) => {
                write!(formatter, "surface handle {handle} is not a window")
            }
            Self::NoLocation => {
                formatter.write_str("pointer must move to a surface before this action")
            }
            Self::ButtonAlreadyPressed(button) => {
                write!(formatter, "pointer button {button:?} is already pressed")
            }
            Self::ButtonNotPressed(button) => {
                write!(formatter, "pointer button {button:?} is not pressed")
            }
        }
    }
}

impl std::error::Error for PointerError {}

/// A resolved render surface used by the pointer bridge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Surface {
    pub handle: Handle,
    pub target: NormalizedRenderTarget,
}

pub fn resolve_surface(world: &World, requested: Option<Handle>) -> Result<Surface, PointerError> {
    let entity = match requested {
        Some(handle) => {
            let entity = handle
                .resolve(world)
                .map_err(|_| PointerError::SurfaceNotLive(handle))?;
            if world
                .get_entity(entity)
                .ok()
                .and_then(|value| value.get::<Window>())
                .is_none()
            {
                return Err(PointerError::SurfaceNotWindow(handle));
            }
            entity
        }
        None => {
            let mut primary = world
                .iter_entities()
                .filter(|entity| entity.contains::<PrimaryWindow>() && entity.contains::<Window>())
                .map(|entity| entity.id());
            let Some(entity) = primary.next() else {
                return Err(PointerError::NoPrimarySurface);
            };
            if primary.next().is_some() {
                return Err(PointerError::AmbiguousPrimarySurface);
            }
            entity
        }
    };
    let target = RenderTarget::Window(WindowRef::Entity(entity))
        .normalize(Some(entity))
        .ok_or(PointerError::SurfaceNotLive(Handle::from(entity)))?;
    Ok(Surface {
        handle: Handle::from(entity),
        target,
    })
}

pub fn pointer_event(
    state: &mut State,
    world: &World,
    command: &Command,
) -> Result<PointerInput, PointerError> {
    command.validate()?;
    match *command {
        Command::Move { surface, position } => {
            let surface = resolve_surface(world, surface)?;
            let delta = match (state.surface, state.position) {
                (Some(previous), Some(previous_position)) if previous == surface.handle => {
                    Vec2::from(position) - Vec2::from(previous_position)
                }
                _ => Vec2::ZERO,
            };
            state.position = Some(position);
            state.surface = Some(surface.handle);
            Ok(PointerInput::new(
                PointerId::Mouse,
                Location {
                    target: surface.target,
                    position: Vec2::from(position),
                },
                PointerAction::Move { delta },
            ))
        }
        Command::Press { button } => {
            let (surface, position) = current_location(state, world)?;
            if !state.pressed.insert(button) {
                return Err(PointerError::ButtonAlreadyPressed(button));
            }
            Ok(PointerInput::new(
                PointerId::Mouse,
                Location {
                    target: surface.target,
                    position: Vec2::from(position),
                },
                PointerAction::Press(button.into()),
            ))
        }
        Command::Release { button } => {
            let (surface, position) = current_location(state, world)?;
            if !state.pressed.remove(&button) {
                return Err(PointerError::ButtonNotPressed(button));
            }
            Ok(PointerInput::new(
                PointerId::Mouse,
                Location {
                    target: surface.target,
                    position: Vec2::from(position),
                },
                PointerAction::Release(button.into()),
            ))
        }
        Command::Scroll { delta } => {
            let (surface, position) = current_location(state, world)?;
            Ok(PointerInput::new(
                PointerId::Mouse,
                Location {
                    target: surface.target,
                    position: Vec2::from(position),
                },
                PointerAction::Scroll {
                    x: delta[0],
                    y: delta[1],
                    unit: MouseScrollUnit::Line,
                    phase: bevy::input::touch::TouchPhase::Moved,
                },
            ))
        }
    }
}

fn current_location(state: &State, world: &World) -> Result<(Surface, [f32; 2]), PointerError> {
    let position = state.position.ok_or(PointerError::NoLocation)?;
    let handle = state.surface.ok_or(PointerError::NoLocation)?;
    Ok((resolve_surface(world, Some(handle))?, position))
}

/// Ensures the picking core has the virtual mouse pointer even when a small test composition omits
/// Bevy's native PointerInputPlugin.
pub fn ensure_mouse_pointer(mut commands: Commands, pointers: Query<&PointerId>) {
    if !pointers.iter().any(PointerId::is_mouse) {
        commands.spawn(PointerId::Mouse);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_all_pointer_variants_without_a_wire_click() {
        let commands = [
            Command::Move {
                surface: None,
                position: [1.0, 2.0],
            },
            Command::Press {
                button: Button::Primary,
            },
            Command::Release {
                button: Button::Middle,
            },
            Command::Scroll { delta: [0.0, -1.0] },
        ];
        for command in commands {
            assert!(command.validate().is_ok());
        }
        assert!(
            Command::Move {
                surface: None,
                position: [f32::NAN, 0.0]
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn surface_and_button_state_errors_are_typed() {
        let mut world = World::new();
        let mut state = State::default();
        let move_command = Command::Move {
            surface: None,
            position: [10.0, 20.0],
        };
        assert_eq!(
            pointer_event(&mut state, &world, &move_command).unwrap_err(),
            PointerError::NoPrimarySurface
        );

        let first = world.spawn((Window::default(), PrimaryWindow)).id();
        let second = world.spawn((Window::default(), PrimaryWindow)).id();
        assert_eq!(
            pointer_event(&mut state, &world, &move_command).unwrap_err(),
            PointerError::AmbiguousPrimarySurface
        );
        world.despawn(second);

        let non_window = world.spawn_empty().id();
        assert_eq!(
            pointer_event(
                &mut state,
                &world,
                &Command::Move {
                    surface: Some(Handle::from(non_window)),
                    position: [10.0, 20.0],
                },
            )
            .unwrap_err(),
            PointerError::SurfaceNotWindow(Handle::from(non_window))
        );

        pointer_event(
            &mut state,
            &world,
            &Command::Move {
                surface: Some(Handle::from(first)),
                position: [10.0, 20.0],
            },
        )
        .unwrap();
        let press = Command::Press {
            button: Button::Secondary,
        };
        pointer_event(&mut state, &world, &press).unwrap();
        assert_eq!(
            pointer_event(&mut state, &world, &press).unwrap_err(),
            PointerError::ButtonAlreadyPressed(Button::Secondary)
        );
        let release = Command::Release {
            button: Button::Secondary,
        };
        pointer_event(&mut state, &world, &release).unwrap();
        assert_eq!(
            pointer_event(&mut state, &world, &release).unwrap_err(),
            PointerError::ButtonNotPressed(Button::Secondary)
        );
    }
}
