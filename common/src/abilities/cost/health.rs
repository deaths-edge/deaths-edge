use bevy::prelude::*;

use crate::{
    abilities::{AbilityId, AbilityInstanceMarker, AbilityMarker, CharacterId, Complete},
    character::{CharacterMarker, Power},
};

/// Ability costs health.
#[derive(Debug, Component)]
pub struct HealthCost(pub f32);

// Looks for instances of the ability and then applies the health cost to the character.
pub fn apply_health_cost(
    instance_query: Query<&AbilityId, (With<Complete>, With<AbilityInstanceMarker>)>,
    ability_query: Query<(&CharacterId, &HealthCost), With<AbilityMarker>>,
    mut character_query: Query<&mut Power, With<CharacterMarker>>,
) {
    let iter = instance_query
        .iter()
        .filter_map(|id| ability_query.get(id.0).ok());

    for (source, cost) in iter {
        let mut power = character_query
            .get_mut(source.0)
            .expect("missing character");
        power.subtract(cost.0);
    }
}
