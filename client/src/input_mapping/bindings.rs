use bevy::prelude::*;

use super::{AbilityKey, BoundKey, MotionKey};

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

pub struct AbilityBindings {
    pub ability_1: KeyCode,
    pub ability_2: KeyCode,
    pub ability_3: KeyCode,
    pub ability_4: KeyCode,
    pub ability_5: KeyCode,
    pub ability_6: KeyCode,
    pub ability_7: KeyCode,
    pub ability_8: KeyCode,
}

impl Default for AbilityBindings {
    fn default() -> Self {
        Self {
            ability_1: KeyCode::Key1,
            ability_2: KeyCode::Key2,
            ability_3: KeyCode::Key3,
            ability_4: KeyCode::Key4,
            ability_5: KeyCode::Key5,
            ability_6: KeyCode::Key6,
            ability_7: KeyCode::Key7,
            ability_8: KeyCode::Key8,
        }
    }
}

impl AbilityBindings {
    fn try_map(&self, key: KeyCode) -> Result<AbilityKey, KeyCode> {
        use AbilityKey::*;

        if key == self.ability_1 {
            Ok(Key1)
        } else if key == self.ability_2 {
            Ok(Key2)
        } else if key == self.ability_3 {
            Ok(Key3)
        } else if key == self.ability_4 {
            Ok(Key4)
        } else if key == self.ability_5 {
            Ok(Key5)
        } else if key == self.ability_6 {
            Ok(Key6)
        } else if key == self.ability_7 {
            Ok(Key7)
        } else if key == self.ability_8 {
            Ok(Key8)
        } else {
            Err(key)
        }
    }
}

#[derive(Default)]
pub struct Bindings {
    movement_bindings: MovementBindings,
    ability_bindings: AbilityBindings,
}

impl Bindings {
    pub fn try_map(&self, key: KeyCode) -> Result<BoundKey, KeyCode> {
        self.movement_bindings
            .try_map(key)
            .map(BoundKey::Motion)
            .or_else(|key| self.ability_bindings.try_map(key).map(BoundKey::Ability))
    }
}
