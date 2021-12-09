mod character;
mod debug;
mod game_camera;
mod input_mapping;
mod music;
mod network;
mod spawning;
mod state;
mod ui;

use std::{net::SocketAddr, time::Duration};

use bevy::{log::LogPlugin, prelude::*};

use network::NetworkPlugin;
use state::*;

fn main() {
    let window_description = WindowDescriptor {
        title: "Death's Edge".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    };

    ////
    // Debug plugin
    const FPS_COLLECTION_INTERVAL: Duration = Duration::from_secs(1);
    const RENDER_UPDATE_INTERVAL: Duration = Duration::from_millis(1_000);
    const ENV_FILTER: &str = concat!(env!("CARGO_PKG_NAME"), "=trace");
    let debug_plugin = debug::DebugTerminalPlugin::new(
        ENV_FILTER,
        FPS_COLLECTION_INTERVAL,
        RENDER_UPDATE_INTERVAL,
    );

    const NETWORK_POLL_INTERVAL: Duration = Duration::from_millis(500);
    let socket: SocketAddr = "127.0.0.1:8001".parse().expect("invalid socket");
    let network_plugin = NetworkPlugin::new(socket, NETWORK_POLL_INTERVAL);

    ////
    // App construction
    App::build()
        // Window description
        .insert_resource(window_description)
        // Default plugins
        .add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        // Debug plugins
        .add_plugin(debug_plugin)
        .add_state(ClientState::Splash)
        .add_plugin(SplashPlugin)
        .add_plugin(ArenaPlugin)
        .add_system(state_transition.system())
        // Network plugin
        .add_plugin(network_plugin)
        .run();
}

fn state_transition(time: Res<Time>, mut app_state: ResMut<State<ClientState>>) {
    let duration = time.time_since_startup();

    if duration < Duration::from_secs(3) {
        return;
    }

    if *app_state.current() != ClientState::Arena {
        app_state
            .set(ClientState::Arena)
            .expect("state transition failed");
    }
}
