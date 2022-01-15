use bevy::prelude::*;

/// Different modes of character control.
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum Control {
    Stun,
    Dazed,
    Root,
}

#[derive(Debug, Clone, Default, Component)]
pub struct Controls(pub Vec<Control>);
