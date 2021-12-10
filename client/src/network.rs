use std::{net::SocketAddr, time::Duration};

use bevy::{app::Events, prelude::*};

use common::{
    actions::Motion,
    network::{
        client::ClientMessage, message_broadcast, NetworkPlugin as BaseNetworkPlugin,
        NetworkSendEvent, Packetting,
    },
};

use crate::state::ClientState;

pub struct GameServer {
    address: SocketAddr,
}

impl GameServer {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
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

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // let connect = SystemSet::on_exit(ClientState::Arena).with_system(connect_server.system());
        let broadcast_movement =
            SystemSet::on_update(ClientState::Arena).with_system(broadcast_movement.system());

        app.add_plugin(self.inner.clone())
            .add_event::<NetworkSendEvent<ClientMessage>>()
            .add_system(message_broadcast::<ClientMessage>.system())
            .add_system_set(broadcast_movement);
    }
}

fn broadcast_movement(
    mut motion_events: EventReader<Motion>,
    mut send_events: EventWriter<NetworkSendEvent<ClientMessage>>,
    game_server: Res<GameServer>,
) {
    send_events.send_batch(
        motion_events
            .iter()
            .cloned()
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
