use bevy::prelude::*;

use super::Effect;
use crate::character::Health;

#[derive(Default, Debug, Clone, Component)]
pub struct Damage(pub f32);

impl<'a> Effect<'a> for Damage {
    type Domain = &'a mut Health;

    fn apply(&self, mut item: Mut<'_, Health>) {
        item.apply_damage(self.0);
    }
}
