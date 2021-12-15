use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    actions::Motion,
    network::{
        client::ClientMessage, server::ServerMessage, NetworkPlugin, NetworkServer, NoSocket,
        Packet, SocketEvent,
    },
};

use crate::state::{GameState, ServerState};

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
        let system_set =
            SystemSet::on_update(ServerState::Running).with_system(handle_client_messages.system());
        app.add_plugin(self.inner.clone())
            .add_system_set(system_set);
    }
}

fn process_passcode(
    address: SocketAddr,
    client_code: u64,
    network_server: &NetworkServer,
    game_state: &mut GameState,
) -> Result<(), NoSocket> {
    if game_state.passcode() == client_code {
        network_server.send_message(address, &ServerMessage::PasscodeAck, Packet::unreliable)?;
    } else {
        // TODO: Send error
    }

    Ok(())
}

fn process_motion(
    motion: Motion,
    network_server: &NetworkServer,
    motion_action: EventReader<CharacterCommand<Motion>>,
) {
}

fn process_packet(packet: Packet, network_server: &NetworkServer, game_state: &mut GameState) {
    match ClientMessage::from_bytes(packet.payload()) {
        Ok(message) => {
            match message {
                ClientMessage::Passcode(client_code) => {
                    let address = packet.addr();
                    // TODO: Handle error
                    let _ = process_passcode(address, client_code, network_server, game_state);
                }
                ClientMessage::Motion(motion) => {}
                _ => (),
            }
            info!(?message);
        }
        Err(error) => {
            error!(%error)
        }
    }
}

fn handle_client_messages(
    mut network_server: ResMut<NetworkServer>,
    mut game_state: ResMut<GameState>,
) {
    while let Ok(Some(event)) = network_server.recv() {
        match event {
            SocketEvent::Timeout(address) => {
                error!(message = "timeout", %address);
            }
            SocketEvent::Packet(packet) => process_packet(packet, &network_server, &mut game_state),
            SocketEvent::Connect(address) => {
                info!(message = "connect", %address);
            }
            SocketEvent::Disconnect(address) => {
                info!(message = "disconnect", %address);
            }
        }
    }
}
