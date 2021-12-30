use bevy::prelude::*;

use crate::character::{CharacterControls, CharacterMarker, Control};

use super::{EffectMarker, EffectTarget};

/// Applies control to the target.
#[derive(Debug)]
pub enum ControlEffectType {
    Add(Control),
    Remove(Control),
}

#[derive()]
pub struct ControlEffect {
    marker: EffectMarker,
    control_effect_type: ControlEffectType,
}

impl ControlEffect {
    pub fn add(control: Control) -> Self {
        Self {
            marker: EffectMarker,
            control_effect_type: ControlEffectType::Add(control),
        }
    }

    pub fn remove(control: Control) -> Self {
        Self {
            marker: EffectMarker,
            control_effect_type: ControlEffectType::Remove(control),
        }
    }
}

pub fn control_effect_apply(
    damage_query: Query<(Entity, &ControlEffectType, &EffectTarget), With<EffectMarker>>,
    mut character_query: Query<&mut CharacterControls, With<CharacterMarker>>,
    mut commands: Commands,
) {
    for (effect_entity, control_effect, effect_target) in damage_query.iter() {
        commands.entity(effect_entity).remove::<ControlEffectType>();

        if let Ok(mut character_control) = character_query.get_mut(effect_target.0) {
            match control_effect {
                ControlEffectType::Add(control) => {
                    tracing::info!(message = "add control", control = ?control);
                    character_control.0.push(control.clone());
                }
                ControlEffectType::Remove(control) => {
                    tracing::info!(message = "remove control", control = ?control);
                    if let Some(index) = character_control.0.iter().position(|item| item == control)
                    {
                        character_control.0.remove(index);
                    }
                }
            }
        }
    }
}
