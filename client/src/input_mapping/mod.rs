mod bindings;
mod keys;
mod mouse;

use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

use crate::{state::ClientState, ui::mouse::WorldMousePosition};

pub use bindings::*;
pub use keys::*;
pub use mouse::*;

pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::on_update(ClientState::Arena)
            .label("input-mapping")
            .with_system(input_map.system());
        app.init_resource::<Bindings>()
            .add_event::<Motion>()
            .add_event::<ActionKey>()
            .add_event::<FocalHold>()
            .add_event::<SelectClick>()
            .add_system_set(system_set);
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum MotionDirection {
    Left,
    Forward,
    Right,
    Backward,
    LeftForward,
    LeftBackward,
    RightForward,
    RightBackward,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Motion(pub Option<MotionDirection>);

impl Motion {
    pub fn release(self, key: MotionKey) -> Self {
        match self.0 {
            None => {
                let direction = match key {
                    MotionKey::Left => MotionDirection::Right,
                    MotionKey::Forward => MotionDirection::Backward,
                    MotionKey::Right => MotionDirection::Left,
                    MotionKey::Backward => MotionDirection::Forward,
                };
                Self(Some(direction))
            }
            Some(direction) => {
                let direction_opt = match (key, direction) {
                    (MotionKey::Left, MotionDirection::Left) => None,
                    (MotionKey::Left, MotionDirection::LeftForward) => {
                        Some(MotionDirection::Forward)
                    }
                    (MotionKey::Left, MotionDirection::Forward) => {
                        Some(MotionDirection::RightForward)
                    }
                    (MotionKey::Left, MotionDirection::Backward) => {
                        Some(MotionDirection::RightBackward)
                    }
                    (MotionKey::Left, MotionDirection::LeftBackward) => {
                        Some(MotionDirection::Backward)
                    }
                    (MotionKey::Forward, MotionDirection::Left) => {
                        Some(MotionDirection::LeftBackward)
                    }
                    (MotionKey::Forward, MotionDirection::LeftForward) => {
                        Some(MotionDirection::Left)
                    }
                    (MotionKey::Forward, MotionDirection::Forward) => None,
                    (MotionKey::Forward, MotionDirection::RightForward) => {
                        Some(MotionDirection::Right)
                    }
                    (MotionKey::Forward, MotionDirection::Right) => {
                        Some(MotionDirection::RightBackward)
                    }
                    (MotionKey::Right, MotionDirection::Forward) => {
                        Some(MotionDirection::LeftForward)
                    }
                    (MotionKey::Right, MotionDirection::RightForward) => {
                        Some(MotionDirection::Forward)
                    }
                    (MotionKey::Right, MotionDirection::Right) => None,
                    (MotionKey::Right, MotionDirection::RightBackward) => {
                        Some(MotionDirection::Backward)
                    }
                    (MotionKey::Right, MotionDirection::Backward) => {
                        Some(MotionDirection::LeftBackward)
                    }
                    (MotionKey::Backward, MotionDirection::Left) => {
                        Some(MotionDirection::LeftForward)
                    }
                    (MotionKey::Backward, MotionDirection::Right) => {
                        Some(MotionDirection::RightForward)
                    }
                    (MotionKey::Backward, MotionDirection::RightBackward) => {
                        Some(MotionDirection::Right)
                    }
                    (MotionKey::Backward, MotionDirection::Backward) => None,
                    (MotionKey::Backward, MotionDirection::LeftBackward) => {
                        Some(MotionDirection::Left)
                    }
                    _ => unreachable!("cannot release"),
                };
                Self(direction_opt)
            }
        }
    }

    pub fn press(self, key: MotionKey) -> Self {
        match self.0 {
            Some(some) => {
                let direction_opt = match (key, some) {
                    (MotionKey::Left, MotionDirection::Forward) => {
                        Some(MotionDirection::LeftForward)
                    }
                    (MotionKey::Left, MotionDirection::RightForward) => {
                        Some(MotionDirection::Forward)
                    }
                    (MotionKey::Left, MotionDirection::Right) => None,
                    (MotionKey::Left, MotionDirection::RightBackward) => {
                        Some(MotionDirection::Backward)
                    }
                    (MotionKey::Left, MotionDirection::Backward) => {
                        Some(MotionDirection::LeftBackward)
                    }
                    (MotionKey::Forward, MotionDirection::Left) => {
                        Some(MotionDirection::LeftForward)
                    }
                    (MotionKey::Forward, MotionDirection::Right) => {
                        Some(MotionDirection::RightForward)
                    }
                    (MotionKey::Forward, MotionDirection::RightBackward) => {
                        Some(MotionDirection::Right)
                    }
                    (MotionKey::Forward, MotionDirection::Backward) => None,
                    (MotionKey::Forward, MotionDirection::LeftBackward) => {
                        Some(MotionDirection::Left)
                    }
                    (MotionKey::Right, MotionDirection::Left) => None,
                    (MotionKey::Right, MotionDirection::LeftForward) => {
                        Some(MotionDirection::Forward)
                    }
                    (MotionKey::Right, MotionDirection::Forward) => {
                        Some(MotionDirection::RightForward)
                    }
                    (MotionKey::Right, MotionDirection::Backward) => {
                        Some(MotionDirection::RightBackward)
                    }
                    (MotionKey::Right, MotionDirection::LeftBackward) => {
                        Some(MotionDirection::Backward)
                    }
                    (MotionKey::Backward, MotionDirection::Left) => {
                        Some(MotionDirection::LeftBackward)
                    }
                    (MotionKey::Backward, MotionDirection::LeftForward) => {
                        Some(MotionDirection::Left)
                    }
                    (MotionKey::Backward, MotionDirection::Forward) => None,
                    (MotionKey::Backward, MotionDirection::RightForward) => {
                        Some(MotionDirection::Right)
                    }
                    (MotionKey::Backward, MotionDirection::Right) => {
                        Some(MotionDirection::RightBackward)
                    }
                    _ => unreachable!("cannot press"),
                };
                Self(direction_opt)
            }
            None => {
                let direction = match key {
                    MotionKey::Left => MotionDirection::Left,
                    MotionKey::Forward => MotionDirection::Forward,
                    MotionKey::Right => MotionDirection::Right,
                    MotionKey::Backward => MotionDirection::Backward,
                };

                Self(Some(direction))
            }
        }
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
    mut motion_events: EventWriter<Motion>,
    mut actions: EventWriter<ActionKey>,
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
            BoundKey::Motion(motion_key) => *current_motion = current_motion.press(motion_key),
            _ => (),
        }
    }

    let released_iter = keyboard_input
        .get_just_released()
        .filter_map(|key| bindings.try_map(*key).ok());

    for input in released_iter {
        match input {
            BoundKey::Motion(motion_key) => *current_motion = current_motion.release(motion_key),
            BoundKey::Action(action_key) => actions.send(action_key),
        }
    }

    if previous_motion != *current_motion {
        motion_events.send(*current_motion);
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
