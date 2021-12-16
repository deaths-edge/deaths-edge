use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::{CharacterClass, CharacterIndex};

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerMessage {
    PasscodeAck,
    SpawnCharacter(SpawnCharacter),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpawnCharacter {
    index: CharacterIndex,
    class: CharacterClass,
    player: bool,
    position: Vec2,
}

impl SpawnCharacter {
    pub fn index(&self) -> CharacterIndex {
        self.index
    }

    pub fn player(&self) -> bool {
        self.player
    }

    pub fn class(&self) -> CharacterClass {
        self.class
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}
