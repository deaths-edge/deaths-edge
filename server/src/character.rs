use bevy::prelude::*;

use common::network::ConnectionHandle;

#[derive(Debug, Component)]
pub struct ClientAddress(pub ConnectionHandle);

#[derive(Bundle)]
pub struct ServerCharacterBundle {
    pub address: ClientAddress,
}
