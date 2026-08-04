// Interactive preview of the procedural stellar surface.

use bevy::{
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    feathers::{
        FeathersPlugins,
        controls::{
            FeathersMenu, FeathersMenuButton, FeathersMenuItem, FeathersMenuPopup, FeathersSlider,
        },
        dark_theme::create_dark_theme,
        theme::{ThemedText, UiTheme},
    },
    input_focus::tab_navigation::TabGroup,
    post_process::bloom::Bloom,
    prelude::*,
    ui_widgets::{
        Activate, SetSliderValue, SliderPrecision, SliderStep, SliderValueChange, ValueChange,
        slider_self_update,
    },
};
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

#[derive(Resource)]
struct StarControls {
    temperature_k: f64,
    manual_compensation_stops: f32,
    limb_darkening: f32,
    granulation_strength: f32,
    display_mode: SurfaceDisplayMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum SurfaceDisplayMode {
    Surface = 0,
    DipoleNoise = 1,
}

impl SurfaceDisplayMode {
    const fn label(self) -> &'static str {
        match self {
            Self::Surface => "Surface",
            Self::DipoleNoise => "Dipole noise",
        }
    }
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
    DisplayMode,
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
            FeathersPlugins,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .insert_resource(StarControls {
            temperature_k: INITIAL_TEMPERATURE_K,
            manual_compensation_stops: INITIAL_MANUAL_COMPENSATION_STOPS,
            limb_darkening: 0.6,
            granulation_strength: 0.16,
            display_mode: SurfaceDisplayMode::Surface,
        })
        .add_systems(Startup, (setup, settings_scene.spawn()))
        .add_systems(Update, update_star)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut star_materials: ResMut<Assets<StarSurfaceMaterial>>,
) {
    let emission = black_body_emission(INITIAL_TEMPERATURE_K)
        .expect("the example temperature must be supported");
    let initial_ev100 = inspection_ev100(emission.photopic_luminance_candelas_per_square_meter)
        - automatic_display_compensation_stops(INITIAL_TEMPERATURE_K);

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(6).unwrap())),
        MeshMaterial3d(star_materials.add(procedural_star_surface_material(emission))),
    ));

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
                            @caption: bsn! { Text("Display mode") ThemedText }
                        }
                        Node { width: percent(100) }
                    ),
                    (
                        @FeathersMenuPopup
                        Children [
                            display_mode_item("Surface", SurfaceDisplayMode::Surface),
                            display_mode_item("Dipole noise", SurfaceDisplayMode::DipoleNoise),
                        ]
                    )
                ]
            ),
            slider_label("View", "Surface", ReadoutKind::DisplayMode),
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

fn display_mode_item(label: &'static str, mode: SurfaceDisplayMode) -> impl Scene {
    bsn! {
        @FeathersMenuItem {
            @caption: bsn! { Text(label) ThemedText }
        }
        on(move |_: On<Activate>, mut controls: ResMut<StarControls>| {
            controls.display_mode = mode;
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
    updated.parameters.display_mode = controls.display_mode as u32;
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
            ReadoutKind::DisplayMode => controls.display_mode.label().into(),
            ReadoutKind::Physics => format!(
                "Surface: {:.2e} W/m^2\nVisible luminance: {:.2e} cd/m^2",
                emission.radiant_exitance_watts_per_square_meter,
                emission.photopic_luminance_candelas_per_square_meter,
            ),
        };
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_compensation_matches_the_visual_calibration_endpoints() {
        assert!((automatic_display_compensation_stops(1_000.0) - 1.5).abs() < f32::EPSILON);
        assert!((automatic_display_compensation_stops(40_000.0) - 0.2).abs() < f32::EPSILON);
    }
}
