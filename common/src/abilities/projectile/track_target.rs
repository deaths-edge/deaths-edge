use std::ops::Mul;

use bevy::prelude::*;

use heron::Velocity;

use crate::{
    abilities::{AbilityId, AbilityInstanceMarker, Complete, InFlight, Preparing, Target},
    character::CharacterMarker,
};

use super::{AbilityInstanceId, ProjectileMarker};

#[derive(Debug, Default)]
pub struct ProjectileTrackTarget;

/// Adds [`Target`] to a projectile from it's parent ability instance.
pub fn adjoin_projectile_target(
    mut projectile_query: Query<
        (Entity, &AbilityInstanceId),
        (With<Preparing>, With<ProjectileMarker>),
    >,
    instance_query: Query<
        (Entity, &AbilityId, &Target),
        (With<Complete>, With<AbilityInstanceMarker>),
    >,
) {
    for (projectile_id, instance_id) in projectile_query.iter() {
        error!("found preparing projectile");
        let (instance_id, ability_id, target) = instance_query
            .get(instance_id.0)
            .expect("failed to find instance");
    }
}

/// Alters [`Velocity`] so that projectiles to track targets.
pub fn projectile_tracking(
    mut projectile_query: Query<
        (&Transform, &mut Velocity, &Target),
        (With<InFlight>, With<ProjectileMarker>),
    >,

    character_query: Query<&Transform, With<CharacterMarker>>,
) {
    for (projectile_transform, mut velocity, target) in projectile_query.iter_mut() {
        let target_transform = character_query
            .get(target.0)
            .expect("failed to find target");

        let direction =
            (target_transform.translation - projectile_transform.translation).normalize_or_zero();
        let speed = velocity.linear.length();

        *velocity.linear = *(speed * direction);
    }
}
