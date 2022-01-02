use bevy::prelude::*;

use crate::character::{CharacterMarker, Health};

use super::{EffectMarker, EffectTarget};

/// Applies damage to the target.
pub struct DamageEffect {
    pub amount: f32,
}

pub fn damage_effect_apply(
    damage_query: Query<(Entity, &DamageEffect, &EffectTarget), With<EffectMarker>>,
    mut character_query: Query<&mut Health, With<CharacterMarker>>,
    mut commands: Commands,
) {
    for (effect_entity, effect_damage, effect_target) in damage_query.iter() {
        commands.entity(effect_entity).remove::<DamageEffect>();

        if let Ok(mut character_health) = character_query.get_mut(effect_target.0) {
            tracing::info!(message = "applied damage", amount = %effect_damage.amount);
            character_health.apply_damage(effect_damage.amount);
        }
    }
}
