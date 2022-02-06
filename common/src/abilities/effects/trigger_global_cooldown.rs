use bevy::prelude::*;

use crate::{
    abilities::{lifecycle::TotalDuration, AbilityMarker},
    character::LastCastInstant,
};

use super::CharacterEffect;

#[derive(Default, Debug, Clone, Component)]
pub struct TriggerGlobalCooldown;

impl CharacterEffect for TriggerGlobalCooldown {
    type Domain<'a> = &'a mut LastCastInstant;

    type Param<'w, 's> = Query<'w, 's, &'static TotalDuration, With<AbilityMarker>>;
    type Fetch = QueryState<&'static TotalDuration, With<AbilityMarker>>;

    fn apply(
        &self,
        parent_id: Entity,

        mut last_cast: Mut<'_, LastCastInstant>,
        param: &Query<&TotalDuration, With<AbilityMarker>>,

        time: &Time,

        _commands: &mut Commands,
    ) {
        info!("applying global cooldown");
        let now = time.last_update().expect("last cast instant");

        // Remove cast time from last cast
        let now = if let Ok(TotalDuration(duration)) = param.get(parent_id) {
            now - *duration
        } else {
            now
        };
        *last_cast = LastCastInstant(Some(now));
    }
}
