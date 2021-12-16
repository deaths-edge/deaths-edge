mod character;
mod network;
mod spawning;
mod state;

use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use common::{character::CharacterCommandPlugin, heron::PhysicsPlugin};
use network::NetworkServerPlugin;
use state::{ServerState, StateTransitionEvent, StateTransitionPlugin};

fn main() {
    const NETWORK_POLL_INTERVAL: Duration = Duration::from_millis(500);
    let socket: SocketAddr = "127.0.0.1:8000".parse().expect("invalid socket");

    let initial_set = SystemSet::new()
        .before("state-transitions")
        .with_system(initial.system());

    ////
    // App construction
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterCommandPlugin::new(ServerState::Running))
        // .add_plugin(LogPlugin)
        // .add_plugin(CorePlugin)
        // .add_plugin(TransformPlugin)
        .add_state(ServerState::Idle)
        .add_plugin(NetworkServerPlugin::new(socket, NETWORK_POLL_INTERVAL))
        .add_plugin(StateTransitionPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_system_set(initial_set)
        .run();
}

fn initial(
    time: Res<Time>,
    app_state: Res<State<ServerState>>,
    mut transition_writer: EventWriter<StateTransitionEvent>,
) {
    let duration = time.time_since_startup();

    if duration < Duration::from_secs(3) {
        return;
    }

    if *app_state.current() == ServerState::Idle {
        transition_writer.send(StateTransitionEvent::Setup {
            passcode: 1234,
            team_size: 2,
        });
    }
}
