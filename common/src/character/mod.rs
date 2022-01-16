mod actions;
mod casting;
mod classes;
mod control;
mod cooldowns;
mod health;
mod index;
mod interrupt;
mod power;
mod speed_multiplier;
mod target;
mod team;

use std::{fmt::Debug, hash::Hash, time::Instant};

use bevy::prelude::*;
use heron::prelude::*;

use crate::physics::WorldLayer;

pub use actions::*;
pub use casting::*;
pub use classes::*;
pub use control::*;
pub use cooldowns::*;
pub use health::*;
pub use index::*;
pub use interrupt::*;
pub use power::*;
pub use speed_multiplier::*;
pub use target::*;
pub use team::*;

#[derive(Debug, Default, Component)]
pub struct CharacterMarker;

// TODO: Stratify into base vs full (base only including that which should be reconciled over the internet)
#[derive(Bundle)]
pub struct CharacterBundle {
    index: CharacterIndex,
    marker: CharacterMarker,
    class: Class,
    team: Team,

    // Physics
    transform: Transform,
    global_transform: GlobalTransform,
    speed_modifier: SpeedMultiplier,
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    velocity: Velocity,
    rotational_constraints: RotationConstraints,
    controls: Controls,

    // Resources
    health: Health,
    power: Power,

    // Casting
    cast_state: CastState,
    last_cast_instant: LastCastInstant,

    target: OptionalTarget,
}

impl CharacterBundle {
    pub fn new(
        index: CharacterIndex,
        transform: Transform,
        class: Class,
        team: Team,
        last_cast_instant: Instant,
    ) -> Self {
        let size = class.size();
        let health = class.health();
        let power = class.power();

        Self {
            index,
            marker: CharacterMarker,
            class,
            team,

            transform,
            global_transform: GlobalTransform::default(),
            speed_modifier: SpeedMultiplier(1.),
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
            controls: Controls::default(),

            power,
            health: Health {
                current: health,
                total: health,
            },

            cast_state: CastState::default(),
            last_cast_instant: LastCastInstant(last_cast_instant),

            target: OptionalTarget::default(),
        }
    }

    pub fn class(&self) -> Class {
        self.class
    }
}

pub struct CharacterPlugin<T> {
    pub state: T,
}

impl<T> Plugin for CharacterPlugin<T>
where
    T: Send + Sync + 'static,
    T: Clone + Copy + Eq + Hash + Debug,
{
    fn build(&self, app: &mut App) {
        let regenerate = SystemSet::on_update(self.state).with_system(regenerate_power);
        app.add_system_set(regenerate)
            .add_plugin(CharacterEntityActionPlugin::new(self.state));
    }
}
