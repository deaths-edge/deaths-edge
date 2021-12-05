use bevy::prelude::*;
use thiserror::Error;

use crate::{
    character::{CharacterCast, CharacterCastState, CharacterTarget},
    effects::{DamageEffect, EffectMarker, EffectTarget},
    spells::*,
};

use super::SpellMaterials;

#[derive(Error, Debug)]
pub enum FireballActionError {
    #[error("global cooldown")]
    GlobalCooldown,
    #[error("no target")]
    NoTarget,
    #[error("out of field of view: {0}")]
    OutOfFieldOfView(f32),
    #[error("out of line-of-sight")]
    LineOfSightObstruction,
}

impl From<GlobalCooldown> for FireballActionError {
    fn from(_: GlobalCooldown) -> Self {
        Self::GlobalCooldown
    }
}

impl From<LineOfSightObstruction> for FireballActionError {
    fn from(_: LineOfSightObstruction) -> Self {
        Self::LineOfSightObstruction
    }
}

impl From<OutOfFieldOfView> for FireballActionError {
    fn from(value: OutOfFieldOfView) -> Self {
        Self::OutOfFieldOfView(value.0)
    }
}

pub fn fireball_action(
    time: &Time,
    physics_world: &PhysicsWorld,

    last_cast_instant: &LastCastInstant,

    character_entity: Entity,
    character_transform: &Transform,
    character_target: &CharacterTarget,
    character_cast_state: &mut CharacterCastState,

    target_query: &Query<&Transform, With<CharacterMarker>>,
) -> Result<(), FireballActionError> {
    check_global_cooldown(time, last_cast_instant)?;

    let start = time.last_update().expect("last update not found");

    let target_entity = if let Some(some) = character_target.id() {
        some
    } else {
        return Err(FireballActionError::NoTarget);
    };

    let target_transform = target_query.get(target_entity).expect("target not found");

    check_line_of_sight(
        character_transform,
        target_transform.translation,
        physics_world,
    )?;

    check_in_front(character_transform, target_transform.translation)?;

    let spell = SpellCast::Fireball {
        source: SpellSource::from(character_entity),
        target: target_entity.into(),
    };
    tracing::info!(message = "casting", ?spell, ?start);
    character_cast_state.set_cast(CharacterCast::new(start, spell));

    Ok(())
}

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

            rigid_body: RigidBody::Sensor,
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
