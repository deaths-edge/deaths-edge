use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        obstructions::{RequiresStationary, UseObstructions},
        AbilityId, Source, Target,
    },
    character::{CastState, CharacterEntityAction, CharacterMarker, Motion},
    dyn_command::DynCommand,
};

use super::InstantBundle;

#[derive(Debug, Default, Clone, Component)]
pub struct CastMarker;

#[derive(Debug, Clone, Component)]
pub struct CastDuration(pub Duration);

#[derive(Component)]
pub struct CastBundle(pub DynCommand);

#[derive(Debug, Default, Component)]
pub struct Complete;

pub fn spawn_complete_cast(
    cast_query: Query<
        (Entity, &Source, Option<&Target>, Option<&InstantBundle>),
        (With<CastMarker>, With<Complete>),
    >,

    mut commands: Commands,
) {
    for (cast_id, source, opt_target, instant_bundle) in cast_query.iter() {
        commands.entity(cast_id).despawn();

        if let Some(instant_bundle) = instant_bundle {
            let mut entity_commands = commands.spawn();
            instant_bundle.0.apply(&mut entity_commands);

            // Snapshot target from cast
            if let Some(target) = opt_target {
                entity_commands.insert(target.clone());
            }

            // Snapshot source
            entity_commands.insert(source.clone());
        }
    }
}

#[derive(Component)]
pub struct Failed;

pub fn despawn_cast(
    cast_query: Query<(Entity, &Source), (With<CastMarker>, Or<(With<Failed>, With<Complete>)>)>,
    mut character_cast: Query<&mut CastState, With<CharacterMarker>>,

    mut commands: Commands,
) {
    for (cast_id, source) in cast_query.iter() {
        commands.entity(cast_id).despawn();

        let mut cast_state = character_cast
            .get_mut(source.0)
            .expect("failed to find character");

        cast_state.0 = None;
    }
}

/// Checks if cast has completed.
pub fn cast_complete(
    time: Res<Time>,

    cast_query: Query<
        (&AbilityId, &CastDuration, &Source),
        (With<CastMarker>, Without<Failed>, Without<Complete>),
    >,
    mut character_query: Query<&CastState, With<CharacterMarker>>,
    ability_query: Query<&UseObstructions>,

    mut commands: Commands,
) {
    let now = time.last_update().expect("cannot find last update");

    for (ability_id, duration, source) in cast_query.iter() {
        let cast_state = character_query
            .get_mut(source.0)
            .expect("failed to find character");
        let cast = cast_state.0.as_ref().expect("found cast but no cast state");
        let end = cast.start + duration.0;

        if end < now {
            let obstructions = ability_query
                .get(ability_id.0)
                .expect("failed to find ability");
            let obstructed = !obstructions.0.is_empty();

            if !obstructed {
                commands.entity(cast.cast_id).insert(Complete);
            }
        }
    }
}

pub fn cast_movement_interrupt(
    mut motion_events: EventReader<CharacterEntityAction<Motion>>,
    cast_query: Query<Entity, (With<CastMarker>, With<RequiresStationary>)>,

    mut commands: Commands,
) {
    for event in motion_events.iter() {
        if !event.action.is_stationary() {
            for cast_id in cast_query.iter() {
                commands.entity(cast_id).insert(Failed);
            }
        }
    }
}
