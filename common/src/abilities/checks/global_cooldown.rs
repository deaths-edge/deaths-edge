use bevy::prelude::*;

use super::{Obstruction, UseObstructions};
use crate::{
    abilities::{AbilityId, AbilityMarker},
    character::{Abilities, CharacterMarker, LastCastInstant, GLOBAL_COOLDOWN},
};

/// Ability obeys global cooldown.
#[derive(Default, Debug, Component)]
pub struct GlobalCooldown;

pub fn check_global_cooldown(
    character_query: Query<(&Abilities, &LastCastInstant), With<CharacterMarker>>,
    time: Res<Time>,
    mut ability_query: Query<&mut UseObstructions, (With<AbilityMarker>, With<GlobalCooldown>)>,
) {
    let last_update = time.last_update().expect("cannot find last update");

    for (abilities, last_cast) in character_query.iter() {
        for AbilityId(ability_id) in *abilities {
            if let Ok(mut obstructions) = ability_query.get_mut(ability_id) {
                if last_cast.0 + GLOBAL_COOLDOWN < last_update {
                    obstructions.0.remove(&Obstruction::GlobalCooldown);
                } else {
                    obstructions.0.insert(Obstruction::GlobalCooldown);
                }
            }
        }
    }
}
