use bevy::prelude::*;

use super::{AbilityInstanceId, ProjectileMarker};
use crate::abilities::{AbilityId, AbilityInstanceMarker, AbilityMarker, Preparing, Target};

#[derive(Debug, Default, Component)]
pub struct ProjectileTarget;

/// Adds [`Target`] to a projectile from it's parent ability instance.
pub fn adjoin_projectile_target(
    projectile_query: Query<
        (Entity, &AbilityInstanceId, &AbilityId),
        (With<Preparing>, With<ProjectileMarker>),
    >,
    instance_query: Query<&Target, With<AbilityInstanceMarker>>,
    ability_query: Query<(), (With<AbilityMarker>, With<ProjectileTarget>)>,

    mut commands: Commands,
) {
    for (projectile_id, instance_id, ability_id) in projectile_query.iter() {
        // Check for `ProjectileTarget`
        if ability_query.get(ability_id.0).is_ok() {
            if let Ok(target) = instance_query.get(instance_id.0) {
                info!(message = "adjoin projectile", component = ?target);
                commands.entity(projectile_id).insert(*target);
            }
        }
    }
}
