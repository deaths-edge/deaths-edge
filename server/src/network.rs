use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::network::{client::ClientMessage, NetworkPlugin, NetworkServer, SocketEvent};

pub struct NetworkServerPlugin {
    inner: NetworkPlugin,
}

impl NetworkServerPlugin {
    pub fn new(address: SocketAddr, poll_interval: Duration) -> Self {
        Self {
            inner: NetworkPlugin::new(address, poll_interval),
        }
    }
}

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(self.inner.clone())
            .add_system(handle_messages.system());
    }
}

fn handle_messages(mut network_server: ResMut<NetworkServer>) {
    while let Ok(Some(event)) = network_server.recv() {
        match event {
            SocketEvent::Timeout(address) => {
                error!(message = "timeout", %address);
            }
            SocketEvent::Packet(packet) => match ClientMessage::from_bytes(packet.payload()) {
                Ok(message) => {
                    info!(?message);
                }
                Err(error) => {
                    error!(%error)
                }
            },
            SocketEvent::Connect(address) => {
                info!(message = "connect", %address);
            }
            SocketEvent::Disconnect(address) => {
                info!(message = "disconnect", %address);
            }
        }
    }
}
