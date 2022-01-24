use bevy::prelude::*;

use crate::character::LastCastInstant;

use super::CharacterEffect;

#[derive(Default, Debug, Clone, Component)]
pub struct TriggerGlobalCooldown;

impl CharacterEffect for TriggerGlobalCooldown {
    type Domain<'a> = &'a mut LastCastInstant;

    type Param<'w, 's> = ();
    type Fetch = ();

    fn apply(
        &self,
        time: &Time,
        mut item: Mut<'_, LastCastInstant>,
        _param: &(),
        _commands: &mut Commands,
    ) {
        info!("applying global cooldown");
        let now = time.last_update().expect("last cast instant");
        *item = LastCastInstant(Some(now));
    }
}
