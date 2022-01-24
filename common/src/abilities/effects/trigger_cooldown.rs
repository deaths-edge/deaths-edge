use std::time::Duration;

use bevy::prelude::*;

use crate::character::LastCastInstant;

use super::AbilityEffect;

#[derive(Clone, Debug, Component)]
pub struct TriggerCooldown(pub Duration);

impl AbilityEffect for TriggerCooldown {
    type Domain<'a> = &'a mut LastCastInstant;

    fn apply(&self, time: &Time, mut item: Mut<'_, LastCastInstant>, _commands: &mut Commands) {
        let now = time.last_update().expect("failed to find last update");
        *item = LastCastInstant(Some(now));
    }
}
