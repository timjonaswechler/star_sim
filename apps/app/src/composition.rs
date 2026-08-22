#[cfg(feature = "automation-control")]
use bevy::{
    asset::AssetPlugin,
    camera::CameraPlugin,
    image::{ImagePlugin, TextureAtlasPlugin},
    mesh::MeshPlugin,
    picking::{InteractionPlugin, PickingPlugin},
    prelude::*,
    scene::ScenePlugin,
    text::TextPlugin,
    transform::TransformPlugin,
    ui::UiPlugin,
    ui_widgets::UiWidgetsPlugins,
    window::{PrimaryWindow, WindowResolution},
};

#[cfg(feature = "automation-control")]
pub(crate) struct Canvas;

#[cfg(feature = "automation-control")]
impl Canvas {
    pub(crate) const WIDTH: u32 = 640;
    pub(crate) const HEIGHT: u32 = 360;
}

#[cfg(feature = "automation-control")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ControlledMode {
    Logical,
    Rendered,
}

#[cfg(feature = "automation-control")]
impl ControlledMode {
    pub(crate) fn parse(
        arguments: impl IntoIterator<Item = std::ffi::OsString>,
    ) -> Result<Self, String> {
        let arguments = arguments.into_iter().collect::<Vec<_>>();
        let [option, value] = arguments.as_slice() else {
            return Err("controlled app expects the Debug Host mode argument".into());
        };
        if option != "--controlled-mode" {
            return Err("controlled app received an unsupported internal argument".into());
        }
        match value.to_str() {
            Some("logical") => Ok(Self::Logical),
            Some("rendered") => Ok(Self::Rendered),
            _ => Err("controlled app mode must be logical or rendered".into()),
        }
    }
}

#[cfg(feature = "automation-control")]
pub(crate) fn controlled(app: &mut App, mode: ControlledMode) -> &mut App {
    match mode {
        ControlledMode::Logical => logical(app),
        ControlledMode::Rendered => rendered(app),
    }
}

#[cfg(feature = "automation-control")]
fn rendered(app: &mut App) -> &mut App {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Star Sim Controlled Session".into(),
                    resolution: WindowResolution::new(Canvas::WIDTH, Canvas::HEIGHT)
                        .with_scale_factor_override(1.0),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build()
            .disable::<bevy::input::InputPlugin>()
            .disable::<bevy::gilrs::GilrsPlugin>(),
    )
    .add_plugins((
        bug_hunter::AutomationControlPlugin::rendered_stdio(),
        bug_hunter::screenshot::Plugin::default(),
    ))
}

#[cfg(feature = "automation-control")]
fn logical(app: &mut App) -> &mut App {
    app.add_plugins(MinimalPlugins).add_plugins((
        AssetPlugin::default(),
        TransformPlugin,
        ImagePlugin::default(),
        TextureAtlasPlugin,
        MeshPlugin,
        CameraPlugin,
        ScenePlugin,
        TextPlugin,
        PickingPlugin,
        InteractionPlugin,
        UiPlugin,
        bug_hunter::AutomationControlPlugin::logical_stdio(),
    ));

    app.world_mut().spawn((
        Name::new("logical-surface"),
        PrimaryWindow,
        Window {
            resolution: WindowResolution::new(Canvas::WIDTH, Canvas::HEIGHT)
                .with_scale_factor_override(1.0),
            resizable: false,
            ..default()
        },
    ));
    app
}

#[cfg(all(test, feature = "automation-control"))]
mod tests {
    use super::*;
    use bevy::{
        input::InputPlugin,
        render::{RenderApp, RenderPlugin},
        window::WindowPlugin,
    };

    #[test]
    fn parses_only_the_debug_hosts_internal_mode_argument() {
        assert_eq!(
            ControlledMode::parse(["--controlled-mode".into(), "logical".into()]).unwrap(),
            ControlledMode::Logical
        );
        assert_eq!(
            ControlledMode::parse(["--controlled-mode".into(), "rendered".into()]).unwrap(),
            ControlledMode::Rendered
        );
        assert!(ControlledMode::parse(Vec::<std::ffi::OsString>::new()).is_err());
        assert!(ControlledMode::parse(["--automation".into()]).is_err());
    }

    #[test]
    fn logical_composition_has_no_native_window_input_or_renderer() {
        let mut app = App::new();
        controlled(&mut app, ControlledMode::Logical);

        assert!(!app.is_plugin_added::<InputPlugin>());
        assert!(!app.is_plugin_added::<bevy::gilrs::GilrsPlugin>());
        assert!(!app.is_plugin_added::<bevy::winit::WinitPlugin>());
        assert!(!app.is_plugin_added::<bevy::picking::input::PointerInputPlugin>());
        assert!(!app.is_plugin_added::<WindowPlugin>());
        assert!(!app.is_plugin_added::<RenderPlugin>());
        assert!(app.get_sub_app(RenderApp).is_none());

        let mut windows = app
            .world_mut()
            .query_filtered::<&Window, With<PrimaryWindow>>();
        let window = windows.single(app.world()).unwrap();
        assert_eq!(window.resolution.physical_width(), Canvas::WIDTH);
        assert_eq!(window.resolution.physical_height(), Canvas::HEIGHT);
        assert_eq!(window.resolution.scale_factor(), 1.0);
        assert!(!window.resizable);
    }

    #[test]
    #[ignore = "requires a native event loop on the test thread"]
    fn rendered_composition_disables_native_input_producers() {
        let mut app = App::new();
        controlled(&mut app, ControlledMode::Rendered);

        assert!(!app.is_plugin_added::<InputPlugin>());
        assert!(!app.is_plugin_added::<bevy::gilrs::GilrsPlugin>());
        assert!(app.is_plugin_added::<WindowPlugin>());
        assert!(app.is_plugin_added::<RenderPlugin>());
        assert!(app.is_plugin_added::<bevy::winit::WinitPlugin>());
    }
}
