use bevy::prelude::*;
use thiserror::Error;

use crate::{
    character::{CharacterCast, CharacterCastState, CharacterTarget},
    effects::{DamageEffect, EffectMarker, EffectTarget},
    spells::*,
};

#[derive(Error, Debug)]
pub enum SpearActionError {
    #[error("global cooldown")]
    GlobalCooldown,
    #[error("no target")]
    NoTarget,
    #[error("out of field of view: {0}")]
    OutOfFieldOfView(f32),
    #[error("out of line-of-sight")]
    LineOfSightObstruction,
}

impl From<GlobalCooldown> for SpearActionError {
    fn from(_: GlobalCooldown) -> Self {
        Self::GlobalCooldown
    }
}

impl From<LineOfSightObstruction> for SpearActionError {
    fn from(_: LineOfSightObstruction) -> Self {
        Self::LineOfSightObstruction
    }
}

impl From<OutOfFieldOfView> for SpearActionError {
    fn from(value: OutOfFieldOfView) -> Self {
        Self::OutOfFieldOfView(value.0)
    }
}

pub fn spear_action(
    time: &Time,
    physics_world: &PhysicsWorld,

    last_cast_instant: &LastCastInstant,

    character_entity: Entity,
    character_transform: &Transform,
    character_target: &CharacterTarget,
    character_cast_state: &mut CharacterCastState,

    target_query: &Query<&Transform, With<CharacterMarker>>,
) -> Result<(), SpearActionError> {
    check_global_cooldown(time, last_cast_instant)?;

    let start = time.last_update().expect("last update not found");

    let target_entity = if let Some(some) = character_target.id() {
        some
    } else {
        return Err(SpearActionError::NoTarget);
    };

    let target_transform = target_query.get(target_entity).expect("target not found");

    check_line_of_sight(
        character_transform,
        target_transform.translation,
        physics_world,
    )?;

    check_in_front(character_transform, target_transform.translation)?;

    let spell = Spell::Spear {
        source: SpellSource(character_entity),
        target: SpellTarget(target_entity),
    };
    tracing::info!(message = "casting", ?spell, ?start);
    character_cast_state.set_cast(CharacterCast::new(start, spell));

    Ok(())
}

#[derive(Bundle)]
pub struct CommonSpearBundle {
    spell_marker: SpellMarker,
    projectile_marker: SpellProjectileMarker,

    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    velocity: Velocity,

    pub source: SpellSource,
    pub target: SpellTarget,
    // TODO: Maybe include effect here?
}

pub const SPEAR_SPEED: f32 = 300.;
pub const SPEAR_SIZE: (f32, f32) = (30., 5.);

impl CommonSpearBundle {
    pub fn new(source: SpellSource, target: SpellTarget, speed_multiplier: f32) -> Self {
        Self {
            spell_marker: SpellMarker::Spear,
            projectile_marker: SpellProjectileMarker,

            source,
            target,

            rigid_body: RigidBody::Sensor,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(SPEAR_SIZE.0, SPEAR_SIZE.0, 0.),
                border_radius: None,
            },
            velocity: Velocity::from(Vec2::new(0., speed_multiplier * SPEAR_SPEED)),
        }
    }
}

#[derive(Bundle)]
pub struct SpearEffect {
    pub marker: EffectMarker,
    pub target: EffectTarget,
    pub damage: DamageEffect,
}
