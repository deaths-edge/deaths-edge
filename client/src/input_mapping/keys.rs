use common::character::{Ability, Motion, NormalMotion, ParallelMotion};

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
pub enum AbilityKey {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
}

impl AbilityKey {
    // TODO: Remapping
    pub fn into_ability(self) -> Ability {
        match self {
            AbilityKey::Key1 => Ability::Ability1,
            AbilityKey::Key2 => Ability::Ability2,
            AbilityKey::Key3 => Ability::Ability3,
            AbilityKey::Key4 => Ability::Ability4,
            AbilityKey::Key5 => Ability::Ability5,
            AbilityKey::Key6 => Ability::Ability6,
            AbilityKey::Key7 => Ability::Ability7,
            AbilityKey::Key8 => Ability::Ability8,
        }
    }
}

#[derive(Debug)]
pub enum BoundKey {
    Ability(AbilityKey),
    Motion(MotionKey),
}
