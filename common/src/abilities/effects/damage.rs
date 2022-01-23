use bevy::prelude::*;

use super::Effect;
use crate::character::Health;

#[derive(Default, Debug, Clone, Component)]
pub struct Damage(pub f32);

impl Effect for Damage {
    type Domain<'a> = &'a mut Health;

    type Param<'w, 's> = ();
    type Fetch = ();

    fn apply(&self, _time: &Time, mut item: Mut<'_, Health>, _param: &(), commands: &mut Commands) {
        item.apply_damage(self.0);
    }
}
