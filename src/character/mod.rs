mod control;
mod index;
mod speed_multiplier;
mod target;
mod health;
mod player;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    input_mapping::{FocalHold, MotionKey, SelectClick},
    physics::Velocity,
    ui::Selected,
};

pub use control::*;
pub use index::*;
pub use speed_multiplier::*;
pub use target::*;
pub use health::*;
pub use player::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::new()
            .label("character-motion")
            .before("collisions")
            .with_system(player_focal_rotate.system())
            .with_system(player_movement.system());
        app.add_startup_system(spawn_player.system())
            .add_startup_system(spawn_char_1.system())
            .add_system_set(system_set)
            .add_system(player_char_select.system());
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
