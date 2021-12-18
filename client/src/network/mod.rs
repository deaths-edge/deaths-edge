mod character_command;

use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::{Action, CharacterClass, CharacterTeam, Motion},
    game::{ArenaPasscode, ArenaPermit},
    network::{
        client::ClientMessage,
        server::{CharacterCommand, GameCommand, ServerMessage, SpawnCharacter},
        CharacterNetworkCommand, NetworkPlugin as BaseNetworkPlugin, NetworkSendEvent,
        NetworkSendPlugin, NetworkServer, Packet, Packetting, SocketEvent,
    },
};

use crate::{character::PlayerState, input_mapping::PlayerInputCommand, state::ClientState, Opt};

use character_command::*;

pub const NETWORK_HANDLE_LABEL: &str = "network-handle";
pub const NETWORK_SEND_LABEL: &str = "network-send";

pub struct GameServer {
    address: SocketAddr,
}

impl GameServer {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }
}

fn request_arena_entry(
    network_server: Res<NetworkServer>,
    game_server: Res<GameServer>,
    opts: Res<Opt>,
) {
    // TODO: Handle
    let _ = network_server.send_message(
        game_server.address(),
        &ClientMessage::Permit(ArenaPermit::new(
            ArenaPasscode(opts.passcode),
            CharacterClass::Medea,
            CharacterTeam::Red,
        )),
        Packet::unreliable,
    );
}

pub fn handle_server_messages(
    mut network_server: ResMut<NetworkServer>,
    mut spawn_writer: EventWriter<SpawnCharacter>,
    mut motion_writer: EventWriter<CharacterNetworkCommand<Motion>>,
) {
    while let Ok(Some(event)) = network_server.recv() {
        match event {
            SocketEvent::Timeout(address) => {
                error!(message = "timeout", %address);
            }
            SocketEvent::Packet(packet) => {
                let payload = packet.payload();
                let address = packet.addr();
                match ServerMessage::from_bytes(payload) {
                    Ok(message) => {
                        info!(message = "received message", ?message, %address);
                        match message {
                            ServerMessage::ArenaPasscodeAck => {}
                            ServerMessage::GameCommand(command) => match command {
                                GameCommand::SpawnCharacter(spawn) => spawn_writer.send(spawn),
                            },
                            ServerMessage::CharacterCommand(command) => match command {
                                CharacterCommand::Motion(motion) => motion_writer.send(motion),
                                CharacterCommand::Action(_) => todo!(),
                            },
                        }
                    }
                    Err(error) => error!(message = "failed to parse packet", %error),
                }
                // match
                // spawn_writer.send(event)
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

/// Listens to [`PlayerInputCommand`] and sends the internal value to the server.
fn send_input<Value>(
    mut input_commands: EventReader<PlayerInputCommand<Value>>,
    mut send_events: EventWriter<NetworkSendEvent<ClientMessage>>,
    game_server: Res<GameServer>,
) where
    Value: Clone + Send + Sync + 'static,
    Value: Into<ClientMessage>,
{
    send_events.send_batch(
        input_commands
            .iter()
            .cloned()
            .map(PlayerInputCommand::into_inner)
            .map(Into::into)
            .map(|message| {
                info!(message = "sending", ?message, address = %game_server.address);
                NetworkSendEvent::new(message, game_server.address, Packetting::Unreliable)
            }),
    )
}

pub struct NetworkPlugin {
    inner: BaseNetworkPlugin,
}

impl NetworkPlugin {
    pub fn new(address: SocketAddr, poll_interval: Duration) -> Self {
        Self {
            inner: BaseNetworkPlugin::new(address, poll_interval),
        }
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Request entry to arena
        // TODO: Do this in lobby
        let send_passcode =
            SystemSet::on_enter(ClientState::Arena).with_system(request_arena_entry.system());

        let handle_server_message = SystemSet::on_update(ClientState::Arena)
            .label(NETWORK_HANDLE_LABEL)
            .with_system(handle_server_messages.system());

        let broadcast_inputs = SystemSet::on_update(PlayerState::Spawned)
            .with_system(send_input::<Motion>.system())
            .with_system(send_input::<Action>.system());

        let network_to_entity = SystemSet::on_update(ClientState::Arena)
            .after(NETWORK_HANDLE_LABEL)
            .with_system(network_to_entity_command::<Motion>.system());

        app.add_plugin(NetworkSendPlugin::<_, ClientMessage>::new(
            ClientState::Arena,
            NETWORK_SEND_LABEL,
        ))
        .add_event::<CharacterNetworkCommand<Motion>>()
        .add_plugin(self.inner.clone())
        .add_system_set(send_passcode)
        .add_system_set(broadcast_inputs)
        .add_system_set(network_to_entity)
        .add_system_set(handle_server_message);
    }
}
