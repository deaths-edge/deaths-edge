use std::time::Duration;

use bevy::prelude::*;

use crate::character::{CharacterMarker, Health};

use super::{AbilityInstance, AbilityMarker, Complete, InstantDamage, Target};

pub struct InstantInterrupt(pub Duration);

pub fn apply_interrupt(
    instance_query: Query<(&AbilityInstance, &Target), With<Complete>>,
    ability_query: Query<&InstantDamage, With<AbilityMarker>>,
    mut character_query: Query<&mut Health, With<CharacterMarker>>,
) {
    // for (ability_id, target) in instance_query.iter() {
    //     if let Ok(InstantDamage(damage)) = ability_query.get(ability_id.0) {
    //         let mut health = character_query
    //             .get_mut(target.0)
    //             .expect("failed to find target");
    //         health.apply_damage(*damage);
    //     }
    // }
}
