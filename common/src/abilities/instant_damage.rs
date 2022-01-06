use bevy::prelude::*;

use super::{AbilityId, AbilityMarker, Complete, Target};
use crate::character::{CharacterMarker, Health};

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
