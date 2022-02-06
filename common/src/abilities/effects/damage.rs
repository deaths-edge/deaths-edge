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
        _parent_id: Entity,

        mut item: Mut<'_, Health>,
        _param: &(),

        _time: &Time,

        _commands: &mut Commands,
    ) {
        item.apply_damage(self.0);
    }
}
