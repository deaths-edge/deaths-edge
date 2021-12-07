use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CharacterIndex(u32);

impl From<u32> for CharacterIndex {
    fn from(val: u32) -> Self {
        Self(val)
    }
}
