use bevy::prelude::*;

use super::{EffectMarker, EffectTarget};

/// Applies a buff to the target.
pub struct BuffEffect {
    // pub buff: CharacterControl,
}

pub fn buff_effect_apply(
    buff_query: Query<(Entity, &BuffEffect, &EffectTarget), With<EffectMarker>>,
    // mut character_query: Query<&mut CharacterHealth>,
    mut commands: Commands,
) {
    for (effect_entity, effect_damage, effect_target) in buff_query.iter() {
        commands.entity(effect_entity).remove::<BuffEffect>();

        // if let Ok(mut character_health) = buff_query.get_mut(effect_target.0) {
        // tracing::info!(message = "applied damage", amount = %effect_damage.amount);
        // character_health.apply_damage(effect_damage.amount);
        // }
    }
}
