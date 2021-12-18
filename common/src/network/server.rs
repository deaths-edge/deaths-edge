use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::{Action, CharacterClass, CharacterIndex, Motion};

use super::CharacterNetworkCommand;

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerMessage {
    ArenaPasscodeAck,
    GameCommand(GameCommand),
    CharacterCommand(CharacterCommand),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CharacterCommand {
    Motion(CharacterNetworkCommand<Motion>),
    Action(CharacterNetworkCommand<Action>),
}

impl From<CharacterNetworkCommand<Motion>> for CharacterCommand {
    fn from(value: CharacterNetworkCommand<Motion>) -> Self {
        Self::Motion(value)
    }
}

impl From<CharacterNetworkCommand<Action>> for CharacterCommand {
    fn from(value: CharacterNetworkCommand<Action>) -> Self {
        Self::Action(value)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GameCommand {
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
    rotation: f32,
}

impl SpawnCharacter {
    pub fn new(
        index: CharacterIndex,
        class: CharacterClass,
        player: bool,
        position: Vec2,
        rotation: f32,
    ) -> Self {
        Self {
            index,
            class,
            player,
            position,
            rotation,
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
