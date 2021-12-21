use std::net::SocketAddr;

use bevy::{core::FixedTimestep, prelude::*};

use common::{
    character::{
        Action, CharacterEntityCommand, CharacterIndex, CharacterMarker, FocalAngle, Motion,
        CHARACTER_COMMANDS,
    },
    game::{ArenaPermit, GameRoster},
    network::{
        client::{ClientCommand, ClientMessage},
        server::{CharacterCommand, Reconcile, ServerMessage},
        CharacterNetworkCommand, NetworkPlugin, NetworkSendEvent, NetworkSendPlugin, NetworkServer,
        Packet, Packetting, SocketEvent, NETWORK_POLL_LABEL,
    },
};

use crate::{character::ClientAddress, state::ServerState};

pub const NETWORK_HANDLE_LABEL: &str = "network-handle";
pub const NETWORK_RELAY_LABEL: &str = "network-relay";
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
            Packetting::ReliableOrdered,
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
    focal_writer: &mut EventWriter<CharacterEntityCommand<FocalAngle>>,

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
                ClientMessage::Command(command) => {
                    let address = packet.addr();
                    let result = match command {
                        ClientCommand::Motion(motion) => {
                            process_command(address, char_query_iter, motion, motion_writer)
                        }
                        ClientCommand::Action(action) => {
                            process_command(address, char_query_iter, action, action_writer)
                        }
                        ClientCommand::Rotate(rotate) => {
                            process_command(address, char_query_iter, rotate, focal_writer)
                        }
                    };
                    if let Err(_) = result {
                        error!("not found");
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
    mut focal_commands: EventWriter<CharacterEntityCommand<FocalAngle>>,
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
                &mut focal_commands,
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
                    NetworkSendEvent::new(message, addr.0, Packetting::ReliableOrdered)
                })
        })
        .flatten();
    network_writer.send_batch(events)
}

pub fn reconcile_broadcast(
    mut network_writer: EventWriter<NetworkSendEvent<ServerMessage>>,
    character_query: Query<(&CharacterIndex, &Transform), With<CharacterMarker>>,
    character_address_query: Query<&ClientAddress, With<CharacterMarker>>,
) {
    let events = character_query
        .iter()
        .map(|(index, transform)| {
            let reconcile = Reconcile {
                index: *index,
                position: transform.translation.truncate(),
            };
            let message = ServerMessage::Reconcile(reconcile);

            character_address_query.iter().map(move |address| {
                NetworkSendEvent::new(message.clone(), **address, Packetting::ReliableOrdered)
            })
        })
        .flatten();

    network_writer.send_batch(events);
}

pub struct NetworkServerPlugin {
    inner: NetworkPlugin,
}

impl NetworkServerPlugin {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            inner: NetworkPlugin::new(address),
        }
    }
}

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::on_update(ServerState::Running)
            .label(NETWORK_HANDLE_LABEL)
            .after(NETWORK_POLL_LABEL)
            .before(CHARACTER_COMMANDS)
            .before(NETWORK_SEND_LABEL)
            .with_system(handle_client_messages.system());

        let relay_commands = SystemSet::on_update(ServerState::Running)
            .label(NETWORK_RELAY_LABEL)
            .after(NETWORK_HANDLE_LABEL)
            .after(CHARACTER_COMMANDS)
            .after(NETWORK_POLL_LABEL)
            .before(NETWORK_SEND_LABEL)
            .with_system(relay_character_commands::<Motion>.system())
            .with_system(relay_character_commands::<FocalAngle>.system());

        let broadcast_reconciles = SystemSet::on_update(ServerState::Running)
            .with_run_criteria(FixedTimestep::step(2.0))
            .after(NETWORK_POLL_LABEL)
            .before(NETWORK_SEND_LABEL)
            .with_system(reconcile_broadcast.system());

        app.add_plugin(NetworkSendPlugin::<_, ServerMessage>::new(
            ServerState::Running,
            NETWORK_SEND_LABEL,
        ))
        .add_plugin(self.inner.clone())
        .add_system_set(system_set)
        .add_system_set(relay_commands)
        .add_system_set(broadcast_reconciles);
    }
}
