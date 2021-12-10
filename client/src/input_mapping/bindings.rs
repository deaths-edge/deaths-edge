use bevy::prelude::*;

use super::{ActionKey, BoundKey, MotionKey};

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
        use super::MotionKey::*;

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
    pub fn try_map(&self, key: KeyCode) -> Result<BoundKey, KeyCode> {
        self.movement_bindings
            .try_map(key)
            .map(BoundKey::Motion)
            .or_else(|key| self.action_bindings.try_map(key).map(BoundKey::Action))
    }
}
