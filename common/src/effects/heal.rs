use bevy::prelude::*;

use super::{EffectMarker, EffectTarget};
use crate::character::{CharacterMarker, Health};

/// Applies healing to the target.
pub struct HealEffect {
    pub amount: f32,
}

pub fn health_effect_apply(
    health_query: Query<(Entity, &HealEffect, &EffectTarget), With<EffectMarker>>,
    mut character_query: Query<&mut Health, With<CharacterMarker>>,
    mut commands: Commands,
) {
    for (effect_entity, effect_damage, effect_target) in health_query.iter() {
        commands.entity(effect_entity).remove::<HealEffect>();

        if let Ok(mut character_health) = character_query.get_mut(effect_target.0) {
            tracing::info!(message = "applied healing", amount = %effect_damage.amount);
            character_health.apply_heal(effect_damage.amount);
        }
    }
}
