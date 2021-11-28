use bevy::{core::Time, prelude::*, utils::Instant};

use crate::spell::Spell;

use super::CharacterTarget;

pub struct CastingPlugin;

impl Plugin for CastingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(complete_casting.system());
    }
}

#[derive(Default, Debug)]
pub struct CharacterCastState {
    cast: Option<CharacterCast>,
}

impl CharacterCastState {
    pub fn set_cast(&mut self, cast: CharacterCast) -> &mut Self {
        self.cast = Some(cast);
        self
    }

    pub fn stop_cast(&mut self) -> &mut Self {
        self.cast = None;
        self
    }

    pub fn cast(&self) -> Option<&CharacterCast> {
        self.cast.as_ref()
    }
}

#[derive(Debug)]
pub struct CharacterCast {
    pub start: Instant,
    pub target: CharacterTarget,
    pub spell: Spell,
}

impl CharacterCast {
    pub fn new(start: Instant, target: CharacterTarget, spell: Spell) -> Self {
        Self {
            start,
            target,
            spell,
        }
    }

    pub fn is_complete(&self, now: Instant) -> bool {
        self.start + self.spell.duration() <= now
    }
}

fn complete_casting(time: Res<Time>, mut query: Query<&mut CharacterCastState>) {
    let last_update = time.last_update().expect("last update not found");
    for mut cast_state in query.iter_mut().filter(|cast_state| {
        cast_state
            .cast()
            .map(|cast| cast.is_complete(last_update))
            .unwrap_or_default()
    }) {
        tracing::info!(message = "completed cast", ?cast_state);
        cast_state.stop_cast();
    }
}
