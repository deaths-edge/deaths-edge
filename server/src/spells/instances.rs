use bevy::prelude::*;

use common::spells::instances::FireballBundle as CommonFireballBundle;

#[derive(Bundle)]
pub struct FireballBundle {
    #[bundle]
    pub common: CommonFireballBundle,
    pub transform: Transform,
    global_transform: GlobalTransform,
}

impl FireballBundle {
    pub fn new(common: CommonFireballBundle, transform: Transform) -> Self {
        Self {
            common,
            transform,
            global_transform: GlobalTransform::default(),
        }
    }
}
