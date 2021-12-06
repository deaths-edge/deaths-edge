use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct CharacterPower {
    pub current: u32,
    pub total: u32,
}
