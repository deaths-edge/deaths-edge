use bevy::prelude::*;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityId, AbilityMarker},
    character::{Abilities, CharacterMarker, OptionalTarget},
};

/// Ability has a maximum range.
#[derive(Default, Debug, Component)]
pub struct MaximumRange(pub f32);

pub fn check_maximum_range(
    character_query: Query<(&Abilities, &OptionalTarget, &Transform), With<CharacterMarker>>,
    mut ability_query: Query<(&MaximumRange, &mut UseObstructions), With<AbilityMarker>>,
    target_query: Query<&Transform, With<CharacterMarker>>,
) {
    for (abilities, target, self_transform) in character_query.iter() {
        for AbilityId(ability_id) in *abilities {
            if let Ok((maximum_range, mut obstructions)) = ability_query.get_mut(ability_id) {
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
    }
}
