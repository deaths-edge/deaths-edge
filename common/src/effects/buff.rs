use bevy::prelude::*;

use crate::character::{Buff, CharacterBuffs, CharacterMarker};

use super::{EffectMarker, EffectTarget};

/// Applies a buff to the target.
pub struct BuffEffect {
    pub buff: Buff,
}

pub fn buff_effect_apply(
    buff_query: Query<(Entity, &BuffEffect, &EffectTarget), With<EffectMarker>>,
    mut character_query: Query<&mut CharacterBuffs, With<CharacterMarker>>,
    mut commands: Commands,
) {
    for (effect_entity, buff_effect, effect_target) in buff_query.iter() {
        commands.entity(effect_entity).remove::<BuffEffect>();

        if let Ok(mut buffs) = character_query.get_mut(effect_target.0) {
            tracing::info!(message = "applied buff", buff = ?buff_effect.buff);
            buffs.0.push(buff_effect.buff.clone());
        }
    }
}
