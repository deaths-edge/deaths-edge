use std::time::Duration;

use bevy::prelude::*;

use crate::character::{CharacterCastState, CharacterIndex, InteruptState};

use super::{EffectMarker, EffectTarget};

#[derive(Default)]
pub struct InteruptEffect {
    lock_duration: Duration,
}

pub fn interupt_effect_apply(
    time: Res<Time>,
    interupt_query: Query<(Entity, &InteruptEffect, &EffectTarget), With<EffectMarker>>,
    mut char_query: Query<(&CharacterIndex, &mut CharacterCastState, &mut InteruptState)>,
    mut commands: Commands,
) {
    for (effect_entity, interupt_effect, effect_target) in interupt_query.iter() {
        commands.entity(effect_entity).remove::<InteruptEffect>();

        let last_update = time.last_update().expect("last update not found");

        if let Some((_, mut casting_state, mut interupt_state)) = char_query
            .iter_mut()
            .find(|(index, _, _)| effect_target == *index)
        {
            let character_cast = casting_state.stop_cast();
            if let Some(cast) = character_cast {
                interupt_state.interupt_until(last_update + interupt_effect.lock_duration);
                tracing::info!(message = "interupted", duration = ?interupt_effect.lock_duration, ?cast);
            }
        }
    }
}
