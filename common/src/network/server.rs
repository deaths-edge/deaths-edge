use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    character::{Ability, CharacterClass, CharacterIndex, FocalAngle, Motion, SelectTarget},
    environment::Map,
};

use super::CharacterNetworkAction;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServerMessage {
    GameAction(GameAction),
    CharacterAction(CharacterAction),
    Reconcile(Reconcile),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CharacterAction {
    Motion(CharacterNetworkAction<Motion>),
    Target(CharacterNetworkAction<SelectTarget>),
    Ability(CharacterNetworkAction<Ability>),
    FocalAngle(CharacterNetworkAction<FocalAngle>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reconcile {
    pub index: CharacterIndex,
    pub position: Vec2,
}

impl From<CharacterNetworkAction<Motion>> for CharacterAction {
    fn from(value: CharacterNetworkAction<Motion>) -> Self {
        Self::Motion(value)
    }
}

impl From<CharacterNetworkAction<SelectTarget>> for CharacterAction {
    fn from(value: CharacterNetworkAction<SelectTarget>) -> Self {
        Self::Target(value)
    }
}

impl From<CharacterNetworkAction<Ability>> for CharacterAction {
    fn from(value: CharacterNetworkAction<Ability>) -> Self {
        Self::Ability(value)
    }
}

impl From<CharacterNetworkAction<FocalAngle>> for CharacterAction {
    fn from(value: CharacterNetworkAction<FocalAngle>) -> Self {
        Self::FocalAngle(value)
    }
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct ArenaSetup {
    pub map: Map,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameAction {
    SpawnCharacter(SpawnCharacter),
    Setup(ArenaSetup),
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
