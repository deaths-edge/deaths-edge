use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::{Action, CharacterClass, CharacterIndex, FocalAngle, Motion, Target};

use super::CharacterNetworkCommand;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServerMessage {
    ArenaPasscodeAck,
    GameCommand(GameCommand),
    CharacterCommand(CharacterCommand),
    Reconcile(Reconcile),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CharacterCommand {
    Motion(CharacterNetworkCommand<Motion>),
    Target(CharacterNetworkCommand<Target>),
    Action(CharacterNetworkCommand<Action>),
    FocalAngle(CharacterNetworkCommand<FocalAngle>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reconcile {
    pub index: CharacterIndex,
    pub position: Vec2,
}

impl From<CharacterNetworkCommand<Motion>> for CharacterCommand {
    fn from(value: CharacterNetworkCommand<Motion>) -> Self {
        Self::Motion(value)
    }
}

impl From<CharacterNetworkCommand<Target>> for CharacterCommand {
    fn from(value: CharacterNetworkCommand<Target>) -> Self {
        Self::Target(value)
    }
}

impl From<CharacterNetworkCommand<Action>> for CharacterCommand {
    fn from(value: CharacterNetworkCommand<Action>) -> Self {
        Self::Action(value)
    }
}

impl From<CharacterNetworkCommand<FocalAngle>> for CharacterCommand {
    fn from(value: CharacterNetworkCommand<FocalAngle>) -> Self {
        Self::FocalAngle(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameCommand {
    SpawnCharacter(SpawnCharacter),
}

impl ServerMessage {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(payload)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
