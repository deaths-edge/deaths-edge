use bevy::{core::Time, prelude::*, utils::Instant};

use crate::spells::SpellCast;

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

    pub fn stop_cast(&mut self) -> Option<CharacterCast> {
        self.cast.take()
    }

    pub fn cast(&self) -> Option<&CharacterCast> {
        self.cast.as_ref()
    }
}

#[derive(Debug)]
pub struct CharacterCast {
    pub start: Instant,
    pub spell: SpellCast,
}

impl CharacterCast {
    pub fn new(start: Instant, spell: SpellCast) -> Self {
        Self { start, spell }
    }

    pub fn is_complete(&self, now: Instant) -> bool {
        self.start + self.spell.duration() <= now
    }
}

fn complete_casting(
    mut query: Query<(
        Entity,
        &Transform,
        &mut CharacterCastState,
        &CharacterTarget,
    )>,
    mut commands: Commands,

    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let last_update = time.last_update().expect("last update not found");
    for (character_entity, transform, mut cast_state, target) in
        query.iter_mut().filter(|(_, _, cast_state, _)| {
            cast_state
                .cast()
                .map(|cast| cast.is_complete(last_update))
                .unwrap_or_default()
        })
    {
        tracing::info!(message = "completed cast", ?cast_state);
        let cast = cast_state.stop_cast().expect("checked valid");

        cast.spell
            .spawn_bundle(character_entity, transform, &mut commands, &mut materials)
    }
}
