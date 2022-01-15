use std::collections::HashSet;

use bevy::prelude::*;

use crate::character::{Cast, CastState, CharacterMarker};

use super::{
    AbilityId, AbilityInstanceId, AbilityInstanceMarker, AbilityMarker, CastType, CharacterId,
    ProjectileMarker,
};

#[derive(Debug, Default, Component)]
pub struct Preparing;

/// If the ability is a cast, then switches from [`Preparing`] to [`Casting`] and sets character
/// [`CastState`].
///
/// If the ability is an instant, then switch from [`Preparing`] to [`Complete`].
pub fn initialize_cast(
    time: Res<Time>,

    instance_query: Query<
        (Entity, &AbilityId),
        (
            With<Preparing>,
            Without<Casting>,
            With<AbilityInstanceMarker>,
        ),
    >,
    ability_query: Query<(&CharacterId, &CastType), With<AbilityMarker>>,
    mut character_query: Query<&mut CastState, With<CharacterMarker>>,

    mut commands: Commands,
) {
    let start = time.last_update().expect("failed to find last update");

    for (instance_id, ability_id) in instance_query.iter() {
        let (source, cast_type) = ability_query
            .get(ability_id.0)
            .expect("failed to find ability");

        match cast_type {
            CastType::Instant => {
                commands
                    .entity(instance_id)
                    .insert(Complete)
                    .remove::<Preparing>();
            }
            CastType::Cast(_) => {
                let mut cast_state = character_query
                    .get_mut(source.0)
                    .expect("failed to find character");

                cast_state.0 = Some(Cast { start, instance_id });

                commands
                    .entity(instance_id)
                    .insert(Casting)
                    .remove::<Preparing>();
            }
            CastType::Channel(_) => todo!(),
        }
    }
}

#[derive(Default, Debug, Component)]
pub struct InFlight;

/// Switches projectile from [`Preparing`] to [`InFlight`].
///
/// Remove [`DeleteObstruction::WaitingForProjectile`] from [`DeleteObstructions`] on instance.
pub fn initialize_projectile(
    projectile_query: Query<
        (Entity, &AbilityInstanceId),
        (With<Preparing>, With<ProjectileMarker>),
    >,
    mut instance_query: Query<&mut DeleteObstructions, With<AbilityInstanceMarker>>,
    mut commands: Commands,
) {
    for (projectile_id, instance_id) in projectile_query.iter() {
        // Switch from `Preparing` to `InFlight`
        info!(message = "projectile", from = ?Preparing, to = ?InFlight);
        commands
            .entity(projectile_id)
            .remove::<Preparing>()
            .insert(InFlight);

        // Remove `WaitingForProjectile` obstruction.
        let mut delete_obstructions = instance_query
            .get_mut(instance_id.0)
            .expect("instance not found");
        delete_obstructions
            .0
            .remove(&DeleteObstruction::WaitingForProjectile);
    }
}

#[derive(Debug, Default, Component)]
pub struct Casting;

/// Waits until casts are complete then switches from [`Casting`] to [`Complete`].
pub fn complete_casting(
    time: Res<Time>,

    instance_query: Query<
        (Entity, &AbilityId),
        (
            With<Casting>,
            Without<Complete>,
            Without<Failed>,
            With<AbilityInstanceMarker>,
        ),
    >,
    ability_query: Query<(&CharacterId, &CastType), With<AbilityMarker>>,
    mut character_query: Query<&mut CastState, With<CharacterMarker>>,

    mut commands: Commands,
) {
    let now = time.last_update().expect("failed to find last update");

    for (instance_id, ability_id) in instance_query.iter() {
        let (source, cast_type) = ability_query
            .get(ability_id.0)
            .expect("failed to find ability");

        match cast_type {
            CastType::Instant => unreachable!("cannot be Casting as an instant"),
            CastType::Cast(duration) => {
                let mut cast_state = character_query
                    .get_mut(source.0)
                    .expect("failed to find character");

                let cast = cast_state.0.as_ref().expect("must be casting");

                assert_eq!(instance_id, cast.instance_id);

                let start = cast.start;

                if start + *duration < now {
                    let mut entity_commands = commands.entity(instance_id);
                    entity_commands.remove::<Casting>().insert(Complete);

                    info!(message = "ability instance", from = ?Casting, to = ?Complete);

                    cast_state.0 = None;
                }
            }
            CastType::Channel(_) => todo!(),
        }
    }
}

/// Waiting for projectile
#[derive(Debug)]
pub struct WaitingForProjectile;

/// Cast has completed successfully.
#[derive(Debug, Default, Component)]
pub struct Complete;

/// Cast has failed.
#[derive(Debug, Default, Component)]
pub struct Failed;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DeleteObstruction {
    WaitingForProjectile,
}

/// Prevents the ability from being immediately cleaned up.
#[derive(Debug, Default, Component)]
pub struct DeleteObstructions(pub HashSet<DeleteObstruction>);

#[derive(Debug, Default, Component)]
pub struct MarkDeletion;

/// Removes ability instances which is [`MarkDeletion`].
pub fn remove_instance(
    query: Query<
        (Entity, &DeleteObstructions, Option<&MarkDeletion>),
        (
            With<AbilityInstanceMarker>,
            Or<(Or<(With<Complete>, With<Failed>)>, With<MarkDeletion>)>,
        ),
    >,
    mut commands: Commands,
) {
    for (instance_id, obstructions, marked) in query.iter() {
        // If `Complete`/`Failed` and yet to be marked then do so
        if marked.is_none() {
            info!(message = "trimming instance", components = ?(Complete, Failed));
            commands
                .entity(instance_id)
                .remove::<Complete>()
                .remove::<Failed>()
                .insert(MarkDeletion);
        }
        // Check whether there are existing obstructions to deletion
        if !obstructions.0.is_empty() {
            // Do not despawn if obstructions exist
            warn!(message = "delete obstructions", ?obstructions);
            continue;
        }

        info!(message = "despawning ability instance");
        commands.entity(instance_id).despawn();
    }
}
