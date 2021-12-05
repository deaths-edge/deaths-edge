use bevy::prelude::*;

use crate::{
    effects::{DamageEffect, EffectMarker, EffectTarget},
    spells::*,
};

use super::SpellMaterials;

#[derive(Bundle)]
pub struct FireballBundle {
    spell_marker: SpellMarker,
    projectile_marker: SpellProjectileMarker,

    #[bundle]
    sprite: SpriteBundle,

    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    velocity: Velocity,

    source: SpellSource,
    target: SpellTarget,
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
            spell_marker: SpellMarker::Fireball,
            projectile_marker: SpellProjectileMarker,

            sprite: SpriteBundle {
                material: materials.fireball_material.clone(),
                sprite: Sprite::new(Vec2::new(FIREBALL_SIZE, FIREBALL_SIZE)),
                transform,
                ..Default::default()
            },

            source,
            target,

            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(FIREBALL_SIZE / 2., FIREBALL_SIZE / 2., 0.),
                border_radius: None,
            },
            velocity: Velocity::from(Vec2::new(0., speed_multiplier * FIREBALL_SPEED)),
        }
    }

    pub fn target(&self) -> SpellTarget {
        self.target
    }
}

#[derive(Bundle)]
pub struct FireballEffect {
    pub marker: EffectMarker,
    pub target: EffectTarget,
    pub damage: DamageEffect,
}
