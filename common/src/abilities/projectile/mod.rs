mod damage;
mod track_target;

use bevy::prelude::*;
use heron::Velocity;

use super::{AbilityId, AbilityInstanceMarker, AbilityMarker, CharacterId, Complete};
use crate::{abilities::Preparing, character::CharacterMarker};

pub use damage::*;
pub use track_target::*;

#[derive(Default)]
pub struct ProjectileMarker;

#[derive(Default, Debug)]
pub struct SpawnProjectile;

#[derive(Debug)]
pub struct AbilityInstanceId(pub Entity);

#[derive(Bundle)]
pub struct BaseProjectile {
    marker: ProjectileMarker,
    instance_id: AbilityInstanceId,
    transform: Transform,
    global_transform: GlobalTransform,
    velocity: Velocity,
}

pub fn spawn_projectile(
    instance_query: Query<(Entity, &AbilityId), (With<Complete>, With<AbilityInstanceMarker>)>,
    ability_query: Query<&CharacterId, (With<SpawnProjectile>, With<AbilityMarker>)>,
    character_query: Query<(&Transform, &Velocity), With<CharacterMarker>>,
    mut commands: Commands,
) {
    for (instance_id, ability_id) in instance_query.iter() {
        if let Ok(character_id) = ability_query.get(ability_id.0) {
            let (transform, velocity) = character_query
                .get(character_id.0)
                .expect("failed to find transform");

            error!("spawned projectile");

            commands
                .spawn()
                .insert_bundle(BaseProjectile {
                    marker: ProjectileMarker,
                    instance_id: AbilityInstanceId(instance_id),
                    velocity: *velocity,
                    transform: *transform,
                    global_transform: GlobalTransform::default(),
                })
                .insert(Preparing);
        }
    }
}