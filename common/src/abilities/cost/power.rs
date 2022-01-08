use bevy::prelude::*;

use crate::{
    abilities::{
        AbilityId, AbilityInstanceMarker, AbilityMarker, CharacterId, Complete, Obstruction,
        UseObstructions,
    },
    character::{CharacterMarker, Power},
};

/// Ability costs power.
#[derive(Default, Debug)]
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

// Looks for instances of the ability and then applies the power cost to the character.
pub fn apply_power_cost(
    instance_query: Query<&AbilityId, (With<Complete>, With<AbilityInstanceMarker>)>,
    ability_query: Query<(&CharacterId, &PowerCost), With<AbilityMarker>>,
    mut character_query: Query<&mut Power, With<CharacterMarker>>,
) {
    for AbilityId(id) in instance_query.iter() {
        let (source, cost) = ability_query.get(*id).expect("failed to find ability");
        let mut power = character_query
            .get_mut(source.0)
            .expect("missing character");

        power.current -= cost.0;
        info!(current_power = ?power.current);
        // assert!(power.current > 0.);
    }
}