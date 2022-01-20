use bevy::prelude::*;

use super::{CastOrAbilityFilter, Obstruction, UseObstructions};
use crate::{
    abilities::Source,
    character::{CharacterMarker, OptionalTarget},
};

/// Ability has a maximum range.
#[derive(Default, Debug, Component)]
pub struct MaximumRange(pub f32);

pub fn check_maximum_range(
    character_query: Query<(&OptionalTarget, &Transform), With<CharacterMarker>>,
    mut ability_query: Query<(&Source, &MaximumRange, &mut UseObstructions), CastOrAbilityFilter>,
    target_query: Query<&Transform, With<CharacterMarker>>,
) {
    for (source, maximum_range, mut obstructions) in ability_query.iter_mut() {
        let (target, self_transform) = character_query
            .get(source.0)
            .expect("failed to find character");

        if let Some(target_id) = target.0 {
            let target_transform = target_query
                .get(target_id.0)
                .expect("failed to find target");

            let distance = self_transform
                .translation
                .distance(target_transform.translation);

            if maximum_range.0 > distance {
                obstructions.0.remove(&Obstruction::OutOfRange);
            } else {
                obstructions.0.insert(Obstruction::OutOfRange);
            }
        }
    }
}
