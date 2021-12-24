use bevy::prelude::*;
use heron::{
    prelude::*,
    rapier_plugin::{PhysicsWorld, RayCastInfo},
};

use crate::{
    character::{LastCastInstant, GLOBAL_COOLDOWN},
    physics::WorldLayer,
};

pub struct GlobalCooldown;

pub fn check_global_cooldown(
    time: &Time,
    last_cast_instant: &LastCastInstant,
) -> Result<(), GlobalCooldown> {
    if last_cast_instant.elapsed(time) > GLOBAL_COOLDOWN {
        Ok(())
    } else {
        Err(GlobalCooldown)
    }
}

#[derive(Debug)]
pub struct LineOfSightObstruction;

pub fn check_line_of_sight(
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

pub struct OutOfFieldOfView(pub f32);

/// Check whether target is in front of source.
pub fn check_in_front(source: &Transform, target: Vec3) -> Result<f32, OutOfFieldOfView> {
    let diff = target - source.translation;
    let angle = diff.angle_between(source.rotation * Vec3::new(0., 1., 0.));
    if -std::f32::consts::PI / 2. < angle && angle < std::f32::consts::PI / 2. {
        Ok(angle)
    } else {
        Err(OutOfFieldOfView(angle))
    }
}
