use bevy::prelude::*;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityMarker, CharacterId},
    character::{CharacterMarker, Power},
};

/// Ability costs power.
#[derive(Default, Debug, Component)]
pub struct PowerCost(pub f32);

/// Check whether character has sufficient power.
pub fn check_power_cost(
    mut ability_query: Query<(&CharacterId, &PowerCost, &mut UseObstructions), With<AbilityMarker>>,
    character_query: Query<&Power, (With<CharacterMarker>, Changed<Power>)>,
) {
    for (source, cost, mut obstructions) in ability_query.iter_mut() {
        if let Ok(power) = character_query.get(source.0) {
            if power.current >= cost.0 {
                obstructions.0.remove(&Obstruction::InsufficientPower);
            } else {
                warn!(message = "insufficient power", current_power = ?power.current, cost = %cost.0);
                obstructions.0.insert(Obstruction::InsufficientPower);
            }
        }
    }
}
