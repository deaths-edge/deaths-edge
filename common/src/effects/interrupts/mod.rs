mod motion;

pub use motion::*;

use std::time::Duration;

use bevy::prelude::*;

use crate::character::{CharacterCastState, InterruptState};

use super::{EffectMarker, EffectTarget};

#[derive(Default)]
pub struct InterruptEffect {
    lock_duration: Duration,
}

pub fn interrupt_effect_apply(
    time: Res<Time>,
    interrupt_query: Query<(Entity, &InterruptEffect, &EffectTarget), With<EffectMarker>>,
    mut character_query: Query<(&mut CharacterCastState, &mut InterruptState)>,
    mut commands: Commands,
) {
    for (effect_entity, interrupt_effect, effect_target) in interrupt_query.iter() {
        commands.entity(effect_entity).remove::<InterruptEffect>();

        let last_update = time.last_update().expect("last update not found");

        if let Ok((mut casting_state, mut interrupt_state)) =
            character_query.get_mut(effect_target.0)
        {
            let character_cast = casting_state.stop_cast();
            if let Some(cast) = character_cast {
                interrupt_state.interrupt_until(last_update + interrupt_effect.lock_duration);
                tracing::info!(message = "interrupted", duration = ?interrupt_effect.lock_duration, ?cast);
            }
        }
    }
}
