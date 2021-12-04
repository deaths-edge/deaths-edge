use bevy::prelude::*;

use crate::character::{CharacterHealth, CharacterIndex};

use super::{EffectMarker, EffectTarget};

pub struct DamageEffect {
    pub amount: u32,
}

pub fn damage_effect_apply(
    damage_query: Query<(Entity, &DamageEffect, &EffectTarget), With<EffectMarker>>,
    mut char_query: Query<&mut CharacterHealth>,
    mut commands: Commands,
) {
    for (effect_entity, effect_damage, effect_target) in damage_query.iter() {
        commands.entity(effect_entity).remove::<DamageEffect>();

        if let Ok(mut character_health) = char_query.get_mut(effect_target.id()) {
            tracing::info!(message = "applied damage", amount = %effect_damage.amount);
            character_health.apply_damage(effect_damage.amount);
        }
    }
}
