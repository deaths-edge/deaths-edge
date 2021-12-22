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

use std::{
    fs::File,
    io::{self, Write},
    net::SocketAddr,
    time::Duration,
};

use bevy::prelude::*;
use bevy_mod_debugdump::schedule_graph::schedule_graph_dot;

use structopt::StructOpt;

use character::PlayerMarker;
use common::{
    character::{CharacterIndex, CharacterMarker},
    network::{find_my_ip_address, SERVER_PORT},
};
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

fn print_positions(
    mut query: Query<(&CharacterIndex, &Transform, With<PlayerMarker>), With<CharacterMarker>>,
) {
    for (index, transform, player) in query.iter_mut() {
        // info!(?index, position = ?transform.translation, %player);
    }
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
        .add_plugin(NetworkPlugin)
        .add_system(print_positions.system());

    // save_schedule_graph(&mut app).expect("failed to save schedule graph");

    app.run();
}

pub fn save_schedule_graph(app: &mut AppBuilder) -> Result<(), io::Error> {
    const PATH: &str = "./schedule.dot";

    let mut schedule_graph = File::create(PATH)?;
    schedule_graph.write(schedule_graph_dot(&app.app.schedule).as_bytes())?;

    Ok(())
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

    let ip_address = find_my_ip_address().expect("can't find ip address");
    let server = SocketAddr::new(ip_address, SERVER_PORT);

    if *app_state.current() != ClientState::Arena {
        // transition_writer.send(StateTransitionEvent::ToArena {
        //     server: settings.server,
        // });
        transition_writer.send(StateTransitionEvent::ToArena { server });
    }
}
