use bevy::prelude::*;

use crate::{
    abilities::{lifecycle::CastDuration, AbilityId, AbilityMarker},
    character::LastCastInstant,
};

use super::CharacterEffect;

#[derive(Default, Debug, Clone, Component)]
pub struct TriggerGlobalCooldown;

impl CharacterEffect for TriggerGlobalCooldown {
    type Domain<'a> = &'a mut LastCastInstant;

    type Param<'w, 's> = Query<'w, 's, &'static CastDuration, With<AbilityMarker>>;
    type Fetch = QueryState<&'static CastDuration, With<AbilityMarker>>;

    fn apply(
        &self,
        time: &Time,
        AbilityId(ability_id): &AbilityId,
        mut last_cast: Mut<'_, LastCastInstant>,
        param: &Query<&CastDuration, With<AbilityMarker>>,
        _commands: &mut Commands,
    ) {
        info!("applying global cooldown");
        let now = time.last_update().expect("last cast instant");

        // Remove cast time from last cast
        let now = if let Ok(CastDuration(duration)) = param.get(*ability_id) {
            now - *duration
        } else {
            now
        };
        *last_cast = LastCastInstant(Some(now));
    }
}
