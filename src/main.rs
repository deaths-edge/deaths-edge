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
    const ENV_FILTER: &str = "deaths_edge=trace";
    let debug_plugin = debug::DebugTerminalPlugin::new(
        ENV_FILTER,
        FPS_COLLECTION_INTERVAL,
        RENDER_UPDATE_INTERVAL,
    );

    ////
    // App construction
    App::new()
        // Window description
        .insert_resource(window_description)
        // Default plugins
        .add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        // Debug plugins
        .add_plugin(debug_plugin)
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
