use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::network::{
    client::ClientMessage, message_broadcast, NetworkPlugin as BaseNetworkPlugin, NetworkSendEvent,
};

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
        app.add_plugin(self.inner.clone())
            .add_event::<NetworkSendEvent<ClientMessage>>()
            .add_system(message_broadcast::<ClientMessage>.system());
    }
}
