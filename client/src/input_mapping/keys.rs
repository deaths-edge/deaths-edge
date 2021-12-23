use common::character::{Action, Motion, NormalMotion, ParallelMotion};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionKey {
    Left,
    Forward,
    Right,
    Backward,
}

impl MotionKey {
    pub fn release(self, motion: Motion) -> Motion {
        match self {
            MotionKey::Left => match motion {
                Motion {
                    parallel,
                    normal: Some(NormalMotion::Left),
                } => Motion {
                    parallel,
                    normal: None,
                },
                Motion {
                    parallel,
                    normal: None,
                } => Motion {
                    parallel,
                    normal: Some(NormalMotion::Right),
                },
                _ => unreachable!("can't be moving right while releasing left key"),
            },
            MotionKey::Forward => match motion {
                Motion {
                    parallel: Some(ParallelMotion::Forward),
                    normal,
                } => Motion {
                    parallel: None,
                    normal,
                },
                Motion {
                    parallel: None,
                    normal,
                } => Motion {
                    parallel: Some(ParallelMotion::Backward),
                    normal,
                },
                _ => unreachable!("can't be moving backward while releasing forward key"),
            },
            MotionKey::Right => match motion {
                Motion {
                    parallel,
                    normal: Some(NormalMotion::Right),
                } => Motion {
                    parallel,
                    normal: None,
                },
                Motion {
                    parallel,
                    normal: None,
                } => Motion {
                    parallel,
                    normal: Some(NormalMotion::Left),
                },
                _ => unreachable!("can't be moving right while releasing left key"),
            },
            MotionKey::Backward => match motion {
                Motion {
                    parallel: Some(ParallelMotion::Backward),
                    normal,
                } => Motion {
                    parallel: None,
                    normal,
                },
                Motion {
                    parallel: None,
                    normal,
                } => Motion {
                    parallel: Some(ParallelMotion::Forward),
                    normal,
                },
                _ => unreachable!("can't be moving forward while releasing backward key"),
            },
        }
    }

    pub fn press(self, motion: Motion) -> Motion {
        match self {
            MotionKey::Left => match motion {
                Motion {
                    parallel,
                    normal: Some(NormalMotion::Right),
                } => Motion {
                    parallel,
                    normal: None,
                },
                Motion {
                    parallel,
                    normal: None,
                } => Motion {
                    parallel,
                    normal: Some(NormalMotion::Left),
                },
                _ => unreachable!("can't be moving right while releasing left key"),
            },
            MotionKey::Forward => match motion {
                Motion {
                    parallel: Some(ParallelMotion::Backward),
                    normal,
                } => Motion {
                    parallel: None,
                    normal,
                },
                Motion {
                    parallel: None,
                    normal,
                } => Motion {
                    parallel: Some(ParallelMotion::Forward),
                    normal,
                },
                _ => unreachable!("can't be moving backward while releasing forward key"),
            },
            MotionKey::Right => match motion {
                Motion {
                    parallel,
                    normal: Some(NormalMotion::Left),
                } => Motion {
                    parallel,
                    normal: None,
                },
                Motion {
                    parallel,
                    normal: None,
                } => Motion {
                    parallel,
                    normal: Some(NormalMotion::Right),
                },
                _ => unreachable!("can't be moving right while releasing left key"),
            },
            MotionKey::Backward => match motion {
                Motion {
                    parallel: Some(ParallelMotion::Forward),
                    normal,
                } => Motion {
                    parallel: None,
                    normal,
                },
                Motion {
                    parallel: None,
                    normal,
                } => Motion {
                    parallel: Some(ParallelMotion::Backward),
                    normal,
                },
                _ => unreachable!("can't be moving forward while releasing backward key"),
            },
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
