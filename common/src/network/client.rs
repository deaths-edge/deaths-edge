use serde::{Deserialize, Serialize};

use super::messages::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    Position(PositionMessage),
    Velocity(VelocityMessage),
}
