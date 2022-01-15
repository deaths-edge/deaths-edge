use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// An index which is stable across clients.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Component)]
pub struct CharacterIndex(pub u8);

impl CharacterIndex {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
