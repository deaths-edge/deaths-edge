use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    Position(PositionMessage),
    Velocity(VelocityMessage),
}
