use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::network::server::NetworkServerPlugin;

fn main() {
    const NETWORK_POLL_INTERVAL: Duration = Duration::from_millis(500);
    let socket: SocketAddr = "127.0.0.1:8000".parse().expect("invalid socket");

    let server_plugin = NetworkServerPlugin::new(socket, NETWORK_POLL_INTERVAL);
    App::build().add_plugin(server_plugin).run();
}
