use std::{fmt::Debug, hash::Hash, time::Duration};

use bevy::prelude::*;

use crate::{
    abilities::{
        obstructions::{RequiresStationary, UseObstructions},
        Source,
    },
    character::{CastId, CharacterEntityAction, CharacterMarker, Motion},
    dyn_command::DynEntityMutate,
};

use super::DESPAWN_LABEL;

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
    cast_query: Query<&Source, (With<CastMarker>, Or<(With<Failed>, With<Complete>)>)>,
    character_query: Query<Entity, With<CharacterMarker>>,

    mut commands: Commands,
) {
    for source in cast_query.iter() {
        let character_id = character_query
            .get(source.0)
            .expect("failed to find character");

        commands.entity(character_id).remove::<CastId>();
    }
}

/// Anchors new casts to cast state.
pub fn cast_anchor(
    cast_query: Query<(Entity, &Source), Added<CastMarker>>,
    character_query: Query<Entity, With<CharacterMarker>>,

    mut commands: Commands,
) {
    for (cast_id, &Source(source)) in cast_query.iter() {
        error!("anchoring cast");
        let character_id = character_query
            .get(source)
            .expect("failed to find character");

        commands.entity(character_id).insert(CastId(cast_id));
    }
}

/// Checks if cast has completed.
pub fn cast_obstruct(
    cast_query: Query<
        (Entity, &UseObstructions),
        (With<CastMarker>, Without<Failed>, Without<Complete>),
    >,

    mut commands: Commands,
) {
    for (cast_id, obstructions) in cast_query.iter() {
        if !obstructions.0.is_empty() {
            commands.entity(cast_id).insert(Failed);
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
            .with_system(cast_obstruct)
            .with_system(cast_movement_interrupt);
        app.add_system_set(anchor).add_system_set(set);
    }
}
