use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::{Action, Motion},
    network::{
        client::ClientMessage, NetworkPlugin as BaseNetworkPlugin, NetworkSendEvent,
        NetworkSendPlugin, NetworkServer, Packet, Packetting,
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
        let send_passcode =
            SystemSet::on_enter(ClientState::Arena).with_system(send_passcode.system());
        let broadcast = SystemSet::on_update(PlayerState::Spawned)
            .with_system(send_input::<Motion>.system())
            .with_system(send_input::<Action>.system());

        app.add_plugin(NetworkSendPlugin::<_, ClientMessage>::new(
            ClientState::Arena,
        ))
        .add_plugin(self.inner.clone())
        .add_system_set(send_passcode)
        .add_system_set(broadcast);
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
