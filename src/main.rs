mod buffs;
mod character;
mod debug;
mod environment;
mod game_event;
mod input_mapping;
mod physics;
mod spell;
mod state;
mod ui;

use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*};

use character::*;
use environment::spawn_environment;
use ui::setup_camera;

fn main() {
    let window_description = WindowDescriptor {
        title: "Death's Edge".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    };

    const FPS_COLLECTION_INTERVAL: Duration = Duration::from_secs(1);
    const RENDER_UPDATE_INTERVAL: Duration = Duration::from_millis(1_000);
    const ENV_FILTER: &str = "deaths_edge=trace";
    let debug_plugin = debug::DebugTerminalPlugin::new(
        ENV_FILTER,
        FPS_COLLECTION_INTERVAL,
        RENDER_UPDATE_INTERVAL,
    );

    App::build()
        .insert_resource(window_description)
        .add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        .add_plugin(debug_plugin)
        .add_plugins(CharacterPlugins)
        .add_plugin(input_mapping::InputMapPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(ui::UIPlugin)
        .add_startup_system(setup_camera.system())
        .add_startup_system(spawn_environment.system())
        .run();
}
