use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{message_broadcast, messages::*, NetworkPlugin};

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    Position(PositionMessage),
    Velocity(VelocityMessage),
}

pub struct NetworkClientPlugin {
    inner: NetworkPlugin,
}

impl NetworkClientPlugin {
    pub fn new(address: SocketAddr, poll_interval: Duration) -> Self {
        Self {
            inner: NetworkPlugin::new(address, poll_interval),
        }
    }
}

impl Plugin for NetworkClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(self.inner.clone())
            .add_system(message_broadcast::<ClientMessage>.system());
    }
}
