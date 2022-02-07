mod cast;
mod status;

use std::{fmt::Debug, hash::Hash, time::Duration};

use bevy::{prelude::*, utils::Instant};

pub use cast::*;
pub use status::*;

use crate::dyn_command::DynEntityMutate;

pub const DESPAWN_LABEL: &str = "despawn";

#[derive(Debug, Clone, Component)]
pub struct TotalDuration(pub Duration);

#[derive(Default, Debug, Clone, Component)]
pub struct ProgressDuration(pub Duration);

#[derive(Debug, Component)]
pub struct Start(pub Instant);

#[derive(Clone, Debug, Component)]
pub struct OnComplete(pub DynEntityMutate);

pub fn on_complete_spawn(
    query: Query<(Entity, &OnComplete), (With<CastMarker>, With<Complete>)>,

    mut commands: Commands,
) {
    for (parent, on_complete) in query.iter() {
        let mut entity_commands = commands.spawn();

        on_complete.0.apply(parent, &mut entity_commands);
    }
}

pub fn despawn(
    query: Query<Entity, Or<(With<Complete>, With<Failed>, With<Dispelled>)>>,
    mut commands: Commands,
) {
    for id in query.iter() {
        commands.entity(id).despawn();
    }
}

/// Once total progress has been made insert [`Complete`].
pub fn complete_progress(
    query: Query<(Entity, &ProgressDuration, &TotalDuration)>,
    mut commands: Commands,
) {
    for (status_id, progress, total) in query.iter() {
        if progress.0 > total.0 {
            commands.entity(status_id).insert(Complete);
        }
    }
}

pub struct LifecyclePlugin<T, L> {
    pub state: T,
    pub label: L,
}

impl<T, L> Plugin for LifecyclePlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,
{
    fn build(&self, app: &mut App) {
        let cast_plugin = CastPlugin {
            state: self.state,
            label: self.label.clone(),
        };
        let status_plugin = StatusPlugin {
            state: self.state,
            label: self.label.clone(),
        };

        let completion = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .before(DESPAWN_LABEL)
            .with_system(complete_progress)
            .with_system(on_complete_spawn);

        let despawn = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .label(DESPAWN_LABEL)
            .with_system(despawn);

        app.add_plugin(cast_plugin)
            .add_plugin(status_plugin)
            .add_system_set(completion)
            .add_system_set(despawn); // TODO: Order this
    }
}
