use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::{Action, CharacterEntityCommand, CharacterIndex, CharacterMarker, Motion},
    game::{ArenaPermit, GameRoster},
    network::{
        client::ClientMessage,
        server::{CharacterCommand, ServerMessage},
        CharacterNetworkCommand, NetworkPlugin, NetworkSendEvent, NetworkSendPlugin, NetworkServer,
        Packet, Packetting, SocketEvent,
    },
};

use crate::{character::ClientAddress, state::ServerState};

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

pub struct CharacterNotFound;

fn process_command<'a, T>(
    address: SocketAddr,
    mut char_query_iter: impl Iterator<Item = (Entity, &'a ClientAddress)>,
    command: T,
    command_writer: &mut EventWriter<CharacterEntityCommand<T>>,
) -> Result<(), CharacterNotFound>
where
    T: Send + Sync + std::fmt::Debug + 'static,
{
    info!(message = "sending entity", ?command);

    let id = char_query_iter
        .find(|(_, addr)| ***addr == address)
        .map(|(id, _)| id)
        .ok_or(CharacterNotFound)?;
    let motion_action = CharacterEntityCommand::new(id, command);
    command_writer.send(motion_action);
    Ok(())
}

fn process_packet<'a>(
    packet: Packet,
    network_writer: &mut EventWriter<NetworkSendEvent<ServerMessage>>,
    game_roster: &mut GameRoster,

    motion_writer: &mut EventWriter<CharacterEntityCommand<Motion>>,
    action_writer: &mut EventWriter<CharacterEntityCommand<Action>>,

    char_query_iter: impl Iterator<Item = (Entity, &'a ClientAddress)>,
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
                    if let Err(_) =
                        process_command(packet.addr(), char_query_iter, motion, motion_writer)
                    {
                        error!("received motion from an unknown character");
                    }
                }
                ClientMessage::Action(action) => {
                    if let Err(_) =
                        process_command(packet.addr(), char_query_iter, action, action_writer)
                    {
                        error!("received action from an unknown character");
                    }
                }
            }
        }
        Err(error) => error!(message = "failed to parse packet", %error),
    }
}

fn handle_client_messages(
    mut network_server: ResMut<NetworkServer>,
    mut network_writer: EventWriter<NetworkSendEvent<ServerMessage>>,

    mut game_roster: ResMut<GameRoster>,
    char_query: Query<(Entity, &ClientAddress), With<CharacterMarker>>,

    mut motion_writer: EventWriter<CharacterEntityCommand<Motion>>,
    mut action_commands: EventWriter<CharacterEntityCommand<Action>>,
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
                char_query.iter(),
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

pub fn relay_character_commands<T>(
    character_index_query: Query<&CharacterIndex, With<CharacterMarker>>,
    character_address_query: Query<(Entity, &ClientAddress), With<CharacterMarker>>,

    mut character_entity_reader: EventReader<CharacterEntityCommand<T>>,
    mut network_writer: EventWriter<NetworkSendEvent<ServerMessage>>,
) where
    T: Send + Sync + 'static,
    T: Clone + std::fmt::Debug,
    CharacterNetworkCommand<T>: Into<CharacterCommand>,
{
    let events = character_entity_reader
        .iter()
        .map(|command| {
            info!(message = "relaying", command = ?command.command());
            let source_id = command.id();
            let index = character_index_query
                .get(source_id)
                .expect("failed to find character");
            character_address_query
                .iter()
                .filter(move |(id, _)| source_id != *id)
                .map(|(_, addr)| {
                    info!(message = "relaying", command = ?command.command(), address = %addr.0);

                    let network_command = CharacterNetworkCommand {
                        index: *index,
                        command: command.command().clone(),
                    };
                    let message = ServerMessage::CharacterCommand(network_command.into());
                    NetworkSendEvent::new(message, addr.0, Packetting::Unreliable)
                })
        })
        .flatten();
    network_writer.send_batch(events)
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

        let relay_commands = SystemSet::on_update(ServerState::Running)
            .after(NETWORK_HANDLE_LABEL)
            .with_system(relay_character_commands::<Motion>.system());

        app.add_plugin(NetworkSendPlugin::<_, ServerMessage>::new(
            ServerState::Running,
            NETWORK_SEND_LABEL,
        ))
        .add_plugin(self.inner.clone())
        .add_system_set(system_set)
        .add_system_set(relay_commands);
    }
}
