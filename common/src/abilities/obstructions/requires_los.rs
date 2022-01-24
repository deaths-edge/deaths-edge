use bevy::prelude::*;
use heron::{
    prelude::*,
    rapier_plugin::{PhysicsWorld, RayCastInfo},
};

use super::{CastOrAbilityFilter, Obstruction, UseObstructions};
use crate::{
    abilities::Source,
    character::{CharacterMarker, OptionalTarget},
    physics::WorldLayer,
};

/// Requires that the target is in LoS.
#[derive(Default, Clone, Debug, Component)]
pub struct RequiresLoS;

#[derive(Debug)]
pub struct LineOfSightObstruction;

pub fn check_los(
    source: &Transform,
    target: Vec3,
    physics_world: &PhysicsWorld,
) -> Result<(), LineOfSightObstruction> {
    let source_position = source.translation;
    let ray = target - source_position;
    let collision_opt = physics_world.ray_cast_with_filter(
        source_position,
        ray,
        true,
        CollisionLayers::none()
            .with_group(WorldLayer::Spell)
            .with_mask(WorldLayer::Environment),
        |_| true,
    );
    if let Some(RayCastInfo {
        collision_point, ..
    }) = collision_opt
    {
        let collision_diff = source_position - collision_point;
        if ray.length() < collision_diff.length() {
            Ok(())
        } else {
            Err(LineOfSightObstruction)
        }
    } else {
        Ok(())
    }
}

pub fn check_required_los(
    physics_world: PhysicsWorld,

    character_query: Query<(&OptionalTarget, &Transform), With<CharacterMarker>>,
    mut ability_query: Query<
        (&Source, &mut UseObstructions),
        (CastOrAbilityFilter, With<RequiresLoS>),
    >,
    target_query: Query<&Transform, With<CharacterMarker>>,
) {
    for (source, mut obstructions) in ability_query.iter_mut() {
        let (target, self_transform) = character_query
            .get(source.0)
            .expect("failed to find character");

        if let Some(target_id) = target.0 {
            let target_transform = target_query
                .get(target_id.0)
                .expect("failed to find target");

            let in_los =
                check_los(self_transform, target_transform.translation, &physics_world).is_ok();

            if in_los {
                obstructions.0.remove(&Obstruction::OutOfLoS);
            } else {
                obstructions.0.insert(Obstruction::OutOfLoS);
            }
        }
    }
}
