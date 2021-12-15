mod materials;
mod player;

use bevy::prelude::*;

pub use materials::*;
pub use player::*;

use common::character::{CastingPlugin, CharacterBundle as CommonCharacterBundle, CharacterClass};

use crate::{state::ClientState, ui::selected::Selected};
// input_mapping::{FocalHold, MotionKey, SelectClick},

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    common: CommonCharacterBundle,
    selected: Selected,
}

impl CharacterBundle {
    pub fn new(
        transform: Transform,
        common: CommonCharacterBundle,
        materials: &CharacterMaterials,
    ) -> Self {
        let size = common.class().size();

        Self {
            sprite: SpriteBundle {
                material: materials.handle(common.class()).clone(),
                transform,
                sprite: Sprite::new(Vec2::new(size.width, size.width)),
                ..Default::default()
            },
            common,
            selected: Selected::default(),
        }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CharacterMaterialPlugin)
            .add_plugin(PlayerPlugin::new(ClientState::Arena))
            .add_plugin(CastingPlugin::new(ClientState::Arena));
    }
}
