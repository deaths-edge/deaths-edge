use bevy::prelude::*;

use common::{character::CharacterBundle as CommonCharacterBundle, network::ConnectionHandle};

pub struct ClientAddress(pub ConnectionHandle);

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    common: CommonCharacterBundle,
    transform: Transform,
    global_transform: GlobalTransform,
    address: ClientAddress,
}

impl CharacterBundle {
    pub fn new(
        transform: Transform,
        common: CommonCharacterBundle,
        address: ClientAddress,
    ) -> Self {
        Self {
            common,
            transform,
            global_transform: GlobalTransform::default(),
            address,
        }
    }
}
