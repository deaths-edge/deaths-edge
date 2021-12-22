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
        find_my_ip_address, network_setup,
        server::{CharacterCommand, Reconcile, ServerMessage},
        CharacterNetworkCommand, ConnectionHandle, NetworkEvent, NetworkResource, NetworkingPlugin,
        NETWORK_SETUP_LABEL, SERVER_PORT,
    },
};

use crate::{character::ClientAddress, state::ServerState};

pub const NETWORK_HANDLE_LABEL: &str = "network-handle";
pub const NETWORK_RELAY_LABEL: &str = "network-relay";

fn process_permit(
    connection_handle: ConnectionHandle,
    client_permit: &ArenaPermit,

    game_roster: &mut GameRoster,

    to_send: &mut Vec<(ConnectionHandle, ServerMessage)>,
) {
    let result = game_roster.apply_permit(connection_handle, client_permit);

    if result.is_ok() {
        info!("permit passed");
        // Send passcode acknowledgement
        to_send.push((connection_handle, ServerMessage::ArenaPasscodeAck));
    } else {
        error!("fraudulent permit");
    }
}

pub struct CharacterNotFound;

fn process_command<'a, T>(
    connection_handle: ConnectionHandle,
    mut char_query_iter: impl Iterator<Item = (Entity, &'a ClientAddress)>,
    command: T,
    command_writer: &mut EventWriter<CharacterEntityCommand<T>>,
) -> Result<(), CharacterNotFound>
where
    T: Send + Sync + std::fmt::Debug + 'static,
{
    info!(message = "sending entity", ?command);

    let id = char_query_iter
        .find(|(_, addr)| ***addr == connection_handle)
        .map(|(id, _)| id)
        .ok_or(CharacterNotFound)?;
    let motion_action = CharacterEntityCommand::new(id, command);
    command_writer.send(motion_action);
    Ok(())
}

fn process_message<'a>(
    connection_handle: ConnectionHandle,
    client_message: ClientMessage,

    game_roster: &mut GameRoster,

    to_send: &mut Vec<(ConnectionHandle, ServerMessage)>,

    motion_writer: &mut EventWriter<CharacterEntityCommand<Motion>>,
    action_writer: &mut EventWriter<CharacterEntityCommand<Action>>,
    focal_writer: &mut EventWriter<CharacterEntityCommand<FocalAngle>>,

    char_query_iter: impl Iterator<Item = (Entity, &'a ClientAddress)>,
) {
    match client_message {
        ClientMessage::Permit(permit) => {
            process_permit(connection_handle, &permit, game_roster, to_send)
        }
        ClientMessage::Command(command) => {
            let result = match command {
                ClientCommand::Motion(motion) => {
                    process_command(connection_handle, char_query_iter, motion, motion_writer)
                }
                ClientCommand::Action(action) => {
                    process_command(connection_handle, char_query_iter, action, action_writer)
                }
                ClientCommand::Rotate(rotate) => {
                    process_command(connection_handle, char_query_iter, rotate, focal_writer)
                }
            };
            if let Err(_) = result {
                error!("not found");
            }
        }
    }
}

fn handle_connects(
    mut net: ResMut<NetworkResource>,
    mut network_reader: EventReader<NetworkEvent>,
) {
    for event in network_reader.iter() {
        info!(message = "received network event", ?event);
        match event {
            NetworkEvent::Error(handle, error) => {
                error!(message = "timeout", %handle, ?error);
            }
            NetworkEvent::Connected(handle) => {
                net.send_message(*handle, ServerMessage::ArenaPasscodeAck)
                    .expect("failed to send ArenaPasscodeAck");
                let addr = net.connections.get(handle).unwrap();

                info!(addr = ?addr.remote_address(), stats = ?addr.stats());
                info!(message = "connected", %handle);
            }
            NetworkEvent::Disconnected(handle) => {
                info!(message = "disconnected", %handle);
            }
            _ => (),
        }
    }
}

fn handle_client_messages(
    // mut network_server: ResMut<NetworkServer>,
    mut net: ResMut<NetworkResource>,

    mut game_roster: ResMut<GameRoster>,
    char_query: Query<(Entity, &ClientAddress), With<CharacterMarker>>,

    mut motion_writer: EventWriter<CharacterEntityCommand<Motion>>,
    mut action_commands: EventWriter<CharacterEntityCommand<Action>>,
    mut focal_commands: EventWriter<CharacterEntityCommand<FocalAngle>>,
) {
    let mut to_send = Vec::new();

    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ClientMessage>() {
            process_message(
                *handle,
                client_message,
                &mut game_roster,
                &mut to_send,
                &mut motion_writer,
                &mut action_commands,
                &mut focal_commands,
                char_query.iter(),
            )
        }
    }

    for (handle, message) in to_send.into_iter() {
        net.send_message(handle, message)
            .expect("failed to send message");
    }
}

pub fn relay_character_commands<T>(
    character_index_query: Query<&CharacterIndex, With<CharacterMarker>>,
    character_address_query: Query<
        (Entity, &CharacterIndex, &ClientAddress),
        With<CharacterMarker>,
    >,

    mut character_entity_reader: EventReader<CharacterEntityCommand<T>>,

    mut net: ResMut<NetworkResource>,
) where
    T: Send + Sync + 'static,
    T: Clone + std::fmt::Debug,
    CharacterNetworkCommand<T>: Into<CharacterCommand>,
{
    for command in character_entity_reader.iter() {
        info!(message = "relaying", command = ?command.command());
        let source_id = command.id();
        let index = character_index_query
            .get(source_id)
            .expect("failed to find character");

        let iter = character_address_query
            .iter()
            .filter(move |(id, _, _)| source_id != *id);

        for (_, _, addr) in iter {
            info!(message = "relaying", command = ?command.command(), address = %addr.0);

            let network_command = CharacterNetworkCommand {
                index: *index,
                command: command.command().clone(),
            };
            let message = ServerMessage::CharacterCommand(network_command.into());
            net.send_message(**addr, message)
                .expect("failed to send CharacterCommand");
        }
    }
}

pub fn reconcile_broadcast(
    mut net: ResMut<NetworkResource>,
    character_query: Query<(&CharacterIndex, &Transform), With<CharacterMarker>>,
) {
    for (index, transform) in character_query.iter() {
        let reconcile = Reconcile {
            index: *index,
            position: transform.translation.truncate(),
        };
        let message = ServerMessage::Reconcile(reconcile);

        net.broadcast_message(message.clone());
    }
}

pub fn startup(mut net: ResMut<NetworkResource>) {
    let ip_address = find_my_ip_address().expect("can't find ip address");
    let address = SocketAddr::new(ip_address, SERVER_PORT);

    // let address: SocketAddr = "127.0.0.1:8000".parse().expect("failed to parse address");
    net.listen(address, None, None);
}

pub struct NetworkServerPlugin;

impl Plugin for NetworkServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let setup = SystemSet::on_enter(ServerState::Running)
            .label(NETWORK_SETUP_LABEL)
            .with_system(startup.system())
            .with_system(network_setup.system());

        let handle_client = SystemSet::on_update(ServerState::Running)
            .label(NETWORK_HANDLE_LABEL)
            // CHARACTER_COMMANDS reads CharacterEntityCommand<Value> events
            .before(CHARACTER_COMMANDS)
            .with_system(handle_client_messages.system())
            .with_system(handle_connects.system());

        let relay_commands = SystemSet::on_update(ServerState::Running)
            .label(NETWORK_RELAY_LABEL)
            // NETWORK_HANDLE_LABEL writes CharacterEntityCommand<Value> events
            .after(NETWORK_HANDLE_LABEL)
            .with_system(relay_character_commands::<Motion>.system())
            .with_system(relay_character_commands::<Action>.system())
            .with_system(relay_character_commands::<FocalAngle>.system());

        let broadcast_reconciles = SystemSet::on_update(ServerState::Running)
            .with_run_criteria(FixedTimestep::step(5.0))
            .with_system(reconcile_broadcast.system());

        app.add_plugin(NetworkingPlugin::default())
            .add_system_set(setup)
            .add_system_set(handle_client)
            .add_system_set(relay_commands)
            .add_system_set(broadcast_reconciles);
    }
}
