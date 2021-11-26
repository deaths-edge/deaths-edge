mod control;
mod index;
mod speed_multiplier;
mod target;
mod health;

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
            .add_system_set(system_set)
            .add_system(player_char_select.system());
    }
}

pub struct Player;

#[derive(Bundle)]
pub struct CharacterBundle {
    character: CharacterIndex,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
    speed_modifier: CharacterSpeedMultiplier,
    health: CharacterHealth,
    target: CharacterTarget,
    selected: Selected,
}

pub fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let character = CharacterBundle {
        character: CharacterIndex::from(0),
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
    commands.spawn_bundle(character).insert(Player);
}

pub fn spawn_char_1(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let character = CharacterBundle {
        character: CharacterIndex::from(1),
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

pub fn player_char_select(
    mut select_clicks: EventReader<SelectClick>,
    mut char_query: QuerySet<(
        Query<(&CharacterIndex, &mut Selected)>,
        Query<(&CharacterIndex, &Transform, &Sprite, &mut Selected)>,
        Query<(&Player, &mut CharacterTarget), Changed<CharacterTarget>>,
    )>,
) {
    const SELECT_SIZE: (f32, f32) = (30., 30.);

    // Get selected click
    let select_click = if let Some(some) = select_clicks.iter().last() {
        some
    } else {
        return;
    };

    // Find selected index
    let selected_index_opt = char_query
        .q1_mut()
        .iter_mut()
        .find(|(_, char_transform, char_sprite, _)| {
            collide(
                select_click.mouse_position.extend(0.),
                SELECT_SIZE.into(),
                char_transform.translation,
                char_sprite.size,
            )
            .is_some()
        })
        .map(|(index, _, _, selected)| (index, selected));

    let selected_index_copy = selected_index_opt
        .map(|(index, mut selected)| {
            // Set selection
            *selected = Selected::Selected;

            *index
        })
        .map(|index| {
            // Set character selection
            if let Ok((_, mut character_target)) = char_query.q2_mut().single_mut() {
                trace!(message = "selected character", ?index);

                character_target.set_index(index);
            };
            index
        });

    // Deselect everything else
    for (_, mut selected) in char_query
        .q0_mut()
        .iter_mut()
        .filter(|(index, _)| Some(**index) != selected_index_copy)
    {
        *selected = Selected::Unselected;
    }
}

pub fn player_focal_rotate(
    mut char_query: Query<&mut Transform, With<Player>>,
    mut events: EventReader<FocalHold>,
) {
    let mut transform = char_query.single_mut().expect("player not found");

    const MINIMUM_FOCAL_LENGTH: f32 = 200.;

    if let Some(event) = events.iter().last() {
        let translation = Vec2::from(transform.translation);

        let diff = event.mouse_position - translation;
        let distance = diff.length();
        let adjustment = distance.min(MINIMUM_FOCAL_LENGTH);
        let new_diff = diff * adjustment / distance;
        // let new_diff = diff;

        let angle = Vec2::new(0., 1.).angle_between(new_diff);
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

    const FORWARD_SPEED: f32 = 1.0;
    const STRAFE_SPEED: f32 = 0.8;
    const BACKPEDDLE_SPEED: f32 = 0.6;

    // Construct direction
    let mut direction = Vec2::ZERO;
    if motion_input.pressed(MotionKey::Left) {
        direction.x -= STRAFE_SPEED;
    }

    if motion_input.pressed(MotionKey::Forward) {
        direction.y += FORWARD_SPEED;
    }

    if motion_input.pressed(MotionKey::Right) {
        direction.x += STRAFE_SPEED;
    }

    if motion_input.pressed(MotionKey::Backward) {
        direction.y -= BACKPEDDLE_SPEED;
    }

    // Normalize
    if direction != Vec2::ZERO {
        let mag = direction.length().max(1.);
        direction = direction / mag;
    }

    direction = (transform.rotation * (direction.extend(0.))).truncate();

    // Assign velocity
    **velocity = direction * speed_multiplier.speed();
}
