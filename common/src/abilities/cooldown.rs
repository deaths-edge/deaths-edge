use std::time::Duration;

use bevy::prelude::*;

use crate::character::LastCastInstant;

use super::{AbilityMarker, Obstruction, UseObstructions};

/// Ability has a cooldown (distinct from [`GlobalCooldown`](super::GlobalCooldown)).
pub struct Cooldown(pub Duration);

pub fn check_cooldown(
    time: Res<Time>,
    mut ability_query: Query<
        (&Cooldown, &LastCastInstant, &mut UseObstructions),
        With<AbilityMarker>,
    >,
) {
    let last_update = time.last_update().expect("cannot find last update");

    for (cooldown, last_cast, mut obstructions) in ability_query.iter_mut() {
        if last_cast.0 + cooldown.0 < last_update {
            obstructions.0.remove(&Obstruction::Cooldown);
        } else {
            obstructions.0.insert(Obstruction::Cooldown);
        }
    }
}
