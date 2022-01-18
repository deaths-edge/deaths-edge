use bevy::prelude::*;

use super::Effect;
use crate::character::Power;

#[derive(Default, Debug, Clone, Component)]
pub struct PowerBurn(pub f32);

impl<'a> Effect<'a> for PowerBurn {
    type Domain = &'a mut Power;

    fn apply(&self, mut item: Mut<'_, Power>) {
        info!(message = "burning power", amount = %self.0);
        item.subtract(self.0);
    }
}
