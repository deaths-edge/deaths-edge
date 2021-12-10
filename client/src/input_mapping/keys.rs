#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotionKey {
    Left,
    Forward,
    Right,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
pub enum BoundKey {
    Action(ActionKey),
    Motion(MotionKey),
}
