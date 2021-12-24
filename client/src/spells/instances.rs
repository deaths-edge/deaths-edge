use bevy::prelude::*;

use common::spells::instances::FireballBundle as CommonFireballBundle;

#[derive(Bundle)]
pub struct FireballBundle {
    #[bundle]
    common: CommonFireballBundle,
    #[bundle]
    sprite: SpriteBundle,
}
