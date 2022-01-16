use bevy::prelude::*;

use crate::{
    abilities::{AbilityMarker, Target},
    character::{CharacterMarker, Health},
};

#[derive(Default, Debug, Component)]
pub struct Damage(pub f32);

pub fn apply_target_damage(
    ability_query: Query<(&Damage, &Target), With<AbilityMarker>>,
    mut character_query: Query<&mut Health, With<CharacterMarker>>,
) {
    for (Damage(damage), target) in ability_query.iter() {
        let mut health = character_query
            .get_mut(target.0)
            .expect("failed to find target");
        health.apply_damage(*damage);
    }
}
