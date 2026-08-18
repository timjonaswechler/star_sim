//! Generische, klickbasierte Tabs für Bevy UI.
//!
//! Dieses Modul implementiert keine Tastatur-Navigation. Es ist nicht mit Bevys
//! [`TabGroup`](bevy::input_focus::tab_navigation::TabGroup), [`TabIndex`](bevy::input_focus::tab_navigation::TabIndex)
//! oder der `Tab`-Taste verbunden. Die Tabs sind eine UI-Komposition aus
//! `ui_widgets::Button`, einem aktiven Wert und mehreren Inhalts-Panels.
//!
//! # Konzept
//!
//! Eine Tab-Gruppe besteht aus drei Teilen:
//!
//! - [`TabsRoot<K>`] speichert den aktiven Wert für genau einen Container.
//! - [`TabTrigger<K>`] markiert einen klickbaren Button und trägt seinen Wert.
//! - [`TabPanel<K>`] markiert den Inhalt, der zu diesem Wert gehört.
//!
//! Beim Klicken eines Triggers passiert Folgendes:
//!
//! 1. `ui_widgets::Button` erzeugt ein [`Activate`](bevy::ui_widgets::Activate)-Event.
//! 2. [`TabsPlugin<K>`] sucht den nächsten [`TabsRoot<K>`]-Vorfahren des Buttons.
//! 3. Der Root speichert den Wert des geklickten Triggers in `active`.
//! 4. Die Synchronisation blendet das passende Panel ein und alle anderen mit
//!    `Display::None` aus.
//! 5. Der aktive Trigger erhält [`TabActive`], damit ein eigenes Theme ihn
//!    hervorheben kann.
//!
//! Der Root liegt absichtlich als Component am Container und nicht als globale
//! Resource vor. Dadurch können mehrere unabhängige Tab-Gruppen dieselben Werte
//! verwenden, ohne sich gegenseitig zu beeinflussen.
//!
//! # Verwendung mit einem Enum
//!
//! Für statische Bereiche wie Settings ist ein Enum der empfehlenswerte Schlüsseltyp:
//!
//! ```rust,ignore
//! #[derive(Clone, Copy, Default, PartialEq, Eq)]
//! enum SettingsSection {
//!     #[default]
//!     General,
//!     View,
//!     Controls,
//! }
//!
//! impl TabKey for SettingsSection {}
//!
//! app.add_plugins(TabsPlugin::<SettingsSection>::default());
//! ```
//!
//! Die Szenen verwenden anschließend denselben Typ für Root, Trigger und Panel:
//!
//! ```rust,ignore
//! bsn! {
//!     TabsRoot::<SettingsSection> {
//!         active: {SettingsSection::General}
//!     }
//!
//!     Node {
//!         flex_direction: FlexDirection::Column,
//!         row_gap: px(12),
//!     }
//!
//!     Children [
//!         (
//!             Node {
//!                 flex_direction: FlexDirection::Row,
//!             }
//!             Children [
//!                 (
//!                     my_button("Allgemein")
//!                     TabTrigger::<SettingsSection> {
//!                         value: {SettingsSection::General}
//!                     }
//!                 ),
//!                 (
//!                     my_button("Ansicht")
//!                     TabTrigger::<SettingsSection> {
//!                         value: {SettingsSection::View}
//!                     }
//!                 ),
//!             ]
//!         ),
//!         (
//!             TabPanel::<SettingsSection> {
//!                 value: {SettingsSection::General}
//!             }
//!             Node { display: Display::Flex }
//!             Children [Text("Allgemeine Einstellungen")]
//!         ),
//!         (
//!             TabPanel::<SettingsSection> {
//!                 value: {SettingsSection::View}
//!             }
//!             Node { display: Display::None }
//!             Children [Text("Ansichtseinstellungen")]
//!         ),
//!     ]
//! }
//! ```
//!
//! `my_button` ist dabei ein normales, thematisiertes `ui_widgets::Button`. Dieses
//! Modul kümmert sich nur um die Zustandslogik und nicht um Layout oder Farben.
//!
//! # Dynamische Werte
//!
//! Wenn Tabs zur Laufzeit erzeugt werden, kann statt eines Enums ein eigener
//! kopierbarer Schlüsseltyp verwendet werden:
//!
//! ```rust,ignore
//! #[derive(Clone, Copy, Default, PartialEq, Eq)]
//! struct InventorySection(u32);
//!
//! impl TabKey for InventorySection {}
//! ```
//!
//! Jeder konkrete Schlüsseltyp benötigt eine eigene Registrierung von
//! [`TabsPlugin`], weil Bevy `TabsRoot<SettingsSection>` und
//! `TabsRoot<InventorySection>` als unterschiedliche Component-Typen behandelt.
//!

use std::marker::PhantomData;

use bevy::{prelude::*, ui_widgets::Activate};

/// A value that identifies one tab inside a [`TabsRoot`].
///
/// The UI module does not need to know the concrete tab type. An application can
/// use an enum for compile-time checked tab values, or another small, copyable
/// value when tabs are created dynamically.
///
/// [`Default`] is required so the generic components can be used directly in
/// BSN scenes. The trait intentionally has no label method: labels, icons and
/// localization belong to the scene, not to the tab identity.
pub trait TabKey: Copy + Eq + Default + Send + Sync + 'static {}

/// The root of one independent tab group.
///
/// Place this component on the container that owns the triggers and panels. A
/// trigger or panel may be nested below the root; the nearest ancestor root
/// determines which group it belongs to.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TabsRoot<K: TabKey> {
    /// The value whose panel is currently displayed.
    pub active: K,
}

/// A clickable tab trigger.
///
/// Add this component to the same entity as a `ui_widgets::Button`. The button
/// is responsible for producing [`Activate`](bevy::ui_widgets::Activate);
/// [`TabsPlugin`] handles the resulting selection change.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TabTrigger<K: TabKey> {
    /// The panel value selected when this trigger is activated.
    pub value: K,
}

/// The content belonging to one tab trigger.
///
/// The synchronization system sets the entity's [`Node::display`] to
/// `Display::Flex` when its value is active and to `Display::None` otherwise.
/// Consequently, inactive panels do not participate in layout.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TabPanel<K: TabKey> {
    /// The trigger value that makes this panel visible.
    pub value: K,
}

/// Applied to the currently active trigger for styling.
///
/// This marker is intentionally separate from Bevy's focus/navigation
/// components. A theme can query it and choose the active-trigger colors.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TabActive;

/// Registers the behavior for one concrete tab key type.
///
/// Bevy treats `TabsRoot<SettingsTab>` and `TabsRoot<InventoryTab>` as different
/// component types. Therefore each concrete key type is registered separately.
///
/// The plugin registers an [`Activate`] observer and the panel/trigger
/// synchronization system. It does not register `TabNavigationPlugin` and does
/// not make the tabs keyboard-navigable by itself.
#[derive(Default)]
pub struct TabsPlugin<K: TabKey>(PhantomData<K>);

impl<K: TabKey> Plugin for TabsPlugin<K> {
    fn build(&self, app: &mut App) {
        app.add_observer(activate_tab::<K>)
            .add_systems(Update, sync_tabs::<K>);
    }
}

fn activate_tab<K: TabKey>(
    event: On<Activate>,
    triggers: Query<&TabTrigger<K>>,
    parents: Query<&ChildOf>,
    mut roots: Query<&mut TabsRoot<K>>,
) {
    let Ok(trigger) = triggers.get(event.entity) else {
        return;
    };

    let Some(root) = parents
        .iter_ancestors(event.entity)
        .find(|entity| roots.contains(*entity))
    else {
        return;
    };

    roots.get_mut(root).unwrap().active = trigger.value;
}

fn sync_tabs<K: TabKey>(
    roots: Query<&TabsRoot<K>>,
    parents: Query<&ChildOf>,
    triggers: Query<(Entity, &TabTrigger<K>, Has<TabActive>)>,
    mut panels: Query<(Entity, &TabPanel<K>, &mut Node)>,
    mut commands: Commands,
) {
    for (entity, trigger, active) in triggers.iter() {
        let Some(selected_value) = parents
            .iter_ancestors(entity)
            .find_map(|ancestor| roots.get(ancestor).ok().map(|root| root.active))
        else {
            continue;
        };

        let should_be_active = selected_value == trigger.value;

        if should_be_active != active {
            if should_be_active {
                commands.entity(entity).insert(TabActive);
            } else {
                commands.entity(entity).remove::<TabActive>();
            }
        }
    }

    for (entity, panel, mut node) in panels.iter_mut() {
        let Some(selected_value) = parents
            .iter_ancestors(entity)
            .find_map(|ancestor| roots.get(ancestor).ok().map(|root| root.active))
        else {
            continue;
        };

        node.display = if selected_value == panel.value {
            Display::Flex
        } else {
            Display::None
        };
    }
}
