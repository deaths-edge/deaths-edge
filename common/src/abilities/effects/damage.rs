use bevy::prelude::*;

use super::CharacterEffect;
use crate::character::Health;

#[derive(Default, Debug, Clone, Component)]
pub struct Damage(pub f32);

impl CharacterEffect for Damage {
    type Domain<'a> = &'a mut Health;

    type Param<'w, 's> = ();
    type Fetch = ();

    fn apply(
        &self,
        _time: &Time,
        mut item: Mut<'_, Health>,
        _param: &(),
        _commands: &mut Commands,
    ) {
        item.apply_damage(self.0);
    }
}
