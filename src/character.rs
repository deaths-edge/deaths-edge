use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::{
    input_mapping::{FocalHold, MotionKey},
    physics::Velocity,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::new()
            .label("character-motion")
            .before("collisions")
            .with_system(player_focal_rotate.system())
            .with_system(player_movement.system());
        app.add_system_set(system_set);
    }
}

pub struct Player;

pub struct Character(usize);

pub struct CharacterSpeedMultiplier(f32);

impl From<f32> for CharacterSpeedMultiplier {
    fn from(val: f32) -> Self {
        Self(val)
    }
}

impl Deref for CharacterSpeedMultiplier {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharacterSpeedMultiplier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CharacterSpeedMultiplier {
    const BASE_SPEED: f32 = 300.;

    pub fn speed(&self) -> f32 {
        Self::BASE_SPEED * self.0
    }
}

pub struct CharacterHealth {
    current: u32,
    total: u32,
}

#[derive(Default)]
pub struct CharacterTarget {
    target: Option<usize>,
}

pub enum CharacterControl {
    Stun,
    Dazed,
    Normal,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    character: Character,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
    speed_modifier: CharacterSpeedMultiplier,
    health: CharacterHealth,
    selection: CharacterTarget,
}

pub fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let character = CharacterBundle {
        character: Character(0),
        velocity: Velocity::from(Vec2::ZERO),
        sprite: SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        },
        speed_modifier: CharacterSpeedMultiplier(1.),
        health: CharacterHealth {
            current: 75,
            total: 100,
        },
        selection: CharacterTarget::default(),
    };
    commands.spawn_bundle(character).insert(Player);
}

pub fn spawn_char_1(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let character = CharacterBundle {
        character: Character(1),
        velocity: Velocity::from(Vec2::ZERO),
        sprite: SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        },
        speed_modifier: CharacterSpeedMultiplier(1.),
        health: CharacterHealth {
            current: 75,
            total: 100,
        },
        selection: CharacterTarget::default(),
    };
    commands.spawn_bundle(character);
}

pub fn player_char_select(
    mouse_position: Res<WorldMousePosition>,
    mouse_click_events: Res<Events<MouseButtonInput>>,
    mut char_query: QuerySet<(
        Query<(&Player, &mut CharacterTarget)>,
        Query<(&Character, &Transform)>,
    )>,
) {
    let (player, selection) = char_query
        .q0_mut()
        .single_mut()
        .expect("failed to find player");
}

pub fn player_focal_rotate(
    mut char_query: Query<(&mut Transform, With<Player>)>,
    mut events: EventReader<FocalHold>,
) {
    let (mut transform, _) = char_query.single_mut().expect("player not found");

    if let Some(event) = events.iter().last() {
        let translation = Vec2::from(transform.translation);

        let angle = -(event.mouse_position - translation).angle_between(Vec2::new(0., 1.));
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn player_movement(
    motion_input: Res<Input<MotionKey>>,

    // Character query
    mut char_query: Query<(
        &CharacterSpeedMultiplier,
        &mut Transform,
        &mut Velocity,
        With<Player>,
    )>,
) {
    let (speed_multiplier, transform, mut velocity, _) =
        char_query.single_mut().expect("player not found");

    // Construct direction
    let mut direction = Vec3::default();
    if motion_input.pressed(MotionKey::Left) {
        direction.x -= 1.;
    }

    if motion_input.pressed(MotionKey::Forward) {
        direction.y += 1.;
    }

    if motion_input.pressed(MotionKey::Right) {
        direction.x += 1.;
    }

    if motion_input.pressed(MotionKey::Backward) {
        direction.y -= 1.;
    }

    // Rotate
    direction = transform.rotation * direction;

    // Assign velocity
    **velocity = (direction * speed_multiplier.speed()).truncate();
}
