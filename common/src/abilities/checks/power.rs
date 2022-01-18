use bevy::prelude::*;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityId, AbilityMarker},
    character::{Abilities, CharacterMarker, Power},
};

/// Ability costs power.
#[derive(Default, Debug, Component)]
pub struct PowerCost(pub f32);

/// Check whether character has sufficient power.
pub fn check_power_cost(
    character_query: Query<(&Abilities, &Power), (With<CharacterMarker>, Changed<Power>)>,
    mut ability_query: Query<(&PowerCost, &mut UseObstructions), With<AbilityMarker>>,
) {
    for (abilities, power) in character_query.iter() {
        for AbilityId(ability_id) in *abilities {
            if let Ok((cost, mut obstructions)) = ability_query.get_mut(ability_id) {
                if power.current >= cost.0 {
                    obstructions.0.remove(&Obstruction::InsufficientPower);
                } else {
                    warn!(message = "insufficient power", current_power = ?power.current, cost = %cost.0);
                    obstructions.0.insert(Obstruction::InsufficientPower);
                }
            }
        }
    }
}
