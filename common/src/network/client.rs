use serde::{Deserialize, Serialize};

use crate::{
    character::{Action, FocalAngle, Motion},
    game::ArenaPermit,
};

/// Primary message sent from client to server.
#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    Permit(ArenaPermit),
    Command(ClientCommand),
}

impl<T: Into<ClientCommand>> From<T> for ClientMessage {
    fn from(value: T) -> Self {
        Self::Command(value.into())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientCommand {
    Motion(Motion),
    Action(Action),
    Rotate(FocalAngle),
}

impl From<Action> for ClientCommand {
    fn from(action: Action) -> Self {
        Self::Action(action)
    }
}

impl From<Motion> for ClientCommand {
    fn from(motion: Motion) -> Self {
        Self::Motion(motion)
    }
}

impl From<FocalAngle> for ClientCommand {
    fn from(angle: FocalAngle) -> Self {
        Self::Rotate(angle)
    }
}

impl ClientMessage {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(payload)
    }
}
