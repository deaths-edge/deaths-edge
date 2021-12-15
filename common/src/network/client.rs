use serde::{Deserialize, Serialize};

use crate::character::{Action, Motion};

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    Motion(Motion),
    Action(Action),
    Passcode(u64),
}

impl From<Action> for ClientMessage {
    fn from(action: Action) -> Self {
        Self::Action(action)
    }
}

impl From<Motion> for ClientMessage {
    fn from(motion: Motion) -> Self {
        Self::Motion(motion)
    }
}

impl ClientMessage {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes::<ClientMessage>(payload)
    }
}
