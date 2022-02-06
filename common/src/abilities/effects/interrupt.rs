use std::time::Duration;

use bevy::prelude::*;

use super::CharacterEffect;
use crate::{
    abilities::{
        lifecycle::{CastMarker, Failed},
        magic_school::*,
        AbilityId,
    },
    character::{CastState, Interrupted},
};

#[derive(Default, Debug, Clone, Component)]
pub struct Interrupt(pub Duration);

type SchoolClassify = (
    Option<&'static Fire>,
    Option<&'static Frost>,
    Option<&'static Nature>,
);
type InterruptableFilter = (With<CastMarker>, With<Interruptable>);

impl CharacterEffect for Interrupt {
    type Domain<'a> = (Entity, &'a mut CastState);

    type Param<'w, 's> = Query<'w, 's, SchoolClassify, InterruptableFilter>;
    type Fetch = QueryState<SchoolClassify, InterruptableFilter>;

    fn apply(
        &self,
        _parent_id: Entity,

        (character_id, cast_state): (Entity, Mut<'_, CastState>),
        schools: &Query<SchoolClassify, InterruptableFilter>,

        time: &Time,

        commands: &mut Commands,
    ) {
        // If casting then grab cast_id
        let cast_id = if let Some(cast) = cast_state.0.as_ref() {
            cast.cast_id
        } else {
            return;
        };

        let now = time.last_update().expect("failed to find last update");
        let until = now + self.0;

        if let Ok((is_fire, is_frost, is_nature)) = schools.get(cast_id) {
            let mut entity_commands = commands.entity(character_id);

            if is_fire.is_some() {
                entity_commands.insert(Interrupted::<Fire>::new(now, until));
            }

            if is_frost.is_some() {
                entity_commands.insert(Interrupted::<Frost>::new(now, until));
            }

            if is_nature.is_some() {
                entity_commands.insert(Interrupted::<Nature>::new(now, until));
            }

            commands.entity(cast_id).insert(Failed);
        }
    }
}
