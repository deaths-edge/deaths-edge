use std::{net::SocketAddr, ops::Deref};

use bevy::prelude::*;

use common::character::CharacterBundle as CommonCharacterBundle;

pub struct ClientAddress(pub SocketAddr);

impl From<SocketAddr> for ClientAddress {
    fn from(value: SocketAddr) -> Self {
        Self(value)
    }
}

impl Deref for ClientAddress {
    type Target = SocketAddr;

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
