use bevy::prelude::*;

use common::{
    effects::{DamageEffect, EffectMarker},
    spells::instances::{CommonFireballBundle, FireballEffect, ToEffect, FIREBALL_SIZE},
};

use crate::spells::SpellMaterials;

#[derive(Bundle)]
pub struct FireballBundle {
    #[bundle]
    pub common: CommonFireballBundle,
    #[bundle]
    sprite: SpriteBundle,
}

impl FireballBundle {
    pub fn new(
        common: CommonFireballBundle,
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
