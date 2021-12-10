use serde::{Deserialize, Serialize};

use crate::actions::Motion;

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    // Position(PositionMessage),
    // Velocity(VelocityMessage),
    Motion(Motion),
}

impl ClientMessage {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes::<ClientMessage>(payload)
    }
}
