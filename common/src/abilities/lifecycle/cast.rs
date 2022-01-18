use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{Source, Target},
    character::{CastState, CharacterMarker},
    dyn_command::DynCommand,
};

use super::InstantBundle;

#[derive(Debug, Default, Clone, Component)]
pub struct CastMarker;

#[derive(Debug, Clone, Component)]
pub struct CastDuration(pub Duration);

#[derive(Component)]
pub struct CastBundle(pub DynCommand);

pub fn cast_complete(
    time: Res<Time>,

    cast_query: Query<(&CastDuration, Option<&Target>, &InstantBundle, &Source), With<CastMarker>>,
    mut character_query: Query<&mut CastState, With<CharacterMarker>>,

    mut commands: Commands,
) {
    let now = time.last_update().expect("cannot find last update");

    for (duration, opt_target, instant_bundle, source) in cast_query.iter() {
        let mut cast_state = character_query
            .get_mut(source.0)
            .expect("failed to find character");
        let cast = cast_state.0.as_ref().expect("found cast but no cast state");
        let end = cast.start + duration.0;

        if end < now {
            let cast = cast_state.0.take().expect("found cast but no cast state");

            // Spawn instant bundle
            let mut entity_commands = commands.spawn();
            instant_bundle.0.apply(&mut entity_commands);

            // Snapshot target from cast
            if let Some(target) = opt_target {
                entity_commands.insert(target.clone());
            }

            // Snapshot source
            entity_commands.insert(source.clone());

            // Remove cast
            commands.entity(cast.cast_id).despawn();
        }
    }
}
