use bevy::prelude::*;

use super::effects::DamageEffect;

use super::*;

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
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        const FIREBALL_SPEED: f32 = 300.;

        Self {
            marker: SpellMarker::Fireball,
            sprite: SpriteBundle {
                material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
                sprite: Sprite::new(Vec2::new(15.0, 15.0)),
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
