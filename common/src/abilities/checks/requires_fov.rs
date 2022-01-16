use std::f32::consts::PI;

use bevy::prelude::*;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityMarker, CharacterId},
    character::{CharacterMarker, OptionalTarget},
};

/// Requires that target is in Field of View.
#[derive(Default, Debug, Component)]
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
    mut ability_query: Query<
        (&CharacterId, &mut UseObstructions),
        (With<AbilityMarker>, With<RequiresFov>),
    >,
    character_query: Query<(&OptionalTarget, &Transform), With<CharacterMarker>>,
    target_query: Query<&Transform, With<CharacterMarker>>,
) {
    for (source, mut obstructions) in ability_query.iter_mut() {
        let (target, self_transform) = character_query
            .get(source.0)
            .expect("missing ability source");

        if let Some(target_id) = target.0 {
            let target_transform = target_query
                .get(target_id.0)
                .expect("failed to find target");

            let in_front = check_fov(self_transform, target_transform.translation).is_ok();

            if in_front {
                obstructions.0.remove(&Obstruction::OutOfFOV);
            } else {
                obstructions.0.insert(Obstruction::OutOfFOV);
            }
        }
    }
}
