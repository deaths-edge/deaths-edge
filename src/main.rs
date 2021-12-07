mod buffs;
mod character;
mod debug;
mod effects;
mod environment;
mod game_camera;
mod game_event;
mod input_mapping;
mod networking;
mod physics;
mod spawning;
mod spells;
mod state;
mod ui;

use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*};
use bevy_ggrs::{GGRSApp, GGRSPlugin};
use clap::Parser;
use ggrs::P2PSession;
use heron::Velocity;

use networking::{input, start_p2p_session};
use state::*;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Venturi Games <venturi.games@protonmail.com>"
)]
pub struct Opts {
    #[clap(short, long)]
    local_port: u16,
    #[clap(short, long)]
    players: Vec<String>,
}

fn main() {
    let window_description = WindowDescriptor {
        title: "Death's Edge".to_string(),
        width: 800.,
        height: 600.,
        vsync: false,
        ..Default::default()
    };

    ////
    // Debug plugin
    const FPS_COLLECTION_INTERVAL: Duration = Duration::from_secs(1);
    const RENDER_UPDATE_INTERVAL: Duration = Duration::from_millis(1_000);
    const ENV_FILTER: &str = "deaths_edge=trace";
    let debug_plugin = debug::DebugTerminalPlugin::new(
        ENV_FILTER,
        FPS_COLLECTION_INTERVAL,
        RENDER_UPDATE_INTERVAL,
    );

    ////
    // Networking
    // read cmd line arguments
    let opt = Opts::parse();
    let num_players = opt.players.len();
    assert!(num_players > 0);

    const INPUT_SIZE: usize = std::mem::size_of::<u8>();
    const FPS: u32 = 60;
    let mut p2p_sess =
        P2PSession::new(2, INPUT_SIZE, opt.local_port).expect("failed to construct P2PSession");
    p2p_sess.set_fps(FPS).expect("invalid fps");
    p2p_sess
        .set_sparse_saving(true)
        .expect("failed to set sparse saving");

    ////
    // App construction
    App::new()
        // Window description
        .insert_resource(window_description)
        // Options
        .insert_resource(opt)
        // Default plugins
        .add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        // Debug plugins
        .add_plugin(debug_plugin)
        // Networking
        .add_plugin(GGRSPlugin)
        .add_startup_system(start_p2p_session)
        .with_update_frequency(FPS)
        .with_input_system(input)
        .with_p2p_session(p2p_sess)
        .register_rollback_type::<Transform>()
        .register_rollback_type::<Velocity>()
        // State
        .add_state(AppState::Splash)
        .add_plugin(SplashPlugin)
        .add_plugin(ArenaPlugin)
        .add_system(state_transition.system())
        .run();
}

fn state_transition(time: Res<Time>, mut app_state: ResMut<State<AppState>>) {
    let duration = time.time_since_startup();

    if duration < Duration::from_secs(3) {
        return;
    }

    if *app_state.current() != AppState::Arena {
        app_state
            .set(AppState::Arena)
            .expect("state transition failed");
    }
}
