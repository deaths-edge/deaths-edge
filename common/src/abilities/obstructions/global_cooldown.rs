use bevy::prelude::*;

use super::{CastOrAbilityFilter, Obstruction, UseObstructions};
use crate::{
    abilities::Source,
    character::{CharacterMarker, LastCastInstant, GLOBAL_COOLDOWN},
};

/// Ability requires a global cooldown.
#[derive(Default, Debug, Component)]
pub struct GlobalCooldown;

pub fn check_global_cooldown(
    character_query: Query<&LastCastInstant, With<CharacterMarker>>,
    time: Res<Time>,
    mut ability_query: Query<
        (&Source, &mut UseObstructions),
        (CastOrAbilityFilter, With<GlobalCooldown>),
    >,
) {
    let last_update = time.last_update().expect("cannot find last update");

    for (Source(source), mut obstructions) in ability_query.iter_mut() {
        let last_cast = character_query
            .get(*source)
            .expect("failed to find character");
        let last_cast = if let Some(last_cast) = last_cast.0 {
            last_cast
        } else {
            continue;
        };

        if last_cast + GLOBAL_COOLDOWN < last_update {
            obstructions.0.remove(&Obstruction::GlobalCooldown);
        } else {
            obstructions.0.insert(Obstruction::GlobalCooldown);
        }
    }
}
