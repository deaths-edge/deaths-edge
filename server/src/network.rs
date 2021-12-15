use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::{Action, CharacterCommand, Motion},
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
    address: &SocketAddr,
    motion: Motion,
    network_server: &NetworkServer,
    game_state: &GameState,
    motion_commands: &mut EventWriter<CharacterCommand<Motion>>,
) {
    if let Some(id) = game_state.id(address) {
        let motion_action = CharacterCommand::new(id, motion);
        motion_commands.send(motion_action);
    } else {
        // TODO: Handle
    }
}

fn process_action(
    address: &SocketAddr,
    action: Action,
    network_server: &NetworkServer,
    game_state: &GameState,
    action_commands: &mut EventWriter<CharacterCommand<Action>>,
) {
}

fn process_packet(
    packet: Packet,
    network_server: &NetworkServer,
    game_state: &mut GameState,
    motion_commands: &mut EventWriter<CharacterCommand<Motion>>,
    action_commands: &mut EventWriter<CharacterCommand<Action>>,
) {
    let address = packet.addr();

    match ClientMessage::from_bytes(packet.payload()) {
        Ok(message) => {
            match message {
                ClientMessage::Passcode(client_code) => {
                    // TODO: Handle error
                    let _ = process_passcode(address, client_code, network_server, game_state);
                }
                ClientMessage::Motion(motion) => process_motion(
                    &address,
                    motion,
                    network_server,
                    game_state,
                    motion_commands,
                ),
                ClientMessage::Action(action) => process_action(
                    &address,
                    action,
                    network_server,
                    game_state,
                    action_commands,
                ),
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
    mut motion_commands: EventWriter<CharacterCommand<Motion>>,
    mut action_commands: EventWriter<CharacterCommand<Action>>,
) {
    while let Ok(Some(event)) = network_server.recv() {
        match event {
            SocketEvent::Timeout(address) => {
                error!(message = "timeout", %address);
            }
            SocketEvent::Packet(packet) => process_packet(
                packet,
                &network_server,
                &mut game_state,
                &mut motion_commands,
                &mut action_commands,
            ),
            SocketEvent::Connect(address) => {
                info!(message = "connect", %address);
            }
            SocketEvent::Disconnect(address) => {
                info!(message = "disconnect", %address);
            }
        }
    }
}
