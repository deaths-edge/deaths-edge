use bevy::prelude::*;

use common::character::CharacterBundle as CommonCharacterBundle;

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    common: CommonCharacterBundle,
    transform: Transform,
}

impl CharacterBundle {
    pub fn new(transform: Transform, common: CommonCharacterBundle) -> Self {
        Self { common, transform }
    }
}
