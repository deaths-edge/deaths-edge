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

    for (source, mut obstructions) in ability_query.iter_mut() {
        let last_cast = character_query
            .get(source.0)
            .expect("failed to find character");
        if last_cast.0 + GLOBAL_COOLDOWN < last_update {
            obstructions.0.remove(&Obstruction::GlobalCooldown);
        } else {
            obstructions.0.insert(Obstruction::GlobalCooldown);
        }
    }
}
