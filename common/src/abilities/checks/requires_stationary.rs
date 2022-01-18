use bevy::prelude::*;
use heron::Velocity;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityId, AbilityMarker},
    character::{Abilities, CharacterMarker},
};

/// Requires that target is stationary while casting.
#[derive(Default, Debug, Component)]
pub struct RequiresStationary;

pub fn check_required_stationary(
    character_query: Query<(&Abilities, &Velocity), (With<CharacterMarker>, Changed<Velocity>)>,
    mut ability_query: Query<&mut UseObstructions, (With<AbilityMarker>, With<RequiresStationary>)>,
) {
    for (abilities, velocity) in character_query.iter() {
        for AbilityId(ability_id) in *abilities {
            if let Ok(mut obstructions) = ability_query.get_mut(ability_id) {
                if velocity.linear == Vec3::ZERO {
                    obstructions.0.remove(&Obstruction::NonStationary);
                } else {
                    obstructions.0.insert(Obstruction::NonStationary);
                }
            }
        }
    }
}
