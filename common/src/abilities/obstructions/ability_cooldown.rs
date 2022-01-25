use std::time::Duration;

use bevy::prelude::*;

use super::{Obstruction, UseObstructions};
use crate::{abilities::AbilityMarker, character::LastCastInstant};

/// Ability has a cooldown (distinct from [`OnGlobalCooldown`](super::OnGlobalCooldown)).
#[derive(Debug, Clone, Component)]
pub struct OnCooldown(pub Duration);

pub fn check_cooldown(
    time: Res<Time>,
    mut ability_query: Query<
        (&OnCooldown, &LastCastInstant, &mut UseObstructions),
        With<AbilityMarker>,
    >,
) {
    let last_update = time.last_update().expect("cannot find last update");

    for (cooldown, last_cast, mut obstructions) in ability_query.iter_mut() {
        let last_cast = if let Some(last_cast) = last_cast.0 {
            last_cast
        } else {
            continue;
        };
        if last_cast + cooldown.0 < last_update {
            obstructions.0.remove(&Obstruction::OnCooldown);
        } else {
            obstructions.0.insert(Obstruction::OnCooldown);
        }
    }
}
