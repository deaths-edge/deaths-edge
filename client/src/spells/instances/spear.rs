use bevy::prelude::*;

use common::{
    character::Buff,
    effects::{BuffEffect, DamageEffect, EffectMarker},
    spells::instances::{CommonSpearBundle, SpearEffect, ToEffect, SPEAR_SIZE},
};

use crate::spells::SpellMaterials;

#[derive(Bundle)]
pub struct SpearBundle {
    #[bundle]
    pub common: CommonSpearBundle,
    #[bundle]
    sprite: SpriteBundle,
}

impl SpearBundle {
    pub fn new(
        common: CommonSpearBundle,
        transform: Transform,
        materials: &SpellMaterials,
    ) -> Self {
        Self {
            common,
            sprite: SpriteBundle {
                sprite: Sprite::new(Vec2::new(SPEAR_SIZE.0, SPEAR_SIZE.1)),
                material: materials.fireball_material.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}

impl ToEffect for SpearBundle {
    type Effect = SpearEffect;

    fn to_effect(self, world: &World) -> Self::Effect {
        SpearEffect {
            marker: EffectMarker,
            target: self.common.target.into(),
            damage: DamageEffect { amount: 15. },
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
