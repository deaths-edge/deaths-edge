use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::{Action, CharacterClass, CharacterCommand, CharacterIndex, Motion},
    network::{
        client::ClientMessage,
        server::{ServerMessage, SpawnCharacter},
        NetworkPlugin, NetworkSendEvent, NetworkSendPlugin, NetworkServer, Packet, Packetting,
        SocketEvent,
    },
};

use crate::state::{GameState, ServerState};

fn process_passcode(
    client_address: SocketAddr,
    client_code: u64,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_state: &mut GameState,
) {
    if game_state.passcode() == client_code {
        // Send passcode acknowledgement
        network_writer.send(NetworkSendEvent::new(
            ServerMessage::PasscodeAck,
            client_address,
            Packetting::Unreliable,
        ));

        // Send spawn
        let spawn_char = SpawnCharacter::new(
            CharacterIndex::from(0),
            CharacterClass::Medea,
            true,
            Vec2::new(0., 0.),
        );
        network_writer.send(NetworkSendEvent::new(
            ServerMessage::SpawnCharacter(spawn_char),
            client_address,
            Packetting::Unreliable,
        ));
    } else {
        // TODO: Send error
    }
}

fn process_motion(
    address: &SocketAddr,
    motion: Motion,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_state: &GameState,
    motion_writer: &mut EventWriter<CharacterCommand<Motion>>,
) {
    if let Some(id) = game_state.id(address) {
        let motion_action = CharacterCommand::new(id, motion);
        motion_writer.send(motion_action);
    } else {
        // TODO: Handle
    }
}

fn process_action(
    address: &SocketAddr,
    action: Action,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_state: &GameState,
    action_commands: &mut EventWriter<CharacterCommand<Action>>,
) {
}

fn process_packet(
    packet: Packet,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_state: &mut GameState,
    motion_writer: &mut EventWriter<CharacterCommand<Motion>>,
    action_commands: &mut EventWriter<CharacterCommand<Action>>,
) {
    let address = packet.addr();

    match ClientMessage::from_bytes(packet.payload()) {
        Ok(message) => {
            match message {
                ClientMessage::Passcode(client_code) => {
                    process_passcode(address, client_code, network_writer, game_state)
                }
                ClientMessage::Motion(motion) => {
                    process_motion(&address, motion, network_writer, game_state, motion_writer)
                }
                ClientMessage::Action(action) => process_action(
                    &address,
                    action,
                    network_writer,
                    game_state,
                    action_commands,
                ),
                _ => (),
            }
            info!(message = "received message", ?message);
        }
        Err(error) => error!(message = "failed to parse packet", %error),
    }
}

fn handle_client_messages(
    mut network_server: ResMut<NetworkServer>,
    mut network_writer: EventWriter<NetworkSendEvent<ServerMessage>>,
    mut game_state: ResMut<GameState>,
    mut motion_writer: EventWriter<CharacterCommand<Motion>>,
    mut action_commands: EventWriter<CharacterCommand<Action>>,
) {
    while let Ok(Some(event)) = network_server.recv() {
        match event {
            SocketEvent::Timeout(address) => {
                error!(message = "timeout", %address);
            }
            SocketEvent::Packet(packet) => process_packet(
                packet,
                &mut network_writer,
                &mut game_state,
                &mut motion_writer,
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
        app.add_plugin(NetworkSendPlugin::<_, ServerMessage>::new(
            ServerState::Running,
        ))
        .add_plugin(self.inner.clone())
        .add_system_set(system_set);
    }
}
