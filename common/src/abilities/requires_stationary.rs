use bevy::prelude::*;
use heron::Velocity;

use crate::character::CharacterMarker;

use super::{AbilityMarker, AbilitySource, Obstruction, UseObstructions};

pub struct RequiresStationary;

pub fn check_required_stationary(
    mut ability_query: Query<(&AbilitySource, &mut UseObstructions), With<AbilityMarker>>,
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
