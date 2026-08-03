// PROTOTYPE: compare three physically distinct stellar-activity scenarios in one Bevy view.

use bevy::{
    asset::RenderAssetUsages,
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    diagnostic::FrameCount,
    feathers::{
        FeathersPlugins,
        controls::{
            FeathersMenu, FeathersMenuButton, FeathersMenuItem, FeathersMenuPopup, FeathersSlider,
        },
        dark_theme::create_dark_theme,
        theme::{ThemedText, UiTheme},
    },
    input_focus::tab_navigation::TabGroup,
    mesh::Indices,
    post_process::bloom::Bloom,
    prelude::*,
    render::render_resource::PrimitiveTopology,
    render::view::screenshot::{Screenshot, save_to_disk},
    ui_widgets::{
        Activate, SetSliderValue, SliderPrecision, SliderStep, SliderValueChange, ValueChange,
        slider_self_update,
    },
};
mod plasma_tube_prototype;
use plasma_tube_prototype::{PlasmaTubeMaterial, PlasmaTubeMaterialPlugin, PlasmaTubeUniform};
use star_sim::{
    physics::thermodynamics::color_temperature::black_body_emission,
    rendering::star_material::{
        StarSurfaceMaterial, StarSurfaceMaterialPlugin, procedural_star_surface_material,
    },
};

const INITIAL_TEMPERATURE_K: f64 = 5_772.0;
const INITIAL_MANUAL_COMPENSATION_STOPS: f32 = 0.0;
const COOL_STAR_COMPENSATION_STOPS: f32 = 1.5;
const HOT_STAR_COMPENSATION_STOPS: f32 = 0.2;
const TARGET_HDR_LUMINANCE: f64 = 1.0;
const FLARE_THREAD_COUNT: usize = 9;
const TUBE_RING_COUNT: usize = 64;
const TUBE_SIDE_COUNT: usize = 16;
const SCREENSHOT_PATH: &str = "examples/star_sphere.png";

#[derive(Resource)]
struct StarControls {
    temperature_k: f64,
    manual_compensation_stops: f32,
    limb_darkening: f32,
    granulation_strength: f32,
    activity_scenario: ActivityScenario,
    activity_cycle_seconds: f32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActivityScenario {
    ConfinedFlare,
    EruptiveFlare,
    StableProminence,
}

impl ActivityScenario {
    const fn label(self) -> &'static str {
        match self {
            Self::ConfinedFlare => "Confined flare",
            Self::EruptiveFlare => "Eruptive + prominence",
            Self::StableProminence => "Stable prominence",
        }
    }
}

#[derive(Component)]
struct FlareLoopThread {
    thread_index: usize,
}

#[derive(Component)]
struct ProminenceTube;

#[derive(Component)]
struct FlareRibbonPoint {
    thread_index: usize,
    polarity: f32,
}

#[derive(Resource)]
struct ActivityAnimation {
    elapsed_seconds: f32,
    loop_materials: Vec<Handle<PlasmaTubeMaterial>>,
    prominence_material: Handle<PlasmaTubeMaterial>,
    prominence_mesh: Handle<Mesh>,
    ribbon_material: Handle<StandardMaterial>,
}

#[derive(Component, Clone, Copy, Default)]
struct TemperatureSlider;

#[derive(Component, Clone, Copy, Default)]
enum ReadoutKind {
    #[default]
    Temperature,
    Exposure,
    AutomaticCompensation,
    ExposureCompensation,
    LimbDarkening,
    Granulation,
    ActivityScenario,
    ActivityPhase,
    ActivityCycle,
    Physics,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Interactive stellar surface".into(),
                    resolution: (1_200, 900).into(),
                    ..default()
                }),
                ..default()
            }),
            StarSurfaceMaterialPlugin,
            PlasmaTubeMaterialPlugin,
            FeathersPlugins,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .insert_resource(StarControls {
            temperature_k: INITIAL_TEMPERATURE_K,
            manual_compensation_stops: INITIAL_MANUAL_COMPENSATION_STOPS,
            limb_darkening: 0.6,
            granulation_strength: 0.16,
            activity_scenario: ActivityScenario::EruptiveFlare,
            activity_cycle_seconds: 14.0,
        })
        .add_systems(Startup, (setup, settings_scene.spawn()))
        .add_systems(
            Update,
            (update_star, animate_activity, capture_preview_once),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut star_materials: ResMut<Assets<StarSurfaceMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut plasma_materials: ResMut<Assets<PlasmaTubeMaterial>>,
) {
    let emission = black_body_emission(INITIAL_TEMPERATURE_K)
        .expect("the example temperature must be supported");
    let initial_ev100 = inspection_ev100(emission.photopic_luminance_candelas_per_square_meter)
        - automatic_display_compensation_stops(INITIAL_TEMPERATURE_K);

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(6).unwrap())),
        MeshMaterial3d(star_materials.add(procedural_star_surface_material(emission))),
    ));

    spawn_activity_prototype(
        &mut commands,
        &mut meshes,
        &mut standard_materials,
        &mut plasma_materials,
    );

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        Exposure {
            ev100: initial_ev100,
        },
        Tonemapping::AcesFitted,
        Bloom::NATURAL,
    ));
}

fn spawn_activity_prototype(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    standard_materials: &mut Assets<StandardMaterial>,
    plasma_materials: &mut Assets<PlasmaTubeMaterial>,
) {
    let ribbon_mesh = meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap());
    let ribbon_material = standard_materials.add(activity_material());
    let mut loop_materials = Vec::with_capacity(FLARE_THREAD_COUNT);

    for thread_index in 0..FLARE_THREAD_COUNT {
        let tube_mesh = meshes.add(build_tube_mesh(
            |t| flare_loop_path(t, thread_index),
            0.004,
            1.5,
        ));
        let material = plasma_materials.add(plasma_material(thread_index as f32 * 1.73));
        loop_materials.push(material.clone());
        commands.spawn((
            Mesh3d(tube_mesh),
            MeshMaterial3d(material.clone()),
            Visibility::Hidden,
            FlareLoopThread { thread_index },
        ));

        for polarity in [-1.0, 1.0] {
            commands.spawn((
                Mesh3d(ribbon_mesh.clone()),
                MeshMaterial3d(ribbon_material.clone()),
                Transform::default(),
                Visibility::Hidden,
                FlareRibbonPoint {
                    thread_index,
                    polarity,
                },
            ));
        }
    }

    let prominence_mesh = meshes.add(build_tube_mesh(|t| prominence_path(t, 0.0), 0.007, 2.2));
    let prominence_material = plasma_materials.add(plasma_material(4.2));
    commands.spawn((
        Mesh3d(prominence_mesh.clone()),
        MeshMaterial3d(prominence_material.clone()),
        Visibility::Hidden,
        ProminenceTube,
    ));

    commands.insert_resource(ActivityAnimation {
        elapsed_seconds: 0.0,
        loop_materials,
        prominence_material,
        prominence_mesh,
        ribbon_material,
    });
}

fn plasma_material(phase_offset: f32) -> PlasmaTubeMaterial {
    PlasmaTubeMaterial {
        parameters: PlasmaTubeUniform {
            color: Vec4::ZERO,
            dynamics: Vec4::new(0.0, 1.0, 0.7, phase_offset),
            detail: Vec4::new(6.5, 0.9, 0.0, 0.0),
        },
    }
}

fn activity_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::BLACK,
        unlit: true,
        ..default()
    }
}

fn settings_scene() -> impl SceneList {
    bsn_list![settings_panel()]
}

fn settings_panel() -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: px(18),
            top: px(18),
            width: px(410),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(9),
            padding: px(14),
            border: px(1),
            border_radius: BorderRadius::all(px(10)),
        }
        TabGroup
        BackgroundColor(Color::srgba(0.035, 0.04, 0.055, 0.94))
        BorderColor::all(Color::srgb(0.24, 0.27, 0.34))
        Children [
            (Text("Star settings") ThemedText),
            (
                @FeathersMenu
                Children [
                    (
                        @FeathersMenuButton {
                            @caption: bsn! { Text("Temperature presets") ThemedText }
                        }
                        Node { width: percent(100) }
                    ),
                    (
                        @FeathersMenuPopup
                        Children [
                            preset_item("Red dwarf · 3200 K", 3_200.0),
                            preset_item("Red giant · 3500 K", 3_500.0),
                            preset_item("K-type star · 4500 K", 4_500.0),
                            preset_item("Sun · 5772 K", 5_772.0),
                            preset_item("A-type star · 10000 K", 10_000.0),
                            preset_item("Blue star · 25000 K", 25_000.0),
                        ]
                    )
                ]
            ),
            slider_label("Temperature", "5772 K", ReadoutKind::Temperature),
            (
                @FeathersSlider { @min: 1000.0, @max: 40000.0, @value: 5772.0 }
                TemperatureSlider
                SliderStep(50.0)
                SliderPrecision(0)
                on(slider_self_update)
                on(|change: On<ValueChange<f32>>, mut controls: ResMut<StarControls>| {
                    controls.temperature_k = f64::from(change.value);
                })
            ),
            slider_label("Inspection EV100", "automatic", ReadoutKind::Exposure),
            slider_label(
                "Auto brightness",
                "temperature based",
                ReadoutKind::AutomaticCompensation,
            ),
            slider_label(
                "Manual adjustment",
                "+0.00 stops",
                ReadoutKind::ExposureCompensation,
            ),
            (
                @FeathersSlider {
                    @min: -3.0,
                    @max: 3.0,
                    @value: INITIAL_MANUAL_COMPENSATION_STOPS,
                }
                SliderStep(0.25)
                SliderPrecision(2)
                on(slider_self_update)
                on(|change: On<ValueChange<f32>>, mut controls: ResMut<StarControls>| {
                    controls.manual_compensation_stops = change.value;
                })
            ),
            slider_label("Limb darkening", "0.60", ReadoutKind::LimbDarkening),
            (
                @FeathersSlider { @min: 0.0, @max: 1.0, @value: 0.6 }
                SliderStep(0.01)
                SliderPrecision(2)
                on(slider_self_update)
                on(|change: On<ValueChange<f32>>, mut controls: ResMut<StarControls>| {
                    controls.limb_darkening = change.value;
                })
            ),
            slider_label("Granulation", "0.16", ReadoutKind::Granulation),
            (
                @FeathersSlider { @min: 0.0, @max: 0.5, @value: 0.16 }
                SliderStep(0.01)
                SliderPrecision(2)
                on(slider_self_update)
                on(|change: On<ValueChange<f32>>, mut controls: ResMut<StarControls>| {
                    controls.granulation_strength = change.value;
                })
            ),
            (
                @FeathersMenu
                Children [
                    (
                        @FeathersMenuButton {
                            @caption: bsn! { Text("Activity scenario") ThemedText }
                        }
                        Node { width: percent(100) }
                    ),
                    (
                        @FeathersMenuPopup
                        Children [
                            activity_scenario_item("Confined flare", ActivityScenario::ConfinedFlare),
                            activity_scenario_item("Eruptive flare + prominence", ActivityScenario::EruptiveFlare),
                            activity_scenario_item("Stable prominence", ActivityScenario::StableProminence),
                        ]
                    )
                ]
            ),
            slider_label(
                "Scenario",
                "Eruptive flare + prominence",
                ReadoutKind::ActivityScenario,
            ),
            slider_label("Activity phase", "Energy build-up", ReadoutKind::ActivityPhase),
            slider_label("Activity cycle", "14.0 s", ReadoutKind::ActivityCycle),
            (
                @FeathersSlider { @min: 6.0, @max: 30.0, @value: 14.0 }
                SliderStep(0.25)
                SliderPrecision(2)
                on(slider_self_update)
                on(|change: On<ValueChange<f32>>, mut controls: ResMut<StarControls>| {
                    controls.activity_cycle_seconds = change.value;
                })
            ),
            (
                Text("Calculating emission…")
                ThemedText
                template_value(ReadoutKind::Physics)
            ),
        ]
    }
}

fn preset_item(label: &'static str, temperature_k: f64) -> impl Scene {
    bsn! {
        @FeathersMenuItem {
            @caption: bsn! { Text(label) ThemedText }
        }
        on(move |_: On<Activate>, mut commands: Commands, slider: Single<Entity, With<TemperatureSlider>>| {
            commands.trigger(SetSliderValue {
                entity: *slider,
                change: SliderValueChange::Absolute(temperature_k as f32),
            });
        })
    }
}

fn activity_scenario_item(label: &'static str, scenario: ActivityScenario) -> impl Scene {
    bsn! {
        @FeathersMenuItem {
            @caption: bsn! { Text(label) ThemedText }
        }
        on(move |_: On<Activate>, mut controls: ResMut<StarControls>, mut animation: ResMut<ActivityAnimation>| {
            controls.activity_scenario = scenario;
            animation.elapsed_seconds = 0.0;
        })
    }
}

fn slider_label(label: &'static str, value: &'static str, kind: ReadoutKind) -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
        }
        Children [
            (Text(label) ThemedText),
            (Text(value) ThemedText template_value(kind)),
        ]
    }
}

fn update_star(
    controls: Res<StarControls>,
    material_handle: Single<&MeshMaterial3d<StarSurfaceMaterial>>,
    mut materials: ResMut<Assets<StarSurfaceMaterial>>,
    mut camera_exposure: Single<&mut Exposure, With<Camera3d>>,
    mut readouts: Query<(&mut Text, &ReadoutKind)>,
) {
    if !controls.is_changed() {
        return;
    }

    let emission = black_body_emission(controls.temperature_k)
        .expect("the controls must produce a supported temperature");
    let mut material = materials
        .get_mut(&material_handle.0)
        .expect("the star material must exist");
    let mut updated = procedural_star_surface_material(emission);
    updated.parameters.limb_darkening = controls.limb_darkening;
    updated.parameters.granulation_strength = controls.granulation_strength;
    *material = updated;
    let automatic_ev100 = inspection_ev100(emission.photopic_luminance_candelas_per_square_meter);
    let automatic_compensation = automatic_display_compensation_stops(controls.temperature_k);
    let effective_ev100 =
        automatic_ev100 - automatic_compensation - controls.manual_compensation_stops;
    camera_exposure.ev100 = effective_ev100;
    for (mut text, kind) in &mut readouts {
        **text = match kind {
            ReadoutKind::Temperature => format!("{:.0} K", controls.temperature_k),
            ReadoutKind::Exposure => format!("{effective_ev100:.2} EV (auto {automatic_ev100:.2})"),
            ReadoutKind::AutomaticCompensation => format!("{automatic_compensation:+.2} stops"),
            ReadoutKind::ExposureCompensation => {
                format!("{:+.2} stops", controls.manual_compensation_stops)
            }
            ReadoutKind::LimbDarkening => format!("{:.2}", controls.limb_darkening),
            ReadoutKind::Granulation => format!("{:.2}", controls.granulation_strength),
            ReadoutKind::ActivityScenario => controls.activity_scenario.label().into(),
            ReadoutKind::ActivityPhase => continue,
            ReadoutKind::ActivityCycle => format!("{:.2} s", controls.activity_cycle_seconds),
            ReadoutKind::Physics => format!(
                "Surface: {:.2e} W/m^2\nVisible luminance: {:.2e} cd/m^2",
                emission.radiant_exitance_watts_per_square_meter,
                emission.photopic_luminance_candelas_per_square_meter,
            ),
        };
    }
}

fn animate_activity(
    time: Res<Time>,
    controls: Res<StarControls>,
    mut animation: ResMut<ActivityAnimation>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut plasma_materials: ResMut<Assets<PlasmaTubeMaterial>>,
    mut geometry: ParamSet<(
        Query<(&FlareLoopThread, &mut Visibility)>,
        Query<&mut Visibility, With<ProminenceTube>>,
        Query<(&FlareRibbonPoint, &mut Transform, &mut Visibility)>,
    )>,
    mut readouts: Query<(&mut Text, &ReadoutKind)>,
) {
    animation.elapsed_seconds += time.delta_secs();
    let phase =
        (animation.elapsed_seconds / controls.activity_cycle_seconds.max(0.1)).rem_euclid(1.0);
    let flare_enabled = controls.activity_scenario != ActivityScenario::StableProminence;
    let eruptive = controls.activity_scenario == ActivityScenario::EruptiveFlare;
    let mut thread_brightness = [0.0; FLARE_THREAD_COUNT];

    let mut thread_fill = [0.0; FLARE_THREAD_COUNT];
    for (thread, mut visibility) in &mut geometry.p0() {
        let birth_phase = 0.28 + thread.thread_index as f32 * 0.038;
        let thread_age = ((phase - birth_phase) / (1.0 - birth_phase)).max(0.0);
        let filling = smooth_step(0.0, 0.2, thread_age);
        let heating = smooth_step(0.0, 0.08, thread_age);
        let cooling = 1.0 - smooth_step(0.5, 0.95, thread_age);
        let brightness = heating * cooling;
        thread_brightness[thread.thread_index] = brightness;
        thread_fill[thread.thread_index] = filling;
        *visibility = if flare_enabled && phase >= birth_phase && brightness > 0.015 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    let prominence_fade = if eruptive {
        1.0 - smooth_step(0.76, 0.93, phase)
    } else {
        1.0
    };
    let prominence_lift = if eruptive {
        0.08 * smooth_step(0.12, 0.3, phase) + 0.68 * smooth_step(0.28, 0.74, phase)
    } else {
        0.015 * (animation.elapsed_seconds * 0.8).sin()
    };
    let show_prominence =
        controls.activity_scenario != ActivityScenario::ConfinedFlare && prominence_fade > 0.02;
    for mut visibility in &mut geometry.p1() {
        *visibility = if show_prominence {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    *meshes
        .get_mut(&animation.prominence_mesh)
        .expect("the prominence tube mesh must exist") =
        build_tube_mesh(|t| prominence_path(t, prominence_lift), 0.007, 2.2);

    let stellar_emission = black_body_emission(controls.temperature_k)
        .expect("the controls must produce a supported temperature");
    let hot_loop_chromaticity: LinearRgba =
        black_body_emission((controls.temperature_k * 1.35).clamp(1_000.0, 40_000.0))
            .expect("the derived loop temperature must be supported")
            .chromaticity
            .into();
    // Cool prominence plasma is not a black body. In visible-light imagery its
    // characteristic color is dominated by hydrogen-alpha emission at 656.3 nm.
    let prominence_chromaticity = LinearRgba::rgb(1.0, 0.12, 0.018);
    let camera_ev100 =
        inspection_ev100(stellar_emission.photopic_luminance_candelas_per_square_meter)
            - automatic_display_compensation_stops(controls.temperature_k)
            - controls.manual_compensation_stops;
    let exposed_surface_luminance = stellar_emission.photopic_luminance_candelas_per_square_meter
        as f32
        * (2.0_f32.powf(-camera_ev100) / 1.2);

    for (thread_index, material_handle) in animation.loop_materials.iter().enumerate() {
        let mut material = plasma_materials
            .get_mut(material_handle)
            .expect("every flare loop material must exist");
        material.parameters.color =
            hdr_color(hot_loop_chromaticity, exposed_surface_luminance * 0.65);
        material.parameters.dynamics.x = thread_brightness[thread_index];
        material.parameters.dynamics.y = thread_fill[thread_index];
        material.parameters.dynamics.z = 0.75 + thread_index as f32 * 0.035;
    }

    let prominence_strength = if controls.activity_scenario == ActivityScenario::StableProminence {
        0.55
    } else {
        (0.45 + 1.55 * smooth_step(0.16, 0.52, phase)) * prominence_fade
    };
    let mut prominence_material = plasma_materials
        .get_mut(&animation.prominence_material)
        .expect("the prominence material must exist");
    prominence_material.parameters.color =
        hdr_color(prominence_chromaticity, exposed_surface_luminance * 0.32);
    prominence_material.parameters.dynamics.x = prominence_strength;
    prominence_material.parameters.dynamics.y = 1.0;
    prominence_material.parameters.dynamics.z = if eruptive { 0.42 } else { 0.18 };

    let ribbon_strength = if flare_enabled {
        smooth_step(0.25, 0.34, phase) * (1.0 - smooth_step(0.62, 0.8, phase))
    } else {
        0.0
    };
    for (point, mut transform, mut visibility) in &mut geometry.p2() {
        let birth_phase = 0.25 + point.thread_index as f32 * 0.038;
        *visibility = if flare_enabled && phase >= birth_phase && ribbon_strength > 0.02 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        let t = if point.polarity < 0.0 { 0.0 } else { 1.0 };
        transform.translation = flare_loop_path(t, point.thread_index) * 1.002;
        transform.scale = Vec3::splat(0.026 + 0.012 * ribbon_strength);
    }
    let mut ribbon_material = standard_materials
        .get_mut(&animation.ribbon_material)
        .expect("the ribbon material must exist");
    set_hdr_base_color(
        &mut ribbon_material,
        hot_loop_chromaticity,
        exposed_surface_luminance * ribbon_strength * 5.0,
    );

    let phase_label = activity_phase_label(controls.activity_scenario, phase);
    for (mut text, kind) in &mut readouts {
        if matches!(kind, ReadoutKind::ActivityPhase) {
            **text = phase_label.into();
        }
    }
}

fn hdr_color(chromaticity: LinearRgba, luminance: f32) -> Vec4 {
    let chromaticity_luminance =
        0.2126 * chromaticity.red + 0.7152 * chromaticity.green + 0.0722 * chromaticity.blue;
    let scale = if chromaticity_luminance > 0.0 {
        luminance / chromaticity_luminance
    } else {
        0.0
    };
    Vec4::new(
        chromaticity.red * scale,
        chromaticity.green * scale,
        chromaticity.blue * scale,
        1.0,
    )
}

fn set_hdr_base_color(material: &mut StandardMaterial, chromaticity: LinearRgba, luminance: f32) {
    material.base_color =
        LinearRgba::from_f32_array(hdr_color(chromaticity, luminance).to_array()).into();
}

fn activity_phase_label(scenario: ActivityScenario, phase: f32) -> &'static str {
    if scenario == ActivityScenario::StableProminence {
        return "Quiescent magnetic support";
    }
    match phase {
        value if value < 0.18 => "Magnetic energy build-up",
        value if value < 0.28 => "Precursor / flux-rope rise",
        value if value < 0.46 => "Impulsive reconnection",
        value if value < 0.68 => "Chromospheric response / hot loops",
        value if value < 0.93 => "Cooling arcade",
        _ => "Active-region recovery",
    }
}

fn flare_loop_path(t: f32, thread_index: usize) -> Vec3 {
    let anchor = Vec3::new(0.5, 0.34, 0.79).normalize();
    let tangent = Vec3::Y.cross(anchor).normalize();
    let bitangent = anchor.cross(tangent).normalize();
    let thread_fraction = thread_index as f32 / (FLARE_THREAD_COUNT - 1) as f32;
    let lateral_offset = (thread_fraction - 0.5) * 0.13;
    let half_span = 0.2 + 0.09 * thread_fraction;
    let signed_span = (t - 0.5) * 2.0 * half_span;
    let surface_direction =
        (anchor + tangent * signed_span + bitangent * lateral_offset).normalize();
    let arch =
        (std::f32::consts::PI * t).sin().max(0.0).powf(0.82) * (0.11 + 0.28 * thread_fraction);
    surface_direction * (1.012 + arch)
}

fn prominence_path(t: f32, lift: f32) -> Vec3 {
    let anchor = Vec3::new(0.51, 0.4, 0.76).normalize();
    let tangent = Vec3::Y.cross(anchor).normalize();
    let bitangent = anchor.cross(tangent).normalize();
    let signed_span = (t - 0.5) * 0.72;
    let surface_direction = (anchor + tangent * signed_span).normalize();
    let arch_shape = (std::f32::consts::PI * t).sin().max(0.0).powf(0.68);
    let magnetic_twist = bitangent * (std::f32::consts::TAU * t).sin() * 0.028;
    surface_direction * (1.014 + arch_shape * (0.28 + lift)) + magnetic_twist
}

/// Builds a connected flux tube. Its cross-section expands toward the apex as
/// a visual proxy for magnetic-flux conservation (A * B is approximately constant).
fn build_tube_mesh(path: impl Fn(f32) -> Vec3, footpoint_radius: f32, apex_expansion: f32) -> Mesh {
    let mut positions = Vec::with_capacity((TUBE_RING_COUNT + 1) * (TUBE_SIDE_COUNT + 1));
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut uvs = Vec::with_capacity(positions.capacity());
    let mut indices = Vec::with_capacity(TUBE_RING_COUNT * TUBE_SIDE_COUNT * 6);

    for ring in 0..=TUBE_RING_COUNT {
        let t = ring as f32 / TUBE_RING_COUNT as f32;
        let center = path(t);
        let previous = path((t - 0.002).max(0.0));
        let next = path((t + 0.002).min(1.0));
        let tangent = (next - previous).normalize_or_zero();
        let radial = center.normalize_or_zero();
        let mut binormal = tangent.cross(radial).normalize_or_zero();
        if binormal.length_squared() < 0.5 {
            binormal = tangent.any_orthonormal_vector();
        }
        let normal_axis = binormal.cross(tangent).normalize_or_zero();
        let apex_weight = (std::f32::consts::PI * t).sin().max(0.0).powf(0.7);
        let local_structure = 0.86
            + 0.13 * (t * std::f32::consts::TAU * 5.0 + 0.7).sin()
            + 0.07 * (t * std::f32::consts::TAU * 13.0).sin();
        let radius = footpoint_radius * (1.0 + apex_expansion * apex_weight) * local_structure;

        for side in 0..=TUBE_SIDE_COUNT {
            let angle = std::f32::consts::TAU * side as f32 / TUBE_SIDE_COUNT as f32;
            let surface_normal = normal_axis * angle.cos() + binormal * angle.sin();
            positions.push((center + surface_normal * radius).to_array());
            normals.push(surface_normal.to_array());
            uvs.push([t, side as f32 / TUBE_SIDE_COUNT as f32]);
        }
    }

    let stride = TUBE_SIDE_COUNT + 1;
    for ring in 0..TUBE_RING_COUNT {
        for side in 0..TUBE_SIDE_COUNT {
            let a = (ring * stride + side) as u32;
            let b = ((ring + 1) * stride + side) as u32;
            indices.extend_from_slice(&[a, b, a + 1, a + 1, b, b + 1]);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn smooth_step(edge0: f32, edge1: f32, value: f32) -> f32 {
    let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Maps the stellar surface luminance to linear HDR white in Bevy's EV100 convention.
fn inspection_ev100(photopic_luminance_cd_m2: f64) -> f32 {
    (photopic_luminance_cd_m2 / (1.2 * TARGET_HDR_LUMINANCE)).log2() as f32
}

/// Perceptual preview calibration from the two visually selected endpoint values.
fn automatic_display_compensation_stops(temperature_k: f64) -> f32 {
    let logarithmic_position = (temperature_k / 1_000.0).ln() / (40_000.0_f64 / 1_000.0).ln();
    COOL_STAR_COMPENSATION_STOPS
        + (HOT_STAR_COMPENSATION_STOPS - COOL_STAR_COMPENSATION_STOPS)
            * logarithmic_position.clamp(0.0, 1.0) as f32
}

fn capture_preview_once(
    mut commands: Commands,
    frame_count: Res<FrameCount>,
    mut captured: Local<bool>,
) {
    if !*captured && frame_count.0 >= 360 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(SCREENSHOT_PATH));
        *captured = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_compensation_matches_the_visual_calibration_endpoints() {
        assert!((automatic_display_compensation_stops(1_000.0) - 1.5).abs() < f32::EPSILON);
        assert!((automatic_display_compensation_stops(40_000.0) - 0.2).abs() < f32::EPSILON);
    }
}
