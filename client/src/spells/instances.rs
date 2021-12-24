use bevy::prelude::*;

use common::spells::instances::{FireballBundle as CommonFireballBundle, FIREBALL_SIZE};

use super::SpellMaterials;

#[derive(Bundle)]
pub struct FireballBundle {
    #[bundle]
    common: CommonFireballBundle,
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
