mod bindings;
mod keys;
mod mouse;

use std::marker::PhantomData;

use crate::{
    character::{PlayerMarker, PlayerState},
    state::ClientState,
    ui::mouse::WorldMousePosition,
};
use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

pub use bindings::*;
use common::character::{Action, CharacterEntityCommand, FocalAngle, Motion};
pub use keys::*;
pub use mouse::*;

pub const INPUT_MAPPING_LABEL: &str = "input-mapping";
pub const INPUT_TO_CHARACTER_LABEL: &str = "input-to-character";

fn input_to_character<Value>(
    mut input_motion: EventReader<PlayerInputCommand<Value>>,
    mut command_motion: EventWriter<CharacterEntityCommand<Value>>,
    player_query: Query<Entity, With<PlayerMarker>>,
) where
    Value: Clone + Send + Sync + 'static,
{
    let entity = player_query.single().expect("missing player");
    command_motion.send_batch(
        input_motion
            .iter()
            .map(|input| CharacterEntityCommand::new(entity, input.0.clone())),
    )
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
            .with_system(input_to_character::<T>.system());

        app.add_event::<PlayerInputCommand<T>>()
            .add_system_set(input_to_character);
    }
}

pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::on_update(ClientState::Arena)
            .label(INPUT_MAPPING_LABEL)
            .with_system(input_map.system());
        app.init_resource::<Bindings>()
            .add_plugin(InputToCharPlugin::<Motion>::new())
            .add_plugin(InputToCharPlugin::<Action>::new())
            .add_plugin(InputToCharPlugin::<FocalAngle>::new())
            .add_event::<SelectClick>()
            .add_system_set(system_set);
    }
}

/// A player input command.
#[derive(Clone)]
pub struct PlayerInputCommand<Action>(pub Action);

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
    character_query: Query<&Transform, With<PlayerMarker>>,

    // Outputs
    mut current_motion: Local<Motion>,
    mut motion_events: EventWriter<PlayerInputCommand<Motion>>,
    mut actions: EventWriter<PlayerInputCommand<Action>>,
    mut focal_holds: EventWriter<PlayerInputCommand<FocalAngle>>,
    mut select_clicks: EventWriter<SelectClick>,
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
            BoundKey::Action(action_key) => {
                actions.send(PlayerInputCommand(action_key.into_action()))
            }
        }
    }

    if previous_motion != *current_motion {
        motion_events.send(PlayerInputCommand(*current_motion));
    }

    let mouse_input_last = mouse_click_events.iter().last();

    match mouse_input_last {
        Some(MouseButtonInput {
            button: MouseButton::Left,
            state: ElementState::Released,
        }) => {
            select_clicks.send(SelectClick {
                mouse_position: mouse_position.position,
            });
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
        let transform = character_query.single().expect("could not find player");
        let translation = transform.translation.truncate();

        let diff = mouse_position.position - translation;

        let angle = Vec2::new(0., 1.).angle_between(diff);

        info!(message = "sending focal angle", %angle);
        focal_holds.send(PlayerInputCommand(FocalAngle(angle)))
    }
}
