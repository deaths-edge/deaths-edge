use bevy::prelude::Bundle;

use common::character::CharacterBundle as CommonCharacterBundle;

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    common: CommonCharacterBundle,
}
