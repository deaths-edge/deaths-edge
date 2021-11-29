mod casting;
mod classes;
mod control;
mod cooldowns;
mod health;
mod index;
mod interupt;
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
pub use player::*;
pub use power::*;
pub use speed_multiplier::*;
pub use target::*;

pub struct CharacterPlugins;

impl PluginGroup for CharacterPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(PlayerPlugin).add(SpawnPlugin).add(CastingPlugin);
    }
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system())
            .add_startup_system(spawn_char_1.system());
    }
}

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
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            index,
            marker: CharacterMarker,
            class,

            speed_modifier: CharacterSpeedMultiplier::from(1.),
            velocity: Velocity::from(Vec2::ZERO),

            sprite: SpriteBundle {
                material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
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

pub fn spawn_char_1(
    time: Res<Time>,
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let character_bundle = CharacterBundle::new(
        CharacterIndex::from(1),
        CharacterClass::Heka,
        &time,
        materials,
    );
    commands.spawn_bundle(character_bundle);
}
