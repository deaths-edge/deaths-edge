use bevy::prelude::*;

use heron::Velocity;

use crate::{
    abilities::{InFlight, Target},
    character::CharacterMarker,
};

use super::ProjectileMarker;

#[derive(Debug, Default)]
pub struct ProjectileTrackTarget;

/// Alters [`Velocity`] so that projectiles to track targets.
pub fn projectile_tracking(
    mut projectile_query: Query<
        (&Transform, &mut Velocity, &Target),
        (With<InFlight>, With<ProjectileMarker>),
    >,

    character_query: Query<&Transform, (With<CharacterMarker>, Changed<Transform>)>,
) {
    for (projectile_transform, mut velocity, target) in projectile_query.iter_mut() {
        info!("tracking projectile");
        if let Ok(target_transform) = character_query.get(target.0) {
            let direction = (target_transform.translation - projectile_transform.translation)
                .normalize_or_zero();
            let speed = velocity.linear.length();

            *velocity.linear = *(direction);
        }
    }
}

pub fn move_projectile(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<ProjectileMarker>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        *transform.translation = *(time.delta().as_secs_f32() * velocity.linear);
    }
}
