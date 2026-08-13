use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct AgentTarget {
    pub id: String,
    pub role: String,
    pub label: String,
    pub actions: Vec<String>,
}

impl AgentTarget {
    pub fn new(
        id: impl Into<String>,
        role: impl Into<String>,
        label: impl Into<String>,
        actions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            id: id.into(),
            role: role.into(),
            label: label.into(),
            actions: actions.into_iter().map(Into::into).collect(),
        }
    }

    pub fn supports(&self, action: &str) -> bool {
        self.actions.iter().any(|candidate| candidate == action)
    }
}

#[derive(Clone, Debug, Default, Resource)]
pub struct TargetRegistry {
    by_id: HashMap<String, Entity>,
    by_entity: HashMap<Entity, String>,
    duplicates: HashMap<String, Vec<Entity>>,
}

impl TargetRegistry {
    pub fn entity(&self, id: &str) -> Result<Entity, RegistryLookupError> {
        if self.duplicates.contains_key(id) {
            return Err(RegistryLookupError::Duplicate(id.to_owned()));
        }
        self.by_id
            .get(id)
            .copied()
            .ok_or_else(|| RegistryLookupError::Unknown(id.to_owned()))
    }

    pub fn contains(&self, id: &str) -> bool {
        self.by_id.contains_key(id) && !self.duplicates.contains_key(id)
    }

    pub fn duplicate_ids(&self) -> impl Iterator<Item = &str> {
        self.duplicates.keys().map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.by_entity.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_entity.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistryLookupError {
    Unknown(String),
    Duplicate(String),
}

pub(crate) fn sync_registry(
    mut registry: ResMut<TargetRegistry>,
    targets: Query<(Entity, &AgentTarget)>,
) {
    registry.by_id.clear();
    registry.by_entity.clear();
    registry.duplicates.clear();
    for (entity, target) in &targets {
        registry.by_entity.insert(entity, target.id.clone());
        if let Some(first) = registry.by_id.insert(target.id.clone(), entity) {
            registry
                .duplicates
                .entry(target.id.clone())
                .or_insert_with(|| vec![first])
                .push(entity);
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TargetObservation {
    pub id: String,
    pub role: String,
    pub label: String,
    pub visible: bool,
    pub enabled: bool,
    pub actions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<Bounds>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Default, Resource, Serialize, Deserialize, PartialEq)]
pub struct Observations {
    pub ui: Vec<TargetObservation>,
    pub scene: Vec<TargetObservation>,
    pub selection: Vec<TargetObservation>,
    pub camera: Vec<TargetObservation>,
}

pub(crate) fn observe_targets(
    targets: Query<(
        &AgentTarget,
        Option<&ComputedNode>,
        Option<&InheritedVisibility>,
        Option<&bevy::ui::InteractionDisabled>,
    )>,
    mut observations: ResMut<Observations>,
) {
    *observations = Observations::default();
    for (target, node, visibility, disabled) in &targets {
        let bounds = node.map(|node| Bounds {
            width: node.size().x,
            height: node.size().y,
        });
        let visible = visibility.is_none_or(|visibility| visibility.get())
            && bounds.is_none_or(|bounds| bounds.width > 0.0 && bounds.height > 0.0);
        let observation = TargetObservation {
            id: target.id.clone(),
            role: target.role.clone(),
            label: target.label.clone(),
            visible,
            enabled: disabled.is_none(),
            actions: target.actions.clone(),
            bounds,
        };
        match target.role.as_str() {
            "button" | "ui" => observations.ui.push(observation),
            "camera" => observations.camera.push(observation),
            "selection" => observations.selection.push(observation),
            _ => observations.scene.push(observation),
        }
    }
    observations
        .ui
        .sort_by(|left, right| left.id.cmp(&right.id));
    observations
        .scene
        .sort_by(|left, right| left.id.cmp(&right.id));
    observations
        .selection
        .sort_by(|left, right| left.id.cmp(&right.id));
    observations
        .camera
        .sort_by(|left, right| left.id.cmp(&right.id));
}
