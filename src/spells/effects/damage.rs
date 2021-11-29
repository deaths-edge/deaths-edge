use bevy::prelude::*;

use crate::character::{CharacterHealth, CharacterIndex};

use super::{EffectMarker, EffectTarget};

pub struct DamageEffect {
    pub amount: u32,
}

pub fn damage_effect_apply(
    damage_query: Query<(Entity, &DamageEffect, &EffectTarget), With<EffectMarker>>,
    mut char_query: Query<(&CharacterIndex, &mut CharacterHealth)>,
    mut commands: Commands,
) {
    for (effect_entity, effect_damage, effect_target) in damage_query.iter() {
        commands.entity(effect_entity).remove::<DamageEffect>();

        if let Some((_, mut character_health)) = char_query
            .iter_mut()
            .find(|(index, _)| effect_target == *index)
        {
            tracing::info!(message = "applied damage", amount = %effect_damage.amount);
            character_health.apply_damage(effect_damage.amount);
        }
    }
}
