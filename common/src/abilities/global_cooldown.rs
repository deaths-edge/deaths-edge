use bevy::prelude::*;

use crate::character::{CharacterMarker, LastCastInstant, GLOBAL_COOLDOWN};

use super::{AbilityMarker, AbilitySource, Obstruction, UseObstructions};

/// Ability obeys global cooldown.
pub struct GlobalCooldown;

pub fn check_global_cooldown(
    time: Res<Time>,
    mut ability_query: Query<
        (&AbilitySource, &mut UseObstructions),
        (With<AbilityMarker>, With<GlobalCooldown>),
    >,
    character_query: Query<&LastCastInstant, With<CharacterMarker>>,
) {
    let last_update = time.last_update().expect("cannot find last update");

    for (source, mut obstructions) in ability_query.iter_mut() {
        let last_cast = character_query
            .get(source.0)
            .expect("missing ability source");
        if last_cast.0 + GLOBAL_COOLDOWN < last_update {
            obstructions.0.remove(&Obstruction::GlobalCooldown);
        } else {
            obstructions.0.insert(Obstruction::GlobalCooldown);
        }
    }
}
