use bevy::{
    input_focus::tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::Button,
};
use ui::components::tabs::{TabKey, TabPanel, TabTrigger, TabsPlugin, TabsRoot};

#[cfg(feature = "automation-control")]
use automation_control::AutomationTarget;

pub(crate) struct MenuPlugin;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum MenuSection {
    #[default]
    Gym,
    Museum,
    Zoo,
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub(crate) struct MenuTab {
    pub(crate) id: &'static str,
    pub(crate) label: &'static str,
    pub(crate) section: MenuSection,
}

impl TabKey for MenuSection {}

#[cfg(feature = "automation-control")]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct SessionObservation {
    active_screen: String,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TabNavigationPlugin, TabsPlugin::<MenuSection>::default()))
            .add_systems(Startup, setup_scene.spawn());

        #[cfg(feature = "automation-control")]
        app.register_type::<SessionObservation>()
            .add_systems(PostStartup, install_controlled_targets)
            .add_systems(Update, sync_session_observation);
    }
}

#[cfg(feature = "automation-control")]
fn install_controlled_targets(
    mut commands: Commands,
    tabs: Query<(Entity, &MenuTab)>,
    roots: Query<(Entity, &TabsRoot<MenuSection>)>,
) {
    for (entity, tab) in &tabs {
        commands
            .entity(entity)
            .insert((AutomationTarget, Name::new(tab.id)));
    }
    for (entity, root) in &roots {
        commands.entity(entity).insert((
            AutomationTarget,
            Name::new("session.status"),
            SessionObservation {
                active_screen: screen_name(root.active).into(),
            },
        ));
    }
}

#[cfg(feature = "automation-control")]
fn sync_session_observation(mut roots: Query<(&TabsRoot<MenuSection>, &mut SessionObservation)>) {
    for (root, mut observation) in &mut roots {
        observation.active_screen = screen_name(root.active).into();
    }
}

#[cfg(feature = "automation-control")]
fn screen_name(section: MenuSection) -> &'static str {
    match section {
        MenuSection::Gym => "gym",
        MenuSection::Museum => "museum",
        MenuSection::Zoo => "zoo",
    }
}

fn setup_scene() -> impl SceneList {
    bsn_list![Camera2d, menu()]
}

fn menu() -> impl Scene {
    bsn! {
        Node {
            width: layout_dimension(Dimension::Width),
            height: layout_dimension(Dimension::Height),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
        }
        TabGroup
        TabsRoot::<MenuSection> {
            active: {MenuSection::Gym}
        }
        Children [
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(8),
                }
                Children [
                    (
                        tab_button("Gym", "menu.tab.gym", MenuSection::Gym)
                        TabTrigger::<MenuSection> {
                            value: {MenuSection::Gym}
                        }
                    ),
                    (
                        tab_button("Museum", "menu.tab.museum", MenuSection::Museum)
                        TabTrigger::<MenuSection> {
                            value: {MenuSection::Museum}
                        }
                    ),
                    (
                        tab_button("Zoo", "menu.tab.zoo", MenuSection::Zoo)
                        TabTrigger::<MenuSection> {
                            value: {MenuSection::Zoo}
                        }
                    ),
                ]
            ),
            (
                TabPanel::<MenuSection> {
                    value: {MenuSection::Gym}
                }
                Node { display: Display::Flex }
                Children [Text("Gym")]
            ),
            (
                TabPanel::<MenuSection> {
                    value: {MenuSection::Museum}
                }
                Node { display: Display::None }
                Children [Text("Museum")]
            ),
            (
                TabPanel::<MenuSection> {
                    value: {MenuSection::Zoo}
                }
                Node { display: Display::None }
                Children [Text("Zoo")]
            ),
        ]
    }
}

enum Dimension {
    Width,
    Height,
}

fn layout_dimension(dimension: Dimension) -> Val {
    #[cfg(feature = "automation-control")]
    return match dimension {
        Dimension::Width => px(crate::composition::Canvas::WIDTH),
        Dimension::Height => px(crate::composition::Canvas::HEIGHT),
    };

    #[cfg(not(feature = "automation-control"))]
    match dimension {
        Dimension::Width | Dimension::Height => percent(100),
    }
}

fn tab_button(label: &'static str, id: &'static str, section: MenuSection) -> impl Scene {
    bsn! {
        MenuTab { id, label, section }
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            border_radius: BorderRadius::ZERO,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Button
        Hovered::default()
        TabIndex(0)
        BorderColor::all(Color::BLACK)
        BackgroundColor(NORMAL_BUTTON)
        Children [(
            Text(label)
            TextFont {
                font_size: FontSize::Px(33.0),
            }
            TextColor(Color::srgb(0.9, 0.9, 0.9))
            TextShadow::default()
        )]
    }
}
