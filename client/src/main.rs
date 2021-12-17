mod character;
mod debug;
mod game_camera;
mod input_mapping;
mod music;
mod network;
mod spawning;
mod state;
mod ui;
mod window;

use std::{net::SocketAddr, time::Duration};

use bevy::{log::LogPlugin, prelude::*};
use structopt::StructOpt;

use network::NetworkPlugin;
use state::*;
use window::window_description;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1:8000")]
    server: SocketAddr,
    #[structopt(short, long, default_value = "127.0.0.1:8001")]
    bind: SocketAddr,
    #[structopt(short, long, default_value = "1234")]
    passcode: u64,
}

fn main() {
    let window_description = window_description();

    let opt = Opt::from_args();

    ////
    // Debug plugin
    const FPS_COLLECTION_INTERVAL: Duration = Duration::from_secs(1);
    const RENDER_UPDATE_INTERVAL: Duration = Duration::from_millis(1_000);
    const ENV_FILTER: &str = concat!(env!("CARGO_PKG_NAME"), "=trace,common=trace");
    let debug_plugin = debug::DebugTerminalPlugin::new(
        ENV_FILTER,
        FPS_COLLECTION_INTERVAL,
        RENDER_UPDATE_INTERVAL,
    );

    const NETWORK_POLL_INTERVAL: Duration = Duration::from_millis(500);
    let network_plugin = NetworkPlugin::new(opt.bind, NETWORK_POLL_INTERVAL);

    let state_transitions = SystemSet::new()
        .before("state-transitions")
        .with_system(state_transition.system());

    ////
    // App construction
    App::build()
        // Settings
        .insert_resource(opt)
        // Window description
        .insert_resource(window_description)
        // Default plugins
        .add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        // Debug plugins
        .add_plugin(debug_plugin)
        .add_state(ClientState::Splash)
        .add_plugin(StateTransitionPlugin)
        .add_plugin(SplashPlugin)
        .add_plugin(ArenaPlugin)
        .add_system_set(state_transitions)
        // Network plugin
        .add_plugin(network_plugin)
        .run();
}

fn state_transition(
    time: Res<Time>,
    app_state: Res<State<ClientState>>,
    mut transition_writer: EventWriter<StateTransitionEvent>,
    settings: Res<Opt>,
) {
    let duration = time.time_since_startup();

    if duration < Duration::from_secs(3) {
        return;
    }

    if *app_state.current() != ClientState::Arena {
        transition_writer.send(StateTransitionEvent::ToArena {
            server: settings.server,
        });
    }
}
