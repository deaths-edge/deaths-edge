mod bindings;
mod keys;
mod mouse;

use crate::{state::ClientState, ui::mouse::WorldMousePosition};
use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

pub use bindings::*;
use common::character::{Action, Motion};
pub use keys::*;
pub use mouse::*;

pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::on_update(ClientState::Arena)
            .label("input-mapping")
            .with_system(input_map.system());
        app.init_resource::<Bindings>()
            .add_event::<PlayerInputCommand<Motion>>()
            .add_event::<PlayerInputCommand<Action>>()
            .add_event::<FocalHold>()
            .add_event::<SelectClick>()
            .add_system_set(system_set);
    }
}

/// A player input command.
#[derive(Clone)]
pub struct PlayerInputCommand<Action> {
    action: Action,
}

impl<Action> PlayerInputCommand<Action> {
    pub fn into_inner(self) -> Action {
        self.action
    }

    pub fn inner(&self) -> &Action {
        &self.action
    }

    pub fn action(&self) -> &Action {
        &self.action
    }
}

impl<Action> From<Action> for PlayerInputCommand<Action> {
    fn from(action: Action) -> Self {
        PlayerInputCommand { action }
    }
}

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

    // Outputs
    mut current_motion: Local<Motion>,
    mut motion_events: EventWriter<PlayerInputCommand<Motion>>,
    mut actions: EventWriter<PlayerInputCommand<Action>>,
    mut select_clicks: EventWriter<SelectClick>,
    mut focal_holds: EventWriter<FocalHold>,
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
                actions.send(PlayerInputCommand::from(action_key.into_action()))
            }
        }
    }

    if previous_motion != *current_motion {
        motion_events.send(PlayerInputCommand::from(*current_motion));
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
        focal_holds.send(FocalHold {
            mouse_position: mouse_position.position,
        })
    }
}
