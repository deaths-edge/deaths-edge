mod casting;
mod classes;
mod commands;
mod control;
mod cooldowns;
mod health;
mod index;
mod interrupt;
mod power;
mod speed_multiplier;
mod target;

use bevy::prelude::*;
use heron::prelude::*;

use crate::physics::WorldLayer;

pub use casting::*;
pub use classes::*;
pub use commands::*;
pub use control::*;
pub use cooldowns::*;
pub use health::*;
pub use index::*;
pub use interrupt::*;
pub use power::*;
pub use speed_multiplier::*;
pub use target::*;

pub struct CharacterMarker;

// TODO: Stratify into base vs full (base only including that which should be reconciled over the internet)
#[derive(Bundle)]
pub struct CharacterBundle {
    index: CharacterIndex,
    marker: CharacterMarker,
    class: CharacterClass,

    // Physics
    speed_modifier: CharacterSpeedMultiplier,
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    velocity: Velocity,
    rotational_constraints: RotationConstraints,

    health: CharacterHealth,
    power: CharacterPower,

    cast_state: CharacterCastState,
    interrupt_state: InterruptState,
    last_cast_instant: LastCastInstant,

    target: CharacterTarget,
}

impl CharacterBundle {
    pub fn new(index: CharacterIndex, class: CharacterClass, time: &Time) -> Self {
        let size = class.size();
        Self {
            index,
            marker: CharacterMarker,
            class,

            speed_modifier: CharacterSpeedMultiplier::from(1.),
            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec2::new(size.width / 2., size.height / 2.).extend(0.),
                border_radius: None,
            },
            collision_layers: CollisionLayers::none()
                .with_group(WorldLayer::Character)
                .with_mask(WorldLayer::Environment),
            velocity: Vec3::ZERO.into(),
            rotational_constraints: RotationConstraints::lock(),

            power: CharacterPower {
                current: 0,
                total: 100,
            },
            health: CharacterHealth {
                current: 75,
                total: 100,
            },

            cast_state: CharacterCastState::default(),
            interrupt_state: InterruptState::default(),
            last_cast_instant: time.startup().into(),

            target: CharacterTarget::default(),
        }
    }

    pub fn class(&self) -> CharacterClass {
        self.class
    }
}
