use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;
use laminar::SocketEvent;

use super::{client::ClientMessage, NetworkPlugin, NetworkServer};

pub enum ServerMessage {}

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

fn handle_messages(network_server: Res<NetworkServer>) {
    let receiver = network_server.receiver().expect("server uninitialized");

    for event in receiver {
        match event {
            SocketEvent::Timeout(address) => {
                error!(message = "timeout", %address);
            }
            SocketEvent::Packet(packet) => {
                match postcard::from_bytes::<ClientMessage>(packet.payload()) {
                    Ok(message) => {
                        info!(?message);
                    }
                    Err(error) => {
                        error!(%error)
                    }
                }
            }
            SocketEvent::Connect(address) => {
                info!(message = "connect", %address);
            }
            SocketEvent::Disconnect(address) => {
                info!(message = "disconnect", %address);
            }
        }
    }
}
