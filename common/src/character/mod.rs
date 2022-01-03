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

use std::{fmt::Debug, hash::Hash};

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

pub struct CharacterMarker;

// TODO: Stratify into base vs full (base only including that which should be reconciled over the internet)
#[derive(Bundle)]
pub struct CharacterBundle {
    index: CharacterIndex,
    marker: CharacterMarker,
    class: Class,
    team: Team,

    // Physics
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
    interrupts: Interrupts,
    last_cast_instant: LastCastInstant,

    target: Target,
}

impl CharacterBundle {
    pub fn new(index: CharacterIndex, class: Class, time: &Time) -> Self {
        let size = class.size();
        let health = class.health();
        Self {
            index,
            marker: CharacterMarker,
            class,
            team: Team::Blue,

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

            power: Power {
                current: 0.,
                total: 100.,
            },
            health: Health {
                current: health,
                total: health,
            },

            cast_state: CastState::default(),
            interrupts: Interrupts::default(),
            last_cast_instant: LastCastInstant(time.startup()),

            target: Target::default(),
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
    fn build(&self, app: &mut AppBuilder) {
        let regenerate = SystemSet::on_update(self.state).with_system(regenerate_power.system());
        app.add_system_set(regenerate)
            // .add_plugin(CastingPlugin::new(self.state))
            .add_plugin(CharacterEntityActionPlugin::new(self.state));
    }
}
