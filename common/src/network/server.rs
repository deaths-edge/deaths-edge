use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::{CharacterClass, CharacterIndex};

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerMessage {
    ArenaPasscodeAck,
    SpawnCharacter(SpawnCharacter),
}

impl ServerMessage {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(payload)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpawnCharacter {
    index: CharacterIndex,
    class: CharacterClass,
    player: bool,
    position: Vec2,
}

impl SpawnCharacter {
    pub fn new(index: CharacterIndex, class: CharacterClass, player: bool, position: Vec2) -> Self {
        Self {
            index,
            class,
            player,
            position,
        }
    }

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
