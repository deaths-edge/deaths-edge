use std::time::Duration;

use bevy::prelude::*;

use super::CharacterEffect;
use crate::{
    abilities::{
        lifecycle::{CastMarker, Failed},
        magic_school::*,
    },
    character::{CastState, Interrupted},
};

#[derive(Default, Debug, Clone, Component)]
pub struct Interrupt(pub Duration);

type SchoolClassify = (With<Fire>, With<Frost>, With<Nature>);
type InterruptableFilter = (With<CastMarker>, With<Interruptable>);

impl CharacterEffect for Interrupt {
    type Domain<'a> = (Entity, &'a mut CastState);

    type Param<'w, 's> = Query<'w, 's, SchoolClassify, InterruptableFilter>;
    type Fetch = QueryState<SchoolClassify, InterruptableFilter>;

    fn apply(
        &self,
        time: &Time,
        (character_id, mut cast_state): (Entity, Mut<'_, CastState>),
        schools: &Query<SchoolClassify, InterruptableFilter>,
        commands: &mut Commands,
    ) {
        // If casting then grab cast_id
        let cast_id = if let Some(cast) = cast_state.0.as_ref() {
            cast.cast_id
        } else {
            return;
        };

        let until = time.last_update().expect("failed to find last update");

        if let Ok((is_fire, is_frost, is_nature)) = schools.get(cast_id) {
            let mut entity_commands = commands.entity(character_id);

            if is_fire {
                entity_commands.insert(Interrupted::<Fire>::new(until));
            }

            if is_frost {
                entity_commands.insert(Interrupted::<Frost>::new(until));
            }

            if is_nature {
                entity_commands.insert(Interrupted::<Nature>::new(until));
            }

            commands.entity(cast_id).insert(Failed);
            cast_state.0 = None;
        }
    }
}
