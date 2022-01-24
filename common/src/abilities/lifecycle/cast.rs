use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{
        obstructions::{RequiresStationary, UseObstructions},
        AbilityId, Source, Target,
    },
    character::{Cast, CastState, CharacterEntityAction, CharacterMarker, Motion},
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
        (&AbilityId, &Source, Option<&Target>, Option<&InstantBundle>),
        (With<CastMarker>, With<Complete>),
    >,

    mut commands: Commands,
) {
    for (ability_id, source, opt_target, instant_bundle) in cast_query.iter() {
        if let Some(instant_bundle) = instant_bundle {
            let mut entity_commands = commands.spawn();
            instant_bundle.0.apply(&mut entity_commands);

            // Snapshot ability id
            entity_commands.insert(*ability_id);

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

/// Anchors new casts to cast state.
pub fn anchor_cast(
    time: Res<Time>,
    cast_query: Query<(Entity, &Source), Added<CastMarker>>,
    mut character_query: Query<&mut CastState, With<CharacterMarker>>,
) {
    let now = time.last_update().expect("failed to find last update");

    for (cast_id, Source(source)) in cast_query.iter() {
        let mut cast_state = character_query
            .get_mut(*source)
            .expect("failed to find character");
        let cast = Cast {
            start: now,
            cast_id,
        };

        if cast_state.0.is_some() {
            panic!("cannot cast while casting");
        }

        cast_state.0 = Some(cast);
    }
}

/// Checks if cast has completed.
pub fn cast_complete(
    time: Res<Time>,

    cast_query: Query<
        (&CastDuration, &UseObstructions, &Source),
        (With<CastMarker>, Without<Failed>, Without<Complete>),
    >,
    mut character_query: Query<&CastState, With<CharacterMarker>>,

    mut commands: Commands,
) {
    let now = time.last_update().expect("cannot find last update");

    for (duration, obstructions, source) in cast_query.iter() {
        let cast_state = character_query
            .get_mut(source.0)
            .expect("failed to find character");
        let cast = cast_state.0.as_ref().expect("found cast but no cast state");
        let end = cast.start + duration.0;

        if end < now {
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
