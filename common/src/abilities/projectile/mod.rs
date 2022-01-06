mod damage;

use bevy::prelude::*;
use heron::Velocity;

use super::{AbilityId, AbilityMarker, CharacterId, Complete};
use crate::character::CharacterMarker;

#[derive(Default)]
pub struct ProjectileMarker;

#[derive(Default, Debug)]
pub struct SpawnProjectile;

#[derive(Bundle, Default)]
pub struct BaseProjectile {
    marker: ProjectileMarker,
    transform: Transform,
    global_transform: GlobalTransform,
    velocity: Velocity,
}

pub fn spawn_projectile(
    instance_query: Query<&AbilityId, With<Complete>>,
    ability_query: Query<&CharacterId, (With<SpawnProjectile>, With<AbilityMarker>)>,
    character_query: Query<(&Transform, &Velocity), With<CharacterMarker>>,
    mut commands: Commands,
) {
    for ability_id in instance_query.iter() {
        if let Ok(character_id) = ability_query.get(ability_id.0) {
            let (transform, velocity) = character_query
                .get(character_id.0)
                .expect("failed to find transform");

            info!("spawned projectile");

            commands.spawn().insert_bundle(BaseProjectile {
                velocity: *velocity,
                transform: *transform,
                ..Default::default()
            });
        }
    }
}
