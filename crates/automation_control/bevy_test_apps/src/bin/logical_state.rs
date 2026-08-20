use bevy::{prelude::*, time::Fixed};
use bevy_test_apps::{LogicalSurface, composition};

#[cfg(feature = "automation")]
use automation_control::AutomationTarget;

const SURFACE_WIDTH: u32 = 640;
const SURFACE_HEIGHT: u32 = 360;
const FIXED_STEP: std::time::Duration = std::time::Duration::from_millis(10);
const TIMER_PERIOD: std::time::Duration = std::time::Duration::from_millis(40);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SessionObservation {
    updates: u64,
    fixed_updates: u64,
    timer_finishes: u64,
    pointer_presses: u64,
    key_a_held: bool,
    key_a_presses: u64,
    key_a_releases: u64,
}

#[derive(Resource)]
struct UpdateTimer(Timer);

fn main() {
    build_app().run();
}

fn build_app() -> App {
    let mut app = App::new();
    composition::logical(&mut app, LogicalSurface::new(SURFACE_WIDTH, SURFACE_HEIGHT));
    app.world_mut()
        .resource_mut::<Time<Fixed>>()
        .set_timestep(FIXED_STEP);
    app.register_type::<SessionObservation>()
        .insert_resource(UpdateTimer(Timer::new(TIMER_PERIOD, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, record_update)
        .add_systems(FixedUpdate, record_fixed_update);
    app
}

fn setup(mut commands: Commands) {
    commands.spawn((Name::new("logical-camera"), Camera2d));
    commands.spawn((
        Name::new("logical-state"),
        SessionObservation {
            updates: 0,
            fixed_updates: 0,
            timer_finishes: 0,
            pointer_presses: 0,
            key_a_held: false,
            key_a_presses: 0,
            key_a_releases: 0,
        },
        #[cfg(feature = "automation")]
        AutomationTarget,
    ));
    commands
        .spawn((
            Name::new("logical-button"),
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: px(220),
                top: px(130),
                width: px(200),
                height: px(100),
                ..default()
            },
            #[cfg(feature = "automation")]
            AutomationTarget,
        ))
        .observe(
            |_: On<Pointer<Press>>, mut state: Single<&mut SessionObservation>| {
                state.pointer_presses += 1;
            },
        );
}

fn record_update(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut timer: ResMut<UpdateTimer>,
    mut state: Single<&mut SessionObservation>,
) {
    state.updates += 1;
    state.key_a_held = keys.pressed(KeyCode::KeyA);
    if keys.just_pressed(KeyCode::KeyA) {
        state.key_a_presses += 1;
    }
    if keys.just_released(KeyCode::KeyA) {
        state.key_a_releases += 1;
    }
    timer.0.tick(time.delta());
    state.timer_finishes += u64::from(timer.0.times_finished_this_tick());
}

fn record_fixed_update(mut state: Single<&mut SessionObservation>) {
    state.fixed_updates += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_state_app_has_one_reflectable_state_and_fixed_surface() {
        let mut app = build_app();
        app.world_mut().run_schedule(Startup);

        let mut states = app.world_mut().query::<(&Name, &SessionObservation)>();
        let (name, state) = states.single(app.world()).unwrap();
        assert_eq!(name.as_str(), "logical-state");
        assert_eq!(state.pointer_presses, 0);
        assert_eq!(app.world().resource::<Time<Fixed>>().timestep(), FIXED_STEP);
    }
}
