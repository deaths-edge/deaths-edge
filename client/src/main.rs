mod character;
mod game_camera;
mod input_mapping;
mod music;
mod network;
mod spawning;
mod state;
mod ui;
mod window;

use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use structopt::StructOpt;

use network::NetworkPlugin;
use state::*;
use window::window_description;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1:8000")]
    server: SocketAddr,
    #[structopt(short, long, default_value = "1234")]
    passcode: u64,
}

fn state_transition(
    time: Res<Time>,
    app_state: Res<State<ClientState>>,
    mut transition_writer: EventWriter<StateTransition>,
    settings: Res<Opt>,
) {
    let duration = time.time_since_startup();

    if duration < Duration::from_secs(3) {
        return;
    }

    if *app_state.current() == ClientState::Splash {
        transition_writer.send(StateTransition::MainLobby);
    }
}

fn main() {
    let window_description = window_description();

    let opt = Opt::from_args();

    let state_transitions = SystemSet::new()
        .before("state-transitions")
        .with_system(state_transition.system());

    ////
    // App construction
    let mut app = App::build();

    app
        // Settings
        .insert_resource(opt)
        // Window description
        .insert_resource(window_description)
        // Default plugins
        .add_plugins(DefaultPlugins)
        // .add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        // Debug plugins
        // .add_plugin(debug_plugin)
        .add_state(ClientState::Splash)
        .add_plugin(StateTransitionPlugin)
        .add_plugin(SplashPlugin)
        .add_plugin(ArenaPlugin)
        .add_system_set(state_transitions)
        // Network plugin
        .add_plugin(NetworkPlugin);

    // save_schedule_graph(&mut app).expect("failed to save schedule graph");

    app.run();
}
