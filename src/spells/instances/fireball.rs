use bevy::prelude::*;

use super::*;

#[derive(Bundle)]
pub struct FireballBundle {
    marker: SpellMarker,
    #[bundle]
    sprite: SpriteBundle,
    source: SpellSource,
    target: SpellTarget,
    tracking: SpellProjectileMarker,
    velocity: Velocity,
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

pub struct FireballImpactBundle {}
