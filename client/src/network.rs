use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::{Action, Motion},
    network::{
        client::ClientMessage,
        server::{ServerMessage, SpawnCharacter},
        NetworkPlugin as BaseNetworkPlugin, NetworkSendEvent, NetworkSendPlugin, NetworkServer,
        Packet, Packetting, SocketEvent,
    },
};

use crate::{character::PlayerState, input_mapping::InputCommand, state::ClientState};

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

fn send_passcode(network_server: Res<NetworkServer>, game_server: Res<GameServer>) {
    // TODO: Handle
    let _ = network_server.send_message(
        game_server.address(),
        &ClientMessage::Passcode(1234),
        Packet::unreliable,
    );
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Send passcode to server on arena enter
        let send_passcode =
            SystemSet::on_enter(ClientState::Arena).with_system(send_passcode.system());

        let broadcast_inputs = SystemSet::on_update(PlayerState::Spawned)
            .with_system(send_input::<Motion>.system())
            .with_system(send_input::<Action>.system());

        let handle_server_message =
            SystemSet::on_update(ClientState::Arena).with_system(handle_server_messages.system());

        app.add_plugin(NetworkSendPlugin::<_, ClientMessage>::new(
            ClientState::Arena,
        ))
        .add_plugin(self.inner.clone())
        .add_system_set(send_passcode)
        .add_system_set(broadcast_inputs)
        .add_system_set(handle_server_message);
    }
}

pub fn handle_server_messages(
    mut network_server: ResMut<NetworkServer>,
    mut spawn_writer: EventWriter<SpawnCharacter>,
) {
    while let Ok(Some(event)) = network_server.recv() {
        match event {
            SocketEvent::Timeout(address) => {
                error!(message = "timeout", %address);
            }
            SocketEvent::Packet(packet) => {
                let payload = packet.payload();
                match ServerMessage::from_bytes(payload) {
                    Ok(message) => match message {
                        ServerMessage::PasscodeAck => {}
                        ServerMessage::SpawnCharacter(spawn) => spawn_writer.send(spawn),
                    },
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

/// Listens to [`InputCommand`] and sends the internal value to the server.
fn send_input<Value>(
    mut input_commands: EventReader<InputCommand<Value>>,
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
            .map(InputCommand::into_inner)
            .map(Into::into)
            .map(|message| {
                info!(message = "sending", ?message, address = %game_server.address);
                NetworkSendEvent::new(message, game_server.address, Packetting::Unreliable)
            }),
    )
}
