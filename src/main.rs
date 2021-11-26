mod buffs;
mod camera;
mod character;
mod debug;
mod environment;
mod input_mapping;
mod physics;
mod state;
mod ui;

use bevy::{log::LogPlugin, prelude::*};

use camera::setup_camera;
use character::*;
use environment::spawn_environment;
use ui::{world_mouse, WorldMousePosition};

fn main() {
    let window_description = WindowDescriptor {
        title: "Death's Edge".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    };

    let debug_plugin = debug::DebugTerminalPlugin::new("deaths_edge=trace");

    App::build()
        .insert_resource(window_description)
        .init_resource::<WorldMousePosition>()
        .add_plugins_with(DefaultPlugins, |plugins| plugins.disable::<LogPlugin>())
        .add_plugin(debug_plugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(input_mapping::InputMapPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_startup_system(setup_camera.system())
        .add_startup_system(spawn_environment.system())
        .add_system(world_mouse.system())
        .run();
}
