use bevy::prelude::*;

use common::{character::CharacterBundle, network::ConnectionHandle};

pub struct ClientAddress(pub ConnectionHandle);

#[derive(Bundle)]
pub struct ServerCharacterBundle {
    address: ClientAddress,
}

impl ServerCharacterBundle {
    pub fn new(address: ClientAddress) -> Self {
        Self { address }
    }
}
