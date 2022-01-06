use bevy::prelude::*;

use crate::{
    abilities::{AbilityId, AbilityMarker, Complete, Target},
    character::{CharacterMarker, Health},
};

#[derive(Default, Debug)]
pub struct InstantDamage(pub f32);

pub fn apply_damage(
    instance_query: Query<(&AbilityId, &Target), With<Complete>>,
    ability_query: Query<&InstantDamage, With<AbilityMarker>>,
    mut character_query: Query<&mut Health, With<CharacterMarker>>,
) {
    for (ability_id, target) in instance_query.iter() {
        if let Ok(InstantDamage(damage)) = ability_query.get(ability_id.0) {
            let mut health = character_query
                .get_mut(target.0)
                .expect("failed to find target");
            health.apply_damage(*damage);
        }
    }
}
