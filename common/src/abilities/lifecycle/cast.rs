use std::{fmt::Debug, hash::Hash, time::Duration};

use bevy::prelude::*;

use crate::{
    abilities::{
        obstructions::{RequiresStationary, UseObstructions},
        Source,
    },
    character::{CastRef, CastState, CharacterEntityAction, CharacterMarker, Motion},
    dyn_command::DynEntityMutate,
};

use super::{TotalDuration, DESPAWN_LABEL};

#[derive(Debug, Default, Clone, Component)]
pub struct CastMarker;

#[derive(Component)]
pub struct Cast {
    pub command: DynEntityMutate,
    pub duration: Duration,
}

#[derive(Debug, Default, Clone, Component)]
pub struct Complete;

#[derive(Component)]
pub struct Failed;

// TODO: Remove this?
pub fn cast_despawn(
    cast_query: Query<(Entity, &Source), (With<CastMarker>, Or<(With<Failed>, With<Complete>)>)>,
    mut character_cast: Query<&mut CastState, With<CharacterMarker>>,
) {
    for source in cast_query.iter() {
        let mut cast_state = character_cast
            .get_mut(source.0)
            .expect("failed to find character");

        cast_state.0 = None;
    }
}

/// Anchors new casts to cast state.
pub fn cast_anchor(
    time: Res<Time>,
    cast_query: Query<(Entity, &Source), Added<CastMarker>>,
    mut character_query: Query<&mut CastState, With<CharacterMarker>>,
) {
    let now = time.last_update().expect("failed to find last update");

    for (cast_id, Source(source)) in cast_query.iter() {
        let mut cast_state = character_query
            .get_mut(*source)
            .expect("failed to find character");
        let cast = CastRef {
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
        (&TotalDuration, &UseObstructions, &Source),
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

pub struct CastPlugin<T, L> {
    pub state: T,
    pub label: L,
}

impl<T, L> Plugin for CastPlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,
{
    fn build(&self, app: &mut App) {
        const CAST_ANCHOR_LABEL: &str = "cast-anchor";
        let anchor = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .label(CAST_ANCHOR_LABEL)
            .before(DESPAWN_LABEL)
            .with_system(cast_anchor);
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .after(CAST_ANCHOR_LABEL)
            .before(DESPAWN_LABEL)
            .with_system(cast_despawn)
            .with_system(cast_complete)
            .with_system(cast_movement_interrupt);
        app.add_system_set(anchor).add_system_set(set);
    }
}
