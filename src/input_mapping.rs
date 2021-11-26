use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
};

use crate::ui::WorldMousePosition;

pub struct InputMapPlugin;

impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::new()
            .label("input-mapping")
            .with_system(input_map.system());
        app.init_resource::<Bindings>()
            .init_resource::<Input<MotionKey>>()
            .init_resource::<Input<ActionKey>>()
            .add_event::<FocalHold>()
            .add_system_set(system_set);
        // TODO: Read from file
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionKey {
    Left,
    Forward,
    Right,
    Backward,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionKey {
    Action1,
    Action2,
    Action3,
    Action4,
    Action5,
    Action6,
    Action7,
    Action8,
}

pub enum BoundInput {
    Action(ActionKey),
    Motion(MotionKey),
}

#[derive(Debug)]
pub struct FocalHold {
    pub mouse_position: Vec2,
}

pub struct MovementBindings {
    pub move_left: KeyCode,
    pub move_forward: KeyCode,
    pub move_right: KeyCode,
    pub move_backward: KeyCode,
}

impl Default for MovementBindings {
    fn default() -> Self {
        Self {
            move_left: KeyCode::A,
            move_forward: KeyCode::W,
            move_right: KeyCode::D,
            move_backward: KeyCode::S,
        }
    }
}

impl MovementBindings {
    fn try_map(&self, key: KeyCode) -> Result<MotionKey, KeyCode> {
        use MotionKey::*;

        if key == self.move_left {
            Ok(Left)
        } else if key == self.move_forward {
            Ok(Forward)
        } else if key == self.move_right {
            Ok(Right)
        } else if key == self.move_backward {
            Ok(Backward)
        } else {
            Err(key)
        }
    }
}

pub struct ActionBindings {
    pub action_1: KeyCode,
    pub action_2: KeyCode,
    pub action_3: KeyCode,
    pub action_4: KeyCode,
    pub action_5: KeyCode,
    pub action_6: KeyCode,
    pub action_7: KeyCode,
    pub action_8: KeyCode,
}

impl Default for ActionBindings {
    fn default() -> Self {
        Self {
            action_1: KeyCode::Key1,
            action_2: KeyCode::Key2,
            action_3: KeyCode::Key3,
            action_4: KeyCode::Key4,
            action_5: KeyCode::Key5,
            action_6: KeyCode::Key6,
            action_7: KeyCode::Key7,
            action_8: KeyCode::Key8,
        }
    }
}

impl ActionBindings {
    fn try_map(&self, key: KeyCode) -> Result<ActionKey, KeyCode> {
        use ActionKey::*;

        if key == self.action_1 {
            Ok(Action1)
        } else if key == self.action_2 {
            Ok(Action2)
        } else if key == self.action_3 {
            Ok(Action3)
        } else if key == self.action_4 {
            Ok(Action4)
        } else if key == self.action_5 {
            Ok(Action5)
        } else if key == self.action_6 {
            Ok(Action6)
        } else if key == self.action_7 {
            Ok(Action7)
        } else if key == self.action_8 {
            Ok(Action8)
        } else {
            Err(key)
        }
    }
}

#[derive(Default)]
pub struct Bindings {
    movement_bindings: MovementBindings,
    action_bindings: ActionBindings,
}

impl Bindings {
    pub fn try_map(&self, key: KeyCode) -> Result<BoundInput, KeyCode> {
        use BoundInput::*;

        self.movement_bindings
            .try_map(key)
            .map(Motion)
            .or_else(|key| self.action_bindings.try_map(key).map(Action))
    }
}

#[derive(PartialEq)]
enum MouseRightElementState {
    Pressed,
    Released,
}

impl Default for MouseRightElementState {
    fn default() -> Self {
        Self::Released
    }
}

fn input_map(
    // Bindings
    bindings: Res<Bindings>,

    // Keyboard input
    keyboard_input: Res<Input<KeyCode>>,

    // Mouse input
    mut mouse_click_events: EventReader<MouseButtonInput>,
    mouse_position: Res<WorldMousePosition>,
    mut mouse_right_state: Local<MouseRightElementState>,

    // Event writers
    mut motion_input: ResMut<Input<MotionKey>>,
    mut action_input: ResMut<Input<ActionKey>>,
    mut focal_holds: EventWriter<FocalHold>,
) {
    motion_input.update();
    action_input.update();

    let pressed_iter = keyboard_input
        .get_just_pressed()
        .filter_map(|key| bindings.try_map(*key).ok());

    for input in pressed_iter {
        match input {
            BoundInput::Motion(motion_key) => motion_input.press(motion_key),
            BoundInput::Action(action_key) => action_input.press(action_key),
        }
    }

    let released_iter = keyboard_input
        .get_just_released()
        .filter_map(|key| bindings.try_map(*key).ok());

    for input in released_iter {
        match input {
            BoundInput::Motion(motion_key) => motion_input.release(motion_key),
            BoundInput::Action(action_key) => action_input.release(action_key),
        }
    }

    let mouse_input_opt = mouse_click_events
        .iter()
        .last()
        .filter(|input| input.button == MouseButton::Right)
        .map(|input| input.state);

    match mouse_input_opt {
        Some(ElementState::Pressed) => *mouse_right_state = MouseRightElementState::Pressed,
        Some(ElementState::Released) => *mouse_right_state = MouseRightElementState::Released,
        None => (),
    };

    if *mouse_right_state == MouseRightElementState::Pressed {
        focal_holds.send(FocalHold {
            mouse_position: mouse_position.position,
        })
    }
}
