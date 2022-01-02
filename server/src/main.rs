mod character;
mod network;
mod spawning;
mod spells;
mod state;

use std::time::Duration;

use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    log::LogPlugin,
    prelude::*,
};

use common::{
    character::{CharacterPlugin, Class, Team},
    game::{ArenaPasscode, ArenaPermit, GameRoster},
    heron::PhysicsPlugin,
};
use network::NetworkServerPlugin;
use state::{ServerState, StateTransitionPlugin};

use crate::spawning::SpawnPlugin;

fn main() {
    // TODO: Read these from CMD line
    let permits = [
        ArenaPermit {
            passcode: ArenaPasscode(1234),
            class: Class::Mars,
            team: Team::Red,
        },
        ArenaPermit {
            passcode: ArenaPasscode(4321),
            class: Class::Medea,
            team: Team::Red,
        },
    ]
    .into_iter()
    .collect();

    ////
    // App construction
    App::build()
        .insert_resource(GameRoster::new(permits))
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(CharacterPlugin { state: ServerState })
        .add_plugin(NetworkServerPlugin)
        .add_plugin(StateTransitionPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(SpawnPlugin)
        .run();
}
