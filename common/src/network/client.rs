use serde::{Deserialize, Serialize};

use crate::{
    character::{Ability, FocalAngle, Motion, Target},
    game::ArenaPermit,
};

/// Primary message sent from client to server.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    Permit(ArenaPermit),
    Action(ClientAction),
}

impl<T: Into<ClientAction>> From<T> for ClientMessage {
    fn from(value: T) -> Self {
        Self::Action(value.into())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ClientAction {
    Motion(Motion),
    Target(Target),
    Ability(Ability),
    Rotate(FocalAngle),
}

impl From<Ability> for ClientAction {
    fn from(ability: Ability) -> Self {
        Self::Ability(ability)
    }
}

impl From<Target> for ClientAction {
    fn from(target: Target) -> Self {
        Self::Target(target)
    }
}

impl From<Motion> for ClientAction {
    fn from(motion: Motion) -> Self {
        Self::Motion(motion)
    }
}

impl From<FocalAngle> for ClientAction {
    fn from(angle: FocalAngle) -> Self {
        Self::Rotate(angle)
    }
}

impl ClientMessage {
    pub fn from_bytes(payload: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(payload)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_stdvec(self)
    }
}
