use bevy::prelude::*;

use crate::{
    effects::{DamageEffect, EffectMarker, EffectTarget},
    spells::*,
};

use super::SpellMaterials;

#[derive(Bundle)]
pub struct FireballBundle {
    marker: SpellMarker,
    #[bundle]
    sprite: SpriteBundle,
    source: SpellSource,
    pub target: SpellTarget,
    tracking: SpellProjectileMarker,
    velocity: Velocity,
    // TODO: Maybe include effect here?
}

impl FireballBundle {
    pub fn new(
        transform: Transform,
        source: SpellSource,
        target: SpellTarget,
        speed_multiplier: f32,
        materials: &SpellMaterials,
    ) -> Self {
        const FIREBALL_SPEED: f32 = 300.;
        const FIREBALL_SIZE: f32 = 15.;

        Self {
            marker: SpellMarker::Fireball,
            sprite: SpriteBundle {
                material: materials.fireball_material.clone(),
                sprite: Sprite::new(Vec2::new(FIREBALL_SIZE, FIREBALL_SIZE)),
                transform,
                ..Default::default()
            },
            source,
            target,
            tracking: SpellProjectileMarker,
            velocity: Velocity::from(Vec2::new(0., speed_multiplier * FIREBALL_SPEED)),
        }
    }
}

#[derive(Bundle)]
pub struct FireballEffect {
    pub marker: EffectMarker,
    pub target: EffectTarget,
    pub damage: DamageEffect,
}
