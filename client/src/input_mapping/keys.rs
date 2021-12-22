use common::character::{Action, Motion, MotionDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionKey {
    Left,
    Forward,
    Right,
    Backward,
}

impl MotionKey {
    pub fn release(self, motion: Motion) -> Motion {
        match motion.0 {
            None => {
                let direction = match self {
                    MotionKey::Left => MotionDirection::Right,
                    MotionKey::Forward => MotionDirection::Backward,
                    MotionKey::Right => MotionDirection::Left,
                    MotionKey::Backward => MotionDirection::Forward,
                };
                Motion(Some(direction))
            }
            Some(direction) => {
                let direction_opt = match (self, direction) {
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
                    (key, direction) => unreachable!("cannot release: {:?} {:?}", key, direction),
                };
                Motion(direction_opt)
            }
        }
    }

    pub fn press(self, motion: Motion) -> Motion {
        match motion.0 {
            Some(direction) => {
                let direction_opt = match (self, direction) {
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
                Motion(direction_opt)
            }
            None => {
                let direction = match self {
                    MotionKey::Left => MotionDirection::Left,
                    MotionKey::Forward => MotionDirection::Forward,
                    MotionKey::Right => MotionDirection::Right,
                    MotionKey::Backward => MotionDirection::Backward,
                };

                Motion(Some(direction))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionKey {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
}

impl ActionKey {
    // TODO: Remapping
    pub fn into_action(self) -> Action {
        match self {
            ActionKey::Key1 => Action::Action1,
            ActionKey::Key2 => Action::Action2,
            ActionKey::Key3 => Action::Action3,
            ActionKey::Key4 => Action::Action4,
            ActionKey::Key5 => Action::Action5,
            ActionKey::Key6 => Action::Action6,
            ActionKey::Key7 => Action::Action7,
            ActionKey::Key8 => Action::Action8,
        }
    }
}

#[derive(Debug)]
pub enum BoundKey {
    Action(ActionKey),
    Motion(MotionKey),
}
