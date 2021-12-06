use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct CharacterIndex(usize);

impl From<usize> for CharacterIndex {
    fn from(val: usize) -> Self {
        Self(val)
    }
}
