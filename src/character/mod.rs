mod control;
mod health;
mod index;
mod player;
mod speed_multiplier;
mod target;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    input_mapping::{FocalHold, MotionKey, SelectClick},
    physics::Velocity,
    ui::Selected,
};

pub use control::*;
pub use health::*;
pub use index::*;
pub use player::*;
pub use speed_multiplier::*;
pub use target::*;

pub struct CharacterPlugins;

impl PluginGroup for CharacterPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(PlayerPlugin).add(SpawnPlugin);
    }
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system())
            .add_startup_system(spawn_char_1.system());
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    index: CharacterIndex,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
    speed_modifier: CharacterSpeedMultiplier,
    health: CharacterHealth,
    target: CharacterTarget,
    selected: Selected,
    // last_cast:
}

pub fn spawn_char_1(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let character = CharacterBundle {
        index: CharacterIndex::from(1),
        velocity: Velocity::from(Vec2::ZERO),
        sprite: SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        },
        speed_modifier: CharacterSpeedMultiplier::from(1.),
        health: CharacterHealth {
            current: 75,
            total: 100,
        },
        target: CharacterTarget::default(),
        selected: Selected::default(),
    };
    commands.spawn_bundle(character);
}
