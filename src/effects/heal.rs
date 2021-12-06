use bevy::prelude::*;

use crate::character::CharacterHealth;

use super::{EffectMarker, EffectTarget};

#[derive(Debug, Component)]
pub struct HealEffect {
    pub amount: u32,
}

pub fn health_effect_apply(
    health_query: Query<(Entity, &HealEffect, &EffectTarget), With<EffectMarker>>,
    mut character_query: Query<&mut CharacterHealth>,
    mut commands: Commands,
) {
    for (effect_entity, effect_damage, effect_target) in health_query.iter() {
        commands.entity(effect_entity).remove::<HealEffect>();

        if let Ok(mut character_health) = character_query.get_mut(effect_target.id()) {
            tracing::info!(message = "applied healing", amount = %effect_damage.amount);
            character_health.apply_heal(effect_damage.amount);
        }
    }
}
