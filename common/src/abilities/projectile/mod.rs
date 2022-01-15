mod damage;
mod speed;
mod target;
mod track_target;

use bevy::prelude::*;
use heron::Velocity;

use super::{
    AbilityId, AbilityInstanceMarker, AbilityMarker, CharacterId, Complete, DeleteObstructions,
};
use crate::{
    abilities::{DeleteObstruction, Preparing},
    character::CharacterMarker,
};

pub use damage::*;
pub use speed::*;
pub use target::*;
pub use track_target::*;

#[derive(Default, Debug, Component)]
pub struct ProjectileMarker;

#[derive(Default, Debug, Component)]
pub struct SpawnProjectile;

#[derive(Debug, Component)]
pub struct AbilityInstanceId(pub Entity);

#[derive(Bundle)]
pub struct BaseProjectile {
    marker: ProjectileMarker,
    instance_id: AbilityInstanceId,
    ability_id: AbilityId,
    transform: Transform,
    global_transform: GlobalTransform,
    velocity: Velocity,
}

pub fn spawn_projectile(
    mut instance_query: Query<
        (Entity, &AbilityId, &mut DeleteObstructions),
        (With<Complete>, With<AbilityInstanceMarker>),
    >,
    ability_query: Query<&CharacterId, (With<SpawnProjectile>, With<AbilityMarker>)>,
    character_query: Query<(&Transform, &Velocity), With<CharacterMarker>>,
    mut commands: Commands,
) {
    for (instance_id, ability_id, mut delete_obstructions) in instance_query.iter_mut() {
        if let Ok(character_id) = ability_query.get(ability_id.0) {
            let (transform, velocity) = character_query
                .get(character_id.0)
                .expect("failed to find transform");

            info!(message = "spawned projectile", state = ?Preparing, ?transform);

            delete_obstructions
                .0
                .insert(DeleteObstruction::WaitingForProjectile);

            commands
                .spawn()
                .insert_bundle(BaseProjectile {
                    marker: ProjectileMarker,
                    instance_id: AbilityInstanceId(instance_id),
                    ability_id: *ability_id,
                    velocity: *velocity,
                    transform: *transform,
                    global_transform: GlobalTransform::default(),
                })
                .insert(Preparing);
        }
    }
}
