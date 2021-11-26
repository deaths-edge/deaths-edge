use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    input_mapping::{FocalHold, MotionKey, SelectClick},
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
        app.add_startup_system(spawn_player.system())
            .add_startup_system(spawn_char_1.system())
            .add_system_set(system_set);
    }
}

pub struct Player;

#[derive(Default, Clone, Copy)]
pub struct CharacterIndex(usize);

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

#[derive(Default, Clone, Copy)]
pub struct CharacterTarget {
    target: Option<usize>,
}

impl CharacterTarget {
    pub fn set_target(&mut self, character: CharacterIndex) -> &mut Self {
        self.target = Some(character.0);
        self
    }

    pub fn deselect(&mut self) -> &mut Self {
        self.target = None;
        self
    }
}

pub enum CharacterControl {
    Stun,
    Dazed,
    Normal,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    character: CharacterIndex,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
    speed_modifier: CharacterSpeedMultiplier,
    health: CharacterHealth,
    selection: CharacterTarget,
}

pub fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let character = CharacterBundle {
        character: CharacterIndex(0),
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
        character: CharacterIndex(1),
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
    mut select_clicks: EventReader<SelectClick>,
    mut char_query: QuerySet<(
        Query<(&Player, &mut CharacterTarget)>,
        Query<(&CharacterIndex, &Transform, &Sprite)>,
    )>,
) {
    const SELECT_SIZE: (f32, f32) = (30., 30.);

    if let Some(select_click) = select_clicks.iter().last() {
        let selected_index = char_query
            .q1()
            .iter()
            .find(|(_, char_transform, char_sprite)| {
                collide(
                    select_click.mouse_position.extend(0.),
                    SELECT_SIZE.into(),
                    char_transform.translation,
                    char_sprite.size,
                )
                .is_some()
            })
            .map(|(index, _, _)| index);

        if let Some(selected_index) = selected_index.cloned() {
            let (_, mut character_target) = char_query
                .q0_mut()
                .single_mut()
                .expect("failed to find player");
            character_target.set_target(selected_index);
        }
    }
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

    // CharacterIndex query
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
