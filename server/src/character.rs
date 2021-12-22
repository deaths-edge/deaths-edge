use std::ops::Deref;

use bevy::prelude::*;

use common::{character::CharacterBundle as CommonCharacterBundle, network::ConnectionHandle};

pub struct ClientAddress(pub ConnectionHandle);

impl From<ConnectionHandle> for ClientAddress {
    fn from(value: ConnectionHandle) -> Self {
        Self(value)
    }
}

impl Deref for ClientAddress {
    type Target = ConnectionHandle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
