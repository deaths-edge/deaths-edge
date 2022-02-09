use bevy::prelude::*;

use common::network::ClientAddress;

#[derive(Bundle)]
pub struct ServerCharacterBundle {
    pub address: ClientAddress,
}
