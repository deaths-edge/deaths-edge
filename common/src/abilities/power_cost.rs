use bevy::prelude::*;

use super::{
    AbilityInstance, AbilityMarker, AbilitySource, Complete, Obstruction, UseObstructions,
};
use crate::character::{CharacterMarker, Power};

/// Ability costs power.
pub struct PowerCost(pub f32);

/// Check whether character has sufficient power.
pub fn check_power_cost(
    mut ability_query: Query<
        (&AbilitySource, &PowerCost, &mut UseObstructions),
        With<AbilityMarker>,
    >,
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

// Looks for instances of the ability and then applies the power cost to the character.
pub fn apply_power_cost(
    instance_query: Query<&AbilityInstance, With<Complete>>,
    ability_query: Query<(&AbilitySource, &PowerCost), With<AbilityMarker>>,
    mut character_query: Query<&mut Power, With<CharacterMarker>>,
) {
    for AbilityInstance(id) in instance_query.iter() {
        let (source, cost) = ability_query.get(*id).expect("failed to find ability");
        let mut power = character_query
            .get_mut(source.0)
            .expect("missing character");

        power.current -= cost.0;
        info!(current_power = ?power.current);
        // assert!(power.current > 0.);
    }
}
