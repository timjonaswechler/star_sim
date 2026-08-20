use bevy::{
    color::palettes::basic,
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};
use std::fmt::Debug;

#[cfg(feature = "automation")]
use automation_control::{AutomationControlPlugin, AutomationTarget};

/// Event opening a new context menu at a pointer position.
#[derive(Event)]
struct OpenContextMenu {
    pos: Vec2,
}

/// Event closing all currently open context menus.
#[derive(Event)]
struct CloseContextMenus;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct ContextMenu;

#[derive(Component)]
struct ContextMenuItem {
    name: &'static str,
    color: Srgba,
}

/// Small reflected application state used by the observation smoke test.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct SessionState {
    menu_open: bool,
    selected_item: String,
}

fn main() {
    let mut binding = App::new();

    #[cfg(feature = "automation")]
    let app = binding
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<bevy::input::InputPlugin>()
                .disable::<bevy::gilrs::GilrsPlugin>()
                .disable::<bevy::input_focus::InputDispatchPlugin>(),
        )
        .add_plugins(AutomationControlPlugin::stdio());
    #[cfg(not(feature = "automation"))]
    let app = binding.add_plugins(DefaultPlugins);

    app.register_type::<SessionState>()
        .add_systems(Startup, setup)
        .add_observer(on_trigger_menu)
        .add_observer(on_trigger_close_menus)
        .add_observer(text_color_on_hover::<Out>(basic::WHITE.into()))
        .add_observer(text_color_on_hover::<Over>(basic::RED.into()))
        .run();
}

fn text_color_on_hover<T: Debug + Clone + Reflect>(
    color: Color,
) -> impl FnMut(On<Pointer<T>>, Query<&mut TextColor>, Query<&Children>) {
    move |mut event: On<Pointer<T>>,
          mut text_color: Query<&mut TextColor>,
          children: Query<&Children>| {
        let Ok(children) = children.get(event.original_event_target()) else {
            return;
        };
        event.propagate(false);
        for child in children.iter() {
            if let Ok(mut col) = text_color.get_mut(child) {
                col.0 = color;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Name::new("background"),
            Background,
            SessionState {
                menu_open: false,
                selected_item: "none".into(),
            },
            #[cfg(feature = "automation")]
            AutomationTarget,
            background_and_button(),
        ))
        .observe(|_: On<Pointer<Press>>, mut commands: Commands| {
            commands.trigger(CloseContextMenus);
        });
}

fn on_trigger_close_menus(
    _event: On<CloseContextMenus>,
    mut commands: Commands,
    menus: Query<Entity, With<ContextMenu>>,
    mut state: Query<&mut SessionState, With<Background>>,
) {
    for entity in menus.iter() {
        commands.entity(entity).despawn();
    }
    if let Ok(mut state) = state.single_mut() {
        state.menu_open = false;
    }
}

fn on_trigger_menu(
    event: On<OpenContextMenu>,
    mut commands: Commands,
    mut state: Query<&mut SessionState, With<Background>>,
) {
    commands.trigger(CloseContextMenus);
    if let Ok(mut state) = state.single_mut() {
        state.menu_open = true;
    }

    let pos = event.pos;
    commands
        .spawn((
            Name::new("context menu"),
            ContextMenu,
            Node {
                position_type: PositionType::Absolute,
                left: px(pos.x),
                top: px(pos.y),
                flex_direction: FlexDirection::Column,
                border_radius: BorderRadius::all(px(4)),
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BackgroundColor(Color::linear_rgb(0.1, 0.1, 0.1)),
            children![
                context_item("fuchsia", basic::FUCHSIA),
                context_item("gray", basic::GRAY),
                context_item("maroon", basic::MAROON),
                context_item("purple", basic::PURPLE),
                context_item("teal", basic::TEAL),
            ],
        ))
        .observe(
            |event: On<Pointer<Press>>,
             menu_items: Query<&ContextMenuItem>,
             mut clear_col: ResMut<ClearColor>,
             mut state: Query<&mut SessionState, With<Background>>,
             mut commands: Commands| {
                let target = event.original_event_target();
                if let Ok(item) = menu_items.get(target) {
                    clear_col.0 = item.color.into();
                    if let Ok(mut state) = state.single_mut() {
                        state.selected_item = item.name.into();
                        state.menu_open = false;
                    }
                    commands.trigger(CloseContextMenus);
                }
            },
        );
}

fn context_item(text: &'static str, color: Srgba) -> impl Bundle {
    (
        Name::new(format!("item-{text}")),
        ContextMenuItem { name: text, color },
        #[cfg(feature = "automation")]
        AutomationTarget,
        Button,
        Node {
            padding: UiRect::all(px(5)),
            ..default()
        },
        children![(
            Pickable::IGNORE,
            Text::new(text),
            TextFont {
                font_size: FontSize::Px(24.0),
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    )
}

fn background_and_button() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(-10),
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            parent
                .spawn((
                    Name::new("button"),
                    #[cfg(feature = "automation")]
                    AutomationTarget,
                    Button,
                    Node {
                        width: px(250),
                        height: px(65),
                        border: UiRect::all(px(5)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BorderColor::all(Color::BLACK),
                    BackgroundColor(Color::BLACK),
                    children![(
                        Pickable::IGNORE,
                        Text::new("Context Menu"),
                        TextFont {
                            font_size: FontSize::Px(28.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextShadow::default(),
                    )],
                ))
                .observe(|mut event: On<Pointer<Press>>, mut commands: Commands| {
                    event.propagate(false);
                    debug!("click: {}", event.pointer_location.position);
                    commands.trigger(OpenContextMenu {
                        pos: event.pointer_location.position,
                    });
                });
        })),
    )
}
