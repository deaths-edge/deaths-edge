use std::time::Duration;

use bevy::prelude::*;

use crate::{
    abilities::{Source, Target},
    dyn_command::DynEntityMutate,
};

use super::{Complete, ProgressDuration, TotalDuration};

#[derive(Debug, Clone, Component)]
pub struct StatusMarker;

#[derive(Debug, Component)]
pub struct StatusDuration(pub Duration);

#[derive(Debug, Component)]
pub struct Dispelled;

#[derive(Debug, Component)]
pub struct FinalEffects(pub DynEntityMutate);

#[derive(Debug, Component)]
pub struct DispelEffects(pub DynEntityMutate);

/// Once total progress has been made insert [`Complete`].
pub fn status_complete(
    query: Query<(Entity, &ProgressDuration, &TotalDuration), With<StatusMarker>>,
    mut commands: Commands,
) {
    for (status_id, progress, total) in query.iter() {
        if progress.0 > total.0 {
            commands.entity(status_id).insert(Complete);
        }
    }
}

pub fn status_cleanup(
    query: Query<Entity, (With<StatusMarker>, Or<(With<Complete>, With<Dispelled>)>)>,
    mut commands: Commands,
) {
    for status_id in query.iter() {
        commands.entity(status_id).despawn();
    }
}

pub fn status_final_spawn(
    query: Query<(&FinalEffects, &Source, &Target), (With<StatusMarker>, With<Complete>)>,
    mut commands: Commands,
) {
    for (FinalEffects(mutation), source, target) in query.iter() {
        let mut entity_commands = commands.spawn();
        mutation.apply(&mut entity_commands);

        entity_commands.insert(*source).insert(*target);
    }
}

pub fn status_dispel_spawn(
    query: Query<(&DispelEffects, &Source, &Target), (With<StatusMarker>, With<Dispelled>)>,
    mut commands: Commands,
) {
    for (DispelEffects(mutation), source, target) in query.iter() {
        let mut entity_commands = commands.spawn();
        mutation.apply(&mut entity_commands);

        entity_commands.insert(*source).insert(*target);
    }
}
