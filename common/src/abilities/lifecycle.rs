use bevy::prelude::*;

use crate::character::{Cast, CastState, CharacterMarker};

use super::{AbilityInstance, AbilityMarker, AbilitySource, CastType};

// pub enum Lifecycle {
//     Preparing,
//     Casting,
//     Complete,
// }

pub struct Preparing;

/// If the ability is a cast, then switches from [`Preparing`] to [`Casting`] and sets character
/// [`CastState`].
///
/// If the ability is an instant, then switch from [`Preparing`] to [`Complete`].
pub fn initialize_cast(
    time: Res<Time>,

    instance_query: Query<(Entity, &AbilityInstance), (With<Preparing>, Without<Casting>)>,
    ability_query: Query<(&AbilitySource, &CastType), With<AbilityMarker>>,
    mut character_query: Query<&mut CastState, With<CharacterMarker>>,

    mut commands: Commands,
) {
    let start = time.last_update().expect("failed to find last update");

    for (instance_id, ability_id) in instance_query.iter() {
        error!("instance found");
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

pub struct Casting;

/// Waits until casts are complete then switches from [`Casting`] to [`Complete`].
pub fn complete_casting(
    time: Res<Time>,

    instance_query: Query<(Entity, &AbilityInstance), (With<Casting>, Without<Complete>)>,
    ability_query: Query<(&AbilitySource, &CastType), With<AbilityMarker>>,
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
                    commands
                        .entity(instance_id)
                        .insert(Complete)
                        .remove::<Casting>();

                    cast_state.0 = None;
                }
            }
            CastType::Channel(_) => todo!(),
        }
    }
}

pub struct Complete;

/// Removes ability instances which is [`Complete`].
pub fn remove_instance(
    query: Query<Entity, (With<AbilityInstance>, With<Complete>)>,
    mut commands: Commands,
) {
    for id in query.iter() {
        commands.entity(id).despawn();
    }
}
