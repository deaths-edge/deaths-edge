use bevy::prelude::*;

use super::CharacterEffect;
use crate::character::Health;

#[derive(Default, Debug, Clone, Component)]
pub struct Dot(pub f32);

impl CharacterEffect for Dot {
    type Domain<'a> = &'a mut Health;

    type Param<'w, 's> = ();
    type Fetch = ();

    fn apply(
        &self,
        _parent_id: Entity,

        mut item: Mut<'_, Health>,
        _param: &(),

        time: &Time,

        _commands: &mut Commands,
    ) {
        let delta = time.delta().as_secs_f32();
        item.apply_damage(self.0 * delta);
    }
}
