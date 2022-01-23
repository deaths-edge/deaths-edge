use bevy::prelude::*;
use heron::Velocity;

use super::{CastOrAbilityFilter, Obstruction, UseObstructions};
use crate::{abilities::Source, character::CharacterMarker};

/// Requires that target is stationary while casting.
#[derive(Default, Debug, Clone, Component)]
pub struct RequiresStationary;

pub fn check_required_stationary(
    character_query: Query<&Velocity, (With<CharacterMarker>, Changed<Velocity>)>,
    mut ability_query: Query<
        (&mut UseObstructions, &Source),
        (CastOrAbilityFilter, With<RequiresStationary>),
    >,
) {
    for (mut obstructions, source) in ability_query.iter_mut() {
        let velocity = character_query
            .get(source.0)
            .expect("failed to find character");

        if velocity.linear == Vec3::ZERO {
            obstructions.0.remove(&Obstruction::NonStationary);
        } else {
            obstructions.0.insert(Obstruction::NonStationary);
        }
    }
}
