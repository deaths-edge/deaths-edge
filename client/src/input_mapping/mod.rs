mod bindings;
mod keys;
mod mouse;

use std::marker::PhantomData;

use crate::{
    character::{PlayerMarker, PlayerState},
    ui::mouse::{WorldMousePosition, WORLD_MOUSE_LABEL},
};
use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    sprite::collide_aabb::collide,
};

pub use bindings::*;
use common::character::{
    Ability, CharacterEntityAction, CharacterIndex, FocalAngle, Motion, SelectTarget,
    CHARACTER_COMMANDS,
};
pub use keys::*;
pub use mouse::*;

pub const INPUT_MAPPING_LABEL: &str = "input-mapping";
pub const INPUT_TO_CHARACTER_LABEL: &str = "input-to-character";

fn input_to_character<Value>(
    mut input_motion: EventReader<PlayerInputAction<Value>>,
    mut command_motion: EventWriter<CharacterEntityAction<Value>>,
    player_query: Query<Entity, With<PlayerMarker>>,
) where
    Value: Clone + Send + Sync + 'static,
{
    let entity = player_query.single().expect("missing player");
    command_motion.send_batch(
        input_motion
            .iter()
            .map(|input| CharacterEntityAction::new(entity, input.0.clone())),
    )
}

/// A player input action.
#[derive(Clone)]
pub struct PlayerInputAction<Ability>(pub Ability);

/// Takes raw inputs and maps them to in game events with the use of [`Bindings`].
fn input_map(
    // Bindings
    bindings: Res<Bindings>,

    // Keyboard input
    keyboard_input: Res<Input<KeyCode>>,

    // Mouse input
    mut mouse_click_events: EventReader<MouseButtonInput>,
    mouse_position: Res<WorldMousePosition>,
    mut mouse_right_state: Local<MouseRightElementState>,

    // Character
    transform_query: Query<&Transform, With<PlayerMarker>>,
    collide_query: Query<(&CharacterIndex, &Transform, &Sprite)>,

    mut last_angle: Local<FocalAngle>,

    // Outputs
    mut current_motion: Local<Motion>,
    mut motion_events: EventWriter<PlayerInputAction<Motion>>,
    mut abilitys: EventWriter<PlayerInputAction<Ability>>,
    mut focal_holds: EventWriter<PlayerInputAction<FocalAngle>>,
    mut target: EventWriter<PlayerInputAction<SelectTarget>>,
) {
    let pressed_iter = keyboard_input
        .get_just_pressed()
        .filter_map(|key| bindings.try_map(*key).ok());

    let previous_motion = *current_motion;

    for input in pressed_iter {
        trace!(message = "just pressed", ?input);
        match input {
            BoundKey::Motion(motion_key) => *current_motion = motion_key.press(*current_motion),
            _ => (),
        }
    }

    let released_iter = keyboard_input
        .get_just_released()
        .filter_map(|key| bindings.try_map(*key).ok());

    for input in released_iter {
        match input {
            BoundKey::Motion(motion_key) => *current_motion = motion_key.release(*current_motion),
            BoundKey::Ability(ability_key) => {
                abilitys.send(PlayerInputAction(ability_key.into_ability()))
            }
        }
    }

    if previous_motion != *current_motion {
        motion_events.send(PlayerInputAction(*current_motion));
    }

    let mouse_input_last = mouse_click_events.iter().last();

    match mouse_input_last {
        Some(MouseButtonInput {
            button: MouseButton::Left,
            state: ElementState::Released,
        }) => {
            const SELECT_SIZE: (f32, f32) = (30., 30.);

            let index_opt = collide_query
                .iter()
                .find(|(_, char_transform, char_sprite)| {
                    collide(
                        mouse_position.position.extend(0.),
                        SELECT_SIZE.into(),
                        char_transform.translation,
                        char_sprite.size,
                    )
                    .is_some()
                })
                .map(|(index, _, _)| *index);

            target.send(PlayerInputAction(SelectTarget(index_opt)));
        }
        Some(MouseButtonInput {
            button: MouseButton::Right,
            state: ElementState::Pressed,
        }) => *mouse_right_state = MouseRightElementState::Pressed,
        Some(MouseButtonInput {
            button: MouseButton::Right,
            state: ElementState::Released,
        }) => *mouse_right_state = MouseRightElementState::Released,
        _ => (),
    }

    if *mouse_right_state == MouseRightElementState::Pressed {
        let transform = transform_query.single().expect("could not find player");
        let translation = transform.translation.truncate();

        let diff = mouse_position.position - translation;

        let angle = FocalAngle(Vec2::new(0., 1.).angle_between(diff));

        info!(message = "sending focal angle", angle = %angle.0);
        if !last_angle.almost_eq(&angle) {
            *last_angle = angle;
            focal_holds.send(PlayerInputAction(angle));
        }
    }
}

pub struct InputToCharPlugin<T> {
    _value: PhantomData<T>,
}

impl<T> InputToCharPlugin<T> {
    pub fn new() -> Self {
        Self {
            _value: PhantomData,
        }
    }
}

impl<T> Plugin for InputToCharPlugin<T>
where
    T: Send + Sync + 'static,
    T: Clone,
{
    fn build(&self, app: &mut AppBuilder) {
        let input_to_character = SystemSet::on_update(PlayerState::Spawned)
            .label(INPUT_TO_CHARACTER_LABEL)
            // INPUT_MAPPING_LABEL sends PlayerInputAction<Value> event
            .after(INPUT_MAPPING_LABEL)
            // CHARACTER_COMMANDS reads CharacterEntityAction<Value>
            .before(CHARACTER_COMMANDS)
            .with_system(input_to_character::<T>.system());

        app.add_event::<PlayerInputAction<T>>()
            .add_system_set(input_to_character);
    }
}

pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::on_update(PlayerState::Spawned)
            .label(INPUT_MAPPING_LABEL)
            // WORLD_MOUSE_LABEL sets WorldMousePosition
            .after(WORLD_MOUSE_LABEL)
            // INPUT_TO_CHARACTER_LABEL reads PlayerInputAction<Value> events
            .before(INPUT_TO_CHARACTER_LABEL)
            .with_system(input_map.system());
        app.init_resource::<Bindings>()
            .add_plugin(InputToCharPlugin::<Motion>::new())
            .add_plugin(InputToCharPlugin::<SelectTarget>::new())
            .add_plugin(InputToCharPlugin::<Ability>::new())
            .add_plugin(InputToCharPlugin::<FocalAngle>::new())
            .add_event::<SelectClick>()
            .add_system_set(system_set);
    }
}
