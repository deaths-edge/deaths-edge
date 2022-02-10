mod abilities;
mod character;
mod game_camera;
mod input_mapping;
// mod music;
mod network;
mod spawning;
mod state;
mod ui;
mod window;

use std::{net::SocketAddr, time::Duration};

use bevy::prelude::*;

use clap::Parser;

use network::NetworkPlugin;
use state::*;
use window::window_description;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Opt {
    #[clap(short, long, default_value = "127.0.0.1:8000")]
    server: SocketAddr,
    #[clap(short, long, default_value = "1234")]
    passcode: u64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    /// Splash screen
    Splash,
    /// Main lobby
    MainLobby,
    /// Connecting
    Connecting,
    /// In arena
    Arena,
}

fn state_transition(
    time: Res<Time>,
    app_state: Res<State<GameState>>,
    mut transition_writer: EventWriter<StateTransition>,
    settings: Res<Opt>,
) {
    let duration = time.time_since_startup();

    if duration < Duration::from_secs(3) {
        return;
    }

    if *app_state.current() == GameState::Splash {
        transition_writer.send(StateTransition::MainLobby);
    }
}

fn main() {
    let window_description = window_description();

    let opt = Opt::parse();

    let state_transitions = SystemSet::new()
        .before("state-transitions")
        .with_system(state_transition);

    ////
    // App construction
    let mut app = App::new();

    app
        // Settings
        .insert_resource(opt)
        // Window description
        .insert_resource(window_description)
        // Default plugins
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Splash)
        .add_plugin(StateTransitionPlugin)
        .add_plugin(SplashPlugin)
        .add_plugin(ArenaPlugin)
        .add_system_set(state_transitions)
        // Network plugin
        .add_plugin(NetworkPlugin);

    app.run();
}
