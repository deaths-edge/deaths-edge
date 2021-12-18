mod character;
mod network;
mod spawning;
mod state;

use std::{net::SocketAddr, time::Duration};

use bevy::{core::FixedTimestep, prelude::*};

use character::ClientAddress;
use common::{
    character::{
        CharacterClass, CharacterEntityCommandPlugin, CharacterIndex, CharacterMarker,
        CharacterTeam,
    },
    game::{ArenaPasscode, ArenaPermit, GameRoster},
    heron::PhysicsPlugin,
};
use network::NetworkServerPlugin;
use state::{ServerState, StateTransitionEvent, StateTransitionPlugin};

use crate::{spawning::SpawnPlugin, state::STATE_TRANSITION_LABEL};

fn main() {
    let socket: SocketAddr = "127.0.0.1:8000".parse().expect("invalid socket");

    let initial_set = SystemSet::new()
        .before(STATE_TRANSITION_LABEL)
        .with_system(initial.system());

    let print_positions = SystemSet::new()
        .with_run_criteria(FixedTimestep::step(1.0))
        .with_system(print_positions.system());

    ////
    // App construction
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(CharacterEntityCommandPlugin::new(ServerState::Running))
        // .add_plugin(LogPlugin)
        // .add_plugin(CorePlugin)
        // .add_plugin(TransformPlugin)
        .add_state(ServerState::Idle)
        .add_plugin(NetworkServerPlugin::new(socket))
        .add_plugin(StateTransitionPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(SpawnPlugin)
        .add_system_set(print_positions)
        .add_system_set(initial_set)
        .run();
}

fn print_positions(
    query: Query<(&CharacterIndex, &ClientAddress, &Transform), With<CharacterMarker>>,
) {
    for (index, address, transform) in query.iter() {
        info!(?index, position = ?transform.translation, address = %address.0);
    }
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
