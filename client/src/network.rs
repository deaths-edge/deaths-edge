use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{
    character::Motion,
    network::{
        client::ClientMessage, message_broadcast, NetworkPlugin as BaseNetworkPlugin,
        NetworkSendEvent, NetworkServer, Packet, Packetting,
    },
};

use crate::{input_mapping::InputCommand, state::ClientState};

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
        let broadcast_movement =
            SystemSet::on_update(ClientState::Arena).with_system(broadcast_movement.system());

        app.add_plugin(self.inner.clone())
            .add_event::<NetworkSendEvent<ClientMessage>>()
            .add_system(message_broadcast::<ClientMessage>.system())
            .add_system_set(send_passcode)
            .add_system_set(broadcast_movement);
    }
}

fn broadcast_movement(
    mut motion_events: EventReader<InputCommand<Motion>>,
    mut send_events: EventWriter<NetworkSendEvent<ClientMessage>>,
    game_server: Res<GameServer>,
) {
    send_events.send_batch(
        motion_events
            .iter()
            .cloned()
            .map(InputCommand::into_inner)
            .map(ClientMessage::Motion)
            .map(|message| {
                info!(message = "sending", ?message, address = %game_server.address);
                NetworkSendEvent {
                    message,
                    address: game_server.address,
                    packetting: Packetting::Unreliable,
                }
            }),
    )
}
