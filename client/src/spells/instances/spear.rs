use bevy::prelude::*;

use common::{
    effects::{DamageEffect, EffectMarker},
    spells::instances::{CommonSpearBundle, SpearEffect, ToEffect, FIREBALL_SIZE},
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
                sprite: Sprite::new(Vec2::new(FIREBALL_SIZE, FIREBALL_SIZE)),
                material: materials.fireball_material.clone(),
                transform,
                ..Default::default()
            },
        }
    }
}

impl From<SpearBundle> for SpearEffect {
    fn from(bundle: SpearBundle) -> Self {
        Self {
            marker: EffectMarker,
            target: bundle.common.target().into(),
            damage: DamageEffect { amount: 30 },
        }
    }
}

impl ToEffect for SpearBundle {
    type Effect = SpearEffect;
}
