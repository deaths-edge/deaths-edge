/// Different modes of character control.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Control {
    Stun,
    Dazed,
    Root,
}

#[derive(Debug, Clone, Default)]
pub struct Controls(pub Vec<Control>);
