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

impl From<FireballBundle> for FireballEffect {
    fn from(bundle: FireballBundle) -> Self {
        Self {
            marker: EffectMarker,
            target: bundle.common.target().into(),
            damage: DamageEffect { amount: 30 },
        }
    }
}

impl ToEffect for FireballBundle {
    type Effect = FireballEffect;
}
