use std::net::SocketAddr;

use bevy::prelude::*;

use common::{
    character::CastingPlugin, effects::EffectPlugin, environment::EnvironmentPlugin,
    game::GameRoster, heron::PhysicsPlugin, spells::SpellPlugin,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ServerState {
    Idle,
    Running,
}

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state_transitions = SystemSet::new()
            .label("state-transitions")
            .with_system(state_transitions.system());
        app.add_event::<StateTransitionEvent>()
            .add_system_set(state_transitions);
    }
}

#[derive(Debug)]
pub enum StateTransitionEvent {
    Setup { roster: GameRoster },
}

pub struct Client {
    id: Entity,
    address: SocketAddr,
}

impl Client {
    pub fn new(id: Entity, address: SocketAddr) -> Self {
        Self { id, address }
    }
}

fn state_transitions(
    mut commands: Commands,
    mut transition_events: EventReader<StateTransitionEvent>,
    mut app_state: ResMut<State<ServerState>>,
) {
    if let Some(event) = transition_events.iter().next() {
        match event {
            StateTransitionEvent::Setup { roster } => {
                if *app_state.current() == ServerState::Idle {
                    info!(message = "inserting roster", ?roster);
                    commands.insert_resource(roster.clone());
                    app_state.set(ServerState::Running).expect("double set");
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
