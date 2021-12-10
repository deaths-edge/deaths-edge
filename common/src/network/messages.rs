use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::character::CharacterClass;

#[derive(Debug, Deserialize, Serialize)]
pub struct PositionMessage {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VelocityMessage {
    x: f32,
    y: f32,
}
