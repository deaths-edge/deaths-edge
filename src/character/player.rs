use bevy::prelude::*;

use super::*;
use crate::{input_mapping::ActionKey, physics::Velocity, spell::Spell};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let character_motion = SystemSet::new()
            .label("character-motion")
            .before("collisions")
            .with_system(player_focal_rotate.system())
            .with_system(player_movement.system());
        let character_actions = SystemSet::new()
            .label("character-actions")
            .with_system(player_action.system())
            .with_system(player_char_select.system());
        app.add_system_set(character_motion)
            .add_system_set(character_actions);
    }
}

pub struct PlayerMarker;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: PlayerMarker,
    #[bundle]
    character_bundle: CharacterBundle,
}

pub fn spawn_player(
    time: Res<Time>,
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let character_bundle = CharacterBundle::new(
        CharacterIndex::from(0),
        CharacterClass::Medea,
        &time,
        materials,
    );
    let player_bundle = PlayerBundle {
        marker: PlayerMarker,
        character_bundle,
    };
    commands.spawn_bundle(player_bundle);
}

/// Receives a [`SelectClick`] event and selects a character.
pub fn player_char_select(
    mut select_clicks: EventReader<SelectClick>,
    mut char_query: QuerySet<(
        Query<(&CharacterIndex, &mut Selected)>,
        Query<(&CharacterIndex, &Transform, &Sprite, &mut Selected)>,
        Query<&mut CharacterTarget, With<PlayerMarker>>,
    )>,
) {
    const SELECT_SIZE: (f32, f32) = (30., 30.);

    // Get selected click
    let select_click = if let Some(some) = select_clicks.iter().last() {
        some
    } else {
        return;
    };

    // Find and set selection
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
        .map(|(index, _, _, selected)| (index, selected))
        .map(|(index, mut selected)| {
            // Set selection
            *selected = Selected::Selected;

            *index
        });

    // Set character selection
    if let Ok(mut character_target) = char_query.q2_mut().single_mut() {
        if let Some(index) = selected_index_opt {
            tracing::info!(message = "selected character", ?index);
            character_target.set_index(index);
        } else {
            character_target.deselect();
        }
    };

    // Deselect everything else
    for (_, mut selected) in char_query
        .q0_mut()
        .iter_mut()
        .filter(|(index, _)| Some(**index) != selected_index_opt)
    {
        *selected = Selected::Unselected;
    }
}

/// Receives [`FocalHold`] event and rotates character in that direction.
pub fn player_focal_rotate(
    mut char_query: Query<&mut Transform, With<PlayerMarker>>,
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

/// Receives [`MotionKey`] input and accelerates character in said direction.
pub fn player_movement(
    motion_input: Res<Input<MotionKey>>,

    // CharacterIndex query
    mut char_query: Query<
        (&CharacterSpeedMultiplier, &mut Transform, &mut Velocity),
        With<PlayerMarker>,
    >,
) {
    let (speed_multiplier, transform, mut velocity) =
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

/// Receives an [`ActionKey`] and performs the associated action.
pub fn player_action(
    time: Res<Time>,

    // ActionKey events
    mut events: EventReader<ActionKey>,

    // CharacterIndex query
    mut char_query: Query<
        (
            &CharacterClass,
            &LastCastInstant,
            &mut CharacterCastState,
            &CharacterTarget,
        ),
        With<PlayerMarker>,
    >,

    // Commands
    mut commands: Commands,
) {
    let (class, last_cast_instant, mut cast_state, target) =
        char_query.single_mut().expect("player not found");

    // Check whether global cooldown has expired
    let global_cooldown_expired =
        last_cast_instant.elapsed(&time).unwrap_or_default() > GLOBAL_COOLDOWN;

    for action_key in events.iter() {
        match class {
            CharacterClass::Mars => {}
            CharacterClass::Medea => match action_key {
                ActionKey::Action1 => {
                    // Check global cooldown
                    if global_cooldown_expired {
                        let start = time.last_update().expect("last update not found");
                        let spell = Spell::Fireball;
                        tracing::info!(message = "casting", ?spell, ?start);
                        cast_state.set_cast(CharacterCast::new(start, *target, spell));
                    }
                }
                ActionKey::Action2 => todo!(),
                ActionKey::Action3 => todo!(),
                ActionKey::Action4 => todo!(),
                ActionKey::Action5 => todo!(),
                ActionKey::Action6 => todo!(),
                ActionKey::Action7 => todo!(),
                ActionKey::Action8 => todo!(),
            },
            CharacterClass::Heka => {}
            CharacterClass::Pluto => {}
            CharacterClass::Mammon => {}
        }
    }
}
