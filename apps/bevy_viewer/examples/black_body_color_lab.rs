// PROTOTYPE: compare normalized black-body chromaticity with a photopic preview.

use bevy::{
    feathers::{
        FeathersPlugins,
        controls::FeathersSlider,
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    prelude::*,
    ui_widgets::{SliderPrecision, SliderStep, ValueChange, slider_self_update},
};
use bevy_viewer::color_temperature::{
    MAX_ACCURATE_COLOR_TEMPERATURE_K, MAX_COLOR_TEMPERATURE_K, MIN_ACCURATE_COLOR_TEMPERATURE_K,
    MIN_COLOR_TEMPERATURE_K, black_body_visible_srgb, kelvin_to_srgb,
};

const INITIAL_KELVIN: f32 = 6_500.0;
const INITIAL_EXPOSURE_EV: f32 = 0.0;
const SLIDER_MIN_KELVIN: f32 = MIN_COLOR_TEMPERATURE_K as f32;
const SLIDER_MAX_KELVIN: f32 = MAX_COLOR_TEMPERATURE_K as f32;

#[derive(Resource)]
struct Temperature(f64);

#[derive(Resource)]
struct ExposureEv(f64);

#[derive(Component, Clone, Copy, Default)]
enum SwatchKind {
    #[default]
    Chromaticity,
    Perceived,
}

#[derive(Component, Clone, Copy, Default)]
enum ReadoutKind {
    #[default]
    Kelvin,
    Exposure,
    Chromaticity,
    Perceived,
    Status,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Black-body chromaticity and perception · Feathers prototype".into(),
                    resolution: (1_080, 720).into(),
                    ..default()
                }),
                ..default()
            }),
            FeathersPlugins,
        ))
        .insert_resource(UiTheme(create_dark_theme()))
        .insert_resource(Temperature(f64::from(INITIAL_KELVIN)))
        .insert_resource(ExposureEv(f64::from(INITIAL_EXPOSURE_EV)))
        .add_systems(Startup, scene.spawn())
        .add_systems(Update, update_preview)
        .run();
}

fn scene() -> impl SceneList {
    bsn_list![Camera2d, color_lab()]
}

fn color_lab() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: px(24),
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children [(
            Node {
                width: percent(100),
                max_width: px(980),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                row_gap: px(14),
                padding: px(20),
                border: px(1),
                border_radius: BorderRadius::all(px(12)),
            }
            BackgroundColor(Color::srgb(0.075, 0.08, 0.095))
            BorderColor::all(Color::srgb(0.22, 0.24, 0.29))
            Children [
                (Text("Black-body color lab") ThemedText),
                (
                    Text("The left side isolates hue; the right side retains visible spectral intensity.")
                    ThemedText
                ),
                (
                    Node {
                        width: percent(100),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Stretch,
                        column_gap: px(14),
                    }
                    Children [
                        preview_panel(
                            "Chromaticity",
                            "Normalized color — brightness intentionally removed",
                            SwatchKind::Chromaticity,
                            ReadoutKind::Chromaticity,
                        ),
                        preview_panel(
                            "Photopic preview",
                            "Visible spectrum — 18% middle-gray exposure",
                            SwatchKind::Perceived,
                            ReadoutKind::Perceived,
                        ),
                    ]
                ),
                (
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                    }
                    Children [
                        (Text("0 K") ThemedText),
                        (Text("6500 K") ThemedText template_value(ReadoutKind::Kelvin)),
                        (Text("40000 K") ThemedText),
                    ]
                ),
                (
                    @FeathersSlider {
                        @min: SLIDER_MIN_KELVIN,
                        @max: SLIDER_MAX_KELVIN,
                        @value: INITIAL_KELVIN,
                    }
                    SliderStep(50.0)
                    SliderPrecision(0)
                    on(slider_self_update)
                    on(|change: On<ValueChange<f32>>, mut temperature: ResMut<Temperature>| {
                        temperature.0 = f64::from(change.value);
                    })
                ),
                (
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                    }
                    Children [
                        (Text("Exposure (right preview only)") ThemedText),
                        (Text("0.0 EV · 1×") ThemedText template_value(ReadoutKind::Exposure)),
                    ]
                ),
                (
                    @FeathersSlider {
                        @min: -40.0,
                        @max: 40.0,
                        @value: INITIAL_EXPOSURE_EV,
                    }
                    SliderStep(0.5)
                    SliderPrecision(1)
                    on(slider_self_update)
                    on(|change: On<ValueChange<f32>>, mut exposure: ResMut<ExposureEv>| {
                        exposure.0 = f64::from(change.value);
                    })
                ),
                (
                    Text("Photopic CIE approximation; area, distance, emissivity, and eye adaptation are not included yet.")
                    ThemedText
                    template_value(ReadoutKind::Status)
                ),
            ]
        )]
    }
}

fn preview_panel(
    title: &'static str,
    description: &'static str,
    swatch: SwatchKind,
    readout: ReadoutKind,
) -> impl Scene {
    bsn! {
        Node {
            flex_grow: 1.0,
            flex_basis: px(0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            padding: px(12),
            border: px(1),
            border_radius: BorderRadius::all(px(8)),
        }
        BackgroundColor(Color::srgb(0.055, 0.06, 0.072))
        BorderColor::all(Color::srgb(0.19, 0.21, 0.25))
        Children [
            (Text(title) ThemedText),
            (Text(description) ThemedText),
            (
                Node {
                    width: percent(100),
                    height: px(235),
                    border: px(1),
                    border_radius: BorderRadius::all(px(7)),
                }
                BackgroundColor(Color::BLACK)
                BorderColor::all(Color::srgb(0.32, 0.34, 0.4))
                template_value(swatch)
            ),
            (
                Text("sRGB(0.000, 0.000, 0.000)")
                ThemedText
                template_value(readout)
            ),
        ]
    }
}

fn update_preview(
    temperature: Res<Temperature>,
    exposure: Res<ExposureEv>,
    mut swatches: Query<(&mut BackgroundColor, &SwatchKind)>,
    mut readouts: Query<(&mut Text, &ReadoutKind)>,
) {
    if !temperature.is_changed() && !exposure.is_changed() {
        return;
    }

    let chromaticity = kelvin_to_srgb(temperature.0).expect("slider range must be supported");
    let perceived =
        black_body_visible_srgb(temperature.0, exposure.0).expect("slider range must be supported");

    for (mut background, kind) in &mut swatches {
        background.0 = match kind {
            SwatchKind::Chromaticity => chromaticity.into(),
            SwatchKind::Perceived => perceived.into(),
        };
    }

    for (mut text, kind) in &mut readouts {
        **text = match kind {
            ReadoutKind::Kelvin => format!("{:.0} K", temperature.0),
            ReadoutKind::Exposure => {
                format!("{:+.1} EV · {:.3}×", exposure.0, 2.0_f64.powf(exposure.0))
            }
            ReadoutKind::Chromaticity => format_srgb(chromaticity),
            ReadoutKind::Perceived => format_srgb(perceived),
            ReadoutKind::Status => approximation_status(temperature.0).to_string(),
        };
    }
}

fn format_srgb(color: Srgba) -> String {
    format!(
        "sRGB({:.3}, {:.3}, {:.3})",
        color.red, color.green, color.blue
    )
}

fn approximation_status(kelvin: f64) -> &'static str {
    if (MIN_ACCURATE_COLOR_TEMPERATURE_K..=MAX_ACCURATE_COLOR_TEMPERATURE_K).contains(&kelvin) {
        "Chromaticity is within its validated range; the right side is a photopic relative-exposure preview."
    } else {
        "Chromaticity is extrapolated here; the right side is a photopic relative-exposure preview."
    }
}
