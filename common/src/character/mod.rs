mod abilities;
mod actions;
mod cast_id;
mod classes;
mod control;
mod cooldowns;
mod health;
mod index;
mod interrupt;
mod power;
mod speed_multiplier;
mod team;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;
use heron::prelude::*;

pub use abilities::*;
pub use actions::*;
pub use cast_id::*;
pub use classes::*;
pub use control::*;
pub use cooldowns::*;
pub use health::*;
pub use index::*;
pub use interrupt::*;
pub use power::*;
pub use speed_multiplier::*;
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
    abilities: Abilities,

    last_cast_instant: LastCastInstant,
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
        let regenerate = SystemSet::on_update(self.state)
            .with_system(regenerate_health)
            .with_system(regenerate_power);
        app.add_system_set(regenerate)
            .add_plugin(CharacterEntityActionPlugin::new(self.state))
            .add_plugin(InterruptedPlugin { state: self.state });
    }
}
