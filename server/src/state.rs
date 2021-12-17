use std::net::SocketAddr;

use bevy::prelude::*;

use common::{
    character::CastingPlugin,
    effects::EffectPlugin,
    environment::EnvironmentPlugin,
    game::GameRoster,
    heron::PhysicsPlugin,
    spells::SpellPlugin,
    state::{ArenaState, SpawningState},
};

use crate::network::{NETWORK_HANDLE_LABEL, NETWORK_SEND_LABEL};

pub const STATE_TRANSITION_LABEL: &str = "state-transition";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ServerState {
    Idle,
    Running,
}

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state_transitions = SystemSet::new()
            .label(STATE_TRANSITION_LABEL)
            .after(NETWORK_HANDLE_LABEL)
            .before(NETWORK_SEND_LABEL)
            .with_system(state_transitions.system());
        app.add_event::<StateTransitionEvent>()
            .add_event::<ArenaState>()
            .add_system_set(state_transitions);
    }
}

#[derive(Debug)]
pub enum StateTransitionEvent {
    Setup { roster: GameRoster },
}

fn state_transitions(
    mut commands: Commands,
    mut transition_events: EventReader<StateTransitionEvent>,
    mut app_state: ResMut<State<ServerState>>,
    mut spawning_state: ResMut<State<SpawningState>>,
) {
    if let Some(event) = transition_events.iter().next() {
        match event {
            StateTransitionEvent::Setup { roster } => {
                if *app_state.current() == ServerState::Idle {
                    info!(message = "inserting roster", ?roster);
                    commands.insert_resource(roster.clone());
                    app_state.set(ServerState::Running).expect("double set");
                    spawning_state
                        .set(SpawningState::Active)
                        .expect("couldn't set spawning state");
                }
            }
        }
    }
}

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CastingPlugin::new(ServerState::Running))
            .add_plugin(SpellPlugin::new(ServerState::Running))
            .add_plugin(EffectPlugin::new(ServerState::Running))
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(EnvironmentPlugin::new(ServerState::Running));
    }
}
