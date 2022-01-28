use bevy::prelude::*;

use super::CharacterEffect;
use crate::{abilities::AbilityId, character::Health};

#[derive(Default, Debug, Clone, Component)]
pub struct Dot(pub f32);

impl CharacterEffect for Dot {
    type Domain<'a> = &'a mut Health;

    type Param<'w, 's> = ();
    type Fetch = ();

    fn apply(
        &self,
        time: &Time,
        _ability_id: &AbilityId,
        mut item: Mut<'_, Health>,
        _param: &(),
        _commands: &mut Commands,
    ) {
        let delta = time.delta();
        item.apply_damage(self.0 * delta);
    }
}
