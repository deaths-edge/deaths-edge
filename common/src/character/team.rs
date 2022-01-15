use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone, Copy, Component)]
pub enum Team {
    Red,
    Blue,
}
