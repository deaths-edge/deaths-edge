use bevy::prelude::*;

use crate::character::{CharacterHealth, CharacterIndex};

use super::{EffectMarker, EffectTarget};

pub struct HealEffect {
    pub amount: u32,
}

pub fn health_effect_apply(
    health_query: Query<(Entity, &HealEffect, &EffectTarget), With<EffectMarker>>,
    mut char_query: Query<(&CharacterIndex, &mut CharacterHealth)>,
    mut commands: Commands,
) {
    for (effect_entity, effect_damage, effect_target) in health_query.iter() {
        commands.entity(effect_entity).remove::<HealEffect>();

        if let Some((_, mut character_health)) = char_query
            .iter_mut()
            .find(|(index, _)| effect_target == *index)
        {
            tracing::info!(message = "applied healing", amount = %effect_damage.amount);
            character_health.apply_damage(effect_damage.amount);
        }
    }
}
