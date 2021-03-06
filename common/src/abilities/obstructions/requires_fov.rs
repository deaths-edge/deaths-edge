use std::f32::consts::PI;

use bevy::prelude::*;

use super::{CastOrAbilityFilter, Obstruction, UseObstructions};
use crate::{
    abilities::{Source, Target},
    character::CharacterMarker,
};

/// Requires that target is in Field of View.
#[derive(Default, Clone, Debug, Component)]
pub struct RequiresFov;

pub struct OutOfFieldOfView(pub f32);

/// Check whether target is in front of source.
fn check_fov(source: &Transform, target: Vec3) -> Result<f32, OutOfFieldOfView> {
    let diff = target - source.translation;
    let angle = diff.angle_between(source.rotation * Vec3::new(0., 1., 0.));
    if -PI / 2. < angle && angle < PI / 2. {
        Ok(angle)
    } else {
        Err(OutOfFieldOfView(angle))
    }
}

pub fn check_required_fov(
    character_query: Query<(Option<&Target>, &Transform), With<CharacterMarker>>,
    mut ability_query: Query<
        (&Source, &mut UseObstructions),
        (CastOrAbilityFilter, With<RequiresFov>),
    >,
    target_query: Query<&Transform, With<CharacterMarker>>,
) {
    for (source, mut obstructions) in ability_query.iter_mut() {
        let (target, self_transform) = character_query
            .get(source.0)
            .expect("failed to find character");
        if let Some(&Target(target_id)) = target {
            let target_transform = target_query.get(target_id).expect("failed to find target");

            let in_front = check_fov(self_transform, target_transform.translation).is_ok();

            if in_front {
                obstructions.0.remove(&Obstruction::OutOfFOV);
            } else {
                obstructions.0.insert(Obstruction::OutOfFOV);
            }
        }
    }
}
