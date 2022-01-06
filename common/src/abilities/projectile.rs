use bevy::prelude::*;
use heron::Velocity;

use super::{AbilityId, Complete};

pub struct ProjectileMarker;

pub struct IncludeProjectile;

pub fn spawn_projectile(
    query: Query<&AbilityId, (With<IncludeProjectile>, With<Complete>)>,
    mut commands: Commands,
) {
    for index in query.iter() {
        // commands.spawn().insert(ProjectileMarker).insert(Velocity);
    }
}

pub struct ProjectileDamage(pub f32);

pub struct ProjectileSpeed(pub f32);

// pub struct ProjectileSprite(SpriteBundle);
