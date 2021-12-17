use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::{Action, CharacterClass, CharacterCommand, CharacterIndex, Motion},
    game::{ArenaPermit, GameRoster},
    network::{
        client::ClientMessage, server::ServerMessage, NetworkPlugin, NetworkSendEvent,
        NetworkSendPlugin, NetworkServer, Packet, Packetting, SocketEvent,
    },
};

use crate::state::ServerState;

pub const NETWORK_HANDLE_LABEL: &str = "network-handle";
pub const NETWORK_SEND_LABEL: &str = "network-send";

fn process_permit(
    client_address: SocketAddr,
    client_permit: &ArenaPermit,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_roster: &mut GameRoster,
) {
    let result = game_roster.apply_permit(client_address, client_permit);

    if result.is_ok() {
        info!("permit passed");
        // Send passcode acknowledgement
        network_writer.send(NetworkSendEvent::new(
            ServerMessage::ArenaPasscodeAck,
            client_address,
            Packetting::Unreliable,
        ));
    } else {
        error!("fraudulent permit");
    }
}

fn process_motion(
    address: &SocketAddr,
    motion: Motion,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_roster: &GameRoster,
    motion_writer: &mut EventWriter<CharacterCommand<Motion>>,
) {
    // if let Some(id) = game_state.id(address) {
    //     let motion_action = CharacterCommand::new(id, motion);
    //     motion_writer.send(motion_action);
    // } else {
    //     // TODO: Handle
    // }
}

fn process_action(
    address: &SocketAddr,
    action: Action,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    action_commands: &mut EventWriter<CharacterCommand<Action>>,
) {
}

fn process_packet(
    packet: Packet,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_roster: &mut GameRoster,
    motion_writer: &mut EventWriter<CharacterCommand<Motion>>,
    action_commands: &mut EventWriter<CharacterCommand<Action>>,
) {
    let address = packet.addr();

    match ClientMessage::from_bytes(packet.payload()) {
        Ok(message) => {
            info!(message = "received message", ?message, %address);

            match message {
                ClientMessage::Permit(permit) => {
                    process_permit(address, &permit, network_writer, game_roster)
                }
                ClientMessage::Motion(motion) => {
                    // process_motion(&address, motion, network_writer, game_state, motion_writer)
                }
                ClientMessage::Action(action) => {
                    //     process_action(
                    //     &address,
                    //     action,
                    //     network_writer,
                    //     game_state,
                    //     action_commands,
                    // )
                }
                _ => (),
            }
            // info!(message = "received message", ?message);
        }
        Err(error) => error!(message = "failed to parse packet", %error),
    }
}

fn handle_client_messages(
    mut network_server: ResMut<NetworkServer>,
    mut network_writer: EventWriter<NetworkSendEvent<ServerMessage>>,
    mut game_roster: ResMut<GameRoster>,
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
                &mut game_roster,
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
        let system_set = SystemSet::on_update(ServerState::Running)
            .label(NETWORK_HANDLE_LABEL)
            .before(NETWORK_SEND_LABEL)
            .with_system(handle_client_messages.system());
        app.add_plugin(NetworkSendPlugin::<_, ServerMessage>::new(
            ServerState::Running,
            NETWORK_SEND_LABEL,
        ))
        .add_plugin(self.inner.clone())
        .add_system_set(system_set);
    }
}
