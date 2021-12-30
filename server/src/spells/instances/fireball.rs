use bevy::prelude::*;

use common::{
    effects::{DamageEffect, EffectMarker},
    spells::instances::{CommonFireballBundle, FireballEffect, ToEffect},
};

#[derive(Bundle)]
pub struct FireballBundle {
    #[bundle]
    pub common: CommonFireballBundle,
    pub transform: Transform,
    global_transform: GlobalTransform,
}

impl FireballBundle {
    pub fn new(common: CommonFireballBundle, transform: Transform) -> Self {
        Self {
            common,
            transform,
            global_transform: GlobalTransform::default(),
        }
    }
}

impl ToEffect for FireballBundle {
    type Effect = FireballEffect;

    fn to_effect(self, world: &World) -> Self::Effect {
        FireballEffect {
            marker: EffectMarker,
            target: self.common.target().into(),
            damage: DamageEffect { amount: 30. },
        }
    }
}
