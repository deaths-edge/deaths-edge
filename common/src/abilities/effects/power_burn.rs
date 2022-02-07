use bevy::prelude::*;

use super::CharacterEffect;
use crate::character::Power;

#[derive(Default, Debug, Clone, Component)]
pub struct PowerBurn(pub f32);

impl CharacterEffect for PowerBurn {
    type Domain<'a> = &'a mut Power;

    type Param<'w, 's> = ();
    type Fetch = ();

    fn apply(
        &self,
        _parent_id: Entity,

        mut item: Mut<'_, Power>,
        _param: &(),

        _time: &Time,

        _commands: &mut Commands,
    ) {
        info!(message = "burning power", amount = %self.0);
        item.subtract(self.0);
    }
}
