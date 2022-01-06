use bevy::prelude::*;
use heron::Velocity;

use crate::character::CharacterMarker;

use super::{AbilityId, AbilityMarker, CharacterId, Complete};

#[derive(Default)]
pub struct ProjectileMarker;

pub struct IncludeProjectile;

#[derive(Bundle, Default)]
pub struct BaseProjectile {
    marker: ProjectileMarker,
    transform: Transform,
    global_transform: GlobalTransform,
    velocity: Velocity,
}

pub fn spawn_projectile(
    instance_query: Query<&AbilityId, (With<IncludeProjectile>, With<Complete>)>,
    ability_query: Query<&CharacterId, With<AbilityMarker>>,
    character_query: Query<(&Transform, &Velocity), With<CharacterMarker>>,
    mut commands: Commands,
) {
    for ability_id in instance_query.iter() {
        let (transform, velocity) = ability_query
            .get(ability_id.0)
            .and_then(|character_id| character_query.get(character_id.0))
            .expect("failed to get transform");
        commands.spawn().insert_bundle(BaseProjectile {
            velocity: *velocity,
            transform: *transform,
            ..Default::default()
        });
    }
}

pub struct ProjectileDamage(pub f32);

pub struct ProjectileSpeed(pub f32);

// pub struct ProjectileSprite(SpriteBundle);
