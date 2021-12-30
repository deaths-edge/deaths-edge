use bevy::prelude::*;

use common::{
    character::Buff,
    effects::{BuffEffect, DamageEffect, EffectMarker},
    spells::instances::{CommonSpearBundle, SpearEffect, ToEffect},
};

#[derive(Bundle)]
pub struct SpearBundle {
    #[bundle]
    pub common: CommonSpearBundle,
    pub transform: Transform,
    global_transform: GlobalTransform,
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

    fn to_effect(self, world: &World) -> Self::Effect {
        SpearEffect {
            marker: EffectMarker,
            target: self.common.target.into(),
            damage: DamageEffect { amount: 30. },
            debuff: BuffEffect {
                buff: Buff::Speared {
                    start: world
                        .get_resource::<Time>()
                        .expect("failed to get time")
                        .last_update()
                        .expect("failed to get last update"),
                },
            },
        }
    }
}
