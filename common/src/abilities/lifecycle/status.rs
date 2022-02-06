use std::{fmt::Debug, hash::Hash, time::Duration};

use bevy::prelude::*;

use crate::dyn_command::DynEntityMutate;

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
    query: Query<(Entity, &FinalEffects), (With<StatusMarker>, With<Complete>)>,
    mut commands: Commands,
) {
    for (id, FinalEffects(mutation)) in query.iter() {
        let mut entity_commands = commands.spawn();
        mutation.apply(id, &mut entity_commands);
    }
}

pub fn status_dispel_spawn(
    query: Query<(Entity, &DispelEffects), (With<StatusMarker>, With<Dispelled>)>,
    mut commands: Commands,
) {
    for (id, DispelEffects(mutation)) in query.iter() {
        let mut entity_commands = commands.spawn();
        mutation.apply(id, &mut entity_commands);
    }
}

pub struct StatusPlugin<T, L> {
    pub state: T,
    pub label: L,
}

impl<T, L> Plugin for StatusPlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,
{
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .with_system(status_complete)
            .with_system(status_cleanup)
            .with_system(status_final_spawn)
            .with_system(status_dispel_spawn);
        app.add_system_set(set);
    }
}
