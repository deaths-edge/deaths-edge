use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
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

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct Motion(pub Option<MotionDirection>);
