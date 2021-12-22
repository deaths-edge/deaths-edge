mod character;
mod network;
mod spawning;
mod state;

use std::time::Duration;

use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    log::LogPlugin,
    prelude::*,
};

use common::{
    character::{CharacterClass, CharacterEntityCommandPlugin, CharacterTeam},
    game::{ArenaPasscode, ArenaPermit, GameRoster},
    heron::PhysicsPlugin,
};
use network::NetworkServerPlugin;
use state::{ServerState, StateTransitionEvent, StateTransitionPlugin};

use crate::{spawning::SpawnPlugin, state::STATE_TRANSITION_LABEL};

fn main() {
    let initial_set = SystemSet::new()
        .before(STATE_TRANSITION_LABEL)
        .with_system(initial.system());

    ////
    // App construction
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(CharacterEntityCommandPlugin::new(ServerState::Running))
        .add_state(ServerState::Idle)
        .add_plugin(NetworkServerPlugin)
        .add_plugin(StateTransitionPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(SpawnPlugin)
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
        let permits = [
            ArenaPermit::new(
                ArenaPasscode(1234),
                CharacterClass::Medea,
                CharacterTeam::Red,
            ),
            ArenaPermit::new(
                ArenaPasscode(4321),
                CharacterClass::Medea,
                CharacterTeam::Red,
            ),
        ]
        .into_iter()
        .collect();
        transition_writer.send(StateTransitionEvent::Setup {
            roster: GameRoster::new(permits),
        });
    }
}
