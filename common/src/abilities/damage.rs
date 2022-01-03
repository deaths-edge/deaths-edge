use bevy::prelude::*;

use super::AbilityInstance;

pub struct TargetDamage(pub f32);

// pub fn apply_damage(
//     instance_query: Query<&AbilityInstance>,
//     ability_query: Query<(&AbilitySource, &PowerCost), With<AbilityMarker>>,
//     mut character_query: Query<&mut Power, With<CharacterMarker>>,
// ) {
// }
