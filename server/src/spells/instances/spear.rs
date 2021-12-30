use bevy::prelude::*;

use common::{
    effects::{DamageEffect, EffectMarker},
    spells::instances::{CommonSpearBundle, SpearEffect, ToEffect},
};

#[derive(Bundle)]
pub struct SpearBundle {
    #[bundle]
    pub common: CommonSpearBundle,
    pub transform: Transform,
    global_transform: GlobalTransform,
}

impl From<SpearBundle> for SpearEffect {
    fn from(bundle: SpearBundle) -> Self {
        Self {
            marker: EffectMarker,
            target: bundle.common.target.into(),
            damage: DamageEffect { amount: 30 },
        }
    }
}

impl SpearBundle {
    pub fn new(common: CommonSpearBundle, transform: Transform) -> Self {
        Self {
            common,
            transform,
            global_transform: GlobalTransform::default(),
        }
    }
}

impl ToEffect for SpearBundle {
    type Effect = SpearEffect;
}
