mod casting;
mod classes;
mod control;
mod cooldowns;
mod health;
mod index;
mod interupt;
mod materials;
mod player;
mod power;
mod speed_multiplier;
mod target;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    input_mapping::{FocalHold, MotionKey, SelectClick},
    physics::Velocity,
    ui::Selected,
};

pub use casting::*;
pub use classes::*;
pub use control::*;
pub use cooldowns::*;
pub use health::*;
pub use index::*;
pub use interupt::*;
pub use materials::*;
pub use player::*;
pub use power::*;
pub use speed_multiplier::*;
pub use target::*;

pub struct CharacterMarker;

// TODO: Stratify into base vs full (base only including that which should be reconcilled over the internet)
#[derive(Bundle)]
pub struct CharacterBundle {
    index: CharacterIndex,
    marker: CharacterMarker,
    class: CharacterClass,

    speed_modifier: CharacterSpeedMultiplier,
    velocity: Velocity,

    #[bundle]
    sprite: SpriteBundle,

    health: CharacterHealth,
    power: CharacterPower,

    cast_state: CharacterCastState,
    interupt_state: InteruptState,
    last_cast_instant: LastCastInstant,

    target: CharacterTarget,
    selected: Selected,
}

impl CharacterBundle {
    pub fn new(
        index: CharacterIndex,
        class: CharacterClass,
        time: &Time,
        materials: &CharacterMaterials,
    ) -> Self {
        let material = materials.handle(class).clone();
        Self {
            index,
            marker: CharacterMarker,
            class,

            speed_modifier: CharacterSpeedMultiplier::from(1.),
            velocity: Velocity::from(Vec2::ZERO),

            sprite: SpriteBundle {
                material,
                sprite: Sprite::new(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },

            power: CharacterPower {
                current: 0,
                total: 100,
            },
            health: CharacterHealth {
                current: 75,
                total: 100,
            },

            cast_state: CharacterCastState::default(),
            interupt_state: InteruptState::default(),
            last_cast_instant: time.startup().into(),

            target: CharacterTarget::default(),
            selected: Selected::default(),
        }
    }
}

pub struct CharacterPlugins;

impl PluginGroup for CharacterPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(CharacterMaterialPlugins)
            .add(PlayerPlugin)
            .add(CastingPlugin);
    }
}
