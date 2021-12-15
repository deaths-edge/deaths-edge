use serde::{Deserialize, Serialize};

use crate::character::Motion;

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    // Position(PositionMessage),
    // Velocity(VelocityMessage),
    Motion(Motion),
    Passcode(u64),
}

impl ClientMessage {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes::<ClientMessage>(payload)
    }
}
