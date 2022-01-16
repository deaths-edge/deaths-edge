use bevy::prelude::*;
use heron::Velocity;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityMarker, CharacterId},
    character::CharacterMarker,
};

/// Requires that target is stationary while casting.
#[derive(Default, Debug, Component)]
pub struct RequiresStationary;

pub fn check_required_stationary(
    mut ability_query: Query<
        (&CharacterId, &mut UseObstructions),
        (With<AbilityMarker>, With<RequiresStationary>),
    >,
    character_query: Query<&Velocity, (With<CharacterMarker>, Changed<Velocity>)>,
) {
    for (source, mut obstructions) in ability_query.iter_mut() {
        if let Ok(velocity) = character_query.get(source.0) {
            if velocity.linear == Vec3::ZERO {
                obstructions.0.remove(&Obstruction::NonStationary);
            } else {
                obstructions.0.insert(Obstruction::NonStationary);
            }
        }
    }
}
