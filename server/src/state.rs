use std::{collections::HashMap, net::SocketAddr};

use bevy::prelude::*;

use common::{
    character::CastingPlugin, effects::EffectPlugin, environment::EnvironmentPlugin,
    heron::PhysicsPlugin, spells::SpellPlugin,
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
    Setup { team_size: usize, passcode: u64 },
}

pub enum Team {
    Red,
    Blue,
}

pub struct Client {
    id: Entity,
    team: Team,
}

impl Client {
    pub fn new(id: Entity, team: Team) -> Self {
        Self { id, team }
    }
}

pub struct GameState {
    team_size: usize,
    passcode: u64,
    characters: HashMap<SocketAddr, Client>,
}

impl GameState {
    pub fn passcode(&self) -> u64 {
        self.passcode
    }

    pub fn id(&self, socket: &SocketAddr) -> Option<Entity> {
        self.characters.get(socket).map(|client| client.id)
    }

    pub fn insert_client(&mut self, address: SocketAddr, client: Client) {
        self.characters.insert(address, client);
    }
}

fn state_transitions(
    mut commands: Commands,
    mut transition_events: EventReader<StateTransitionEvent>,
    mut app_state: ResMut<State<ServerState>>,
) {
    if let Some(event) = transition_events.iter().next() {
        info!(state_transition = ?event);
        match event {
            StateTransitionEvent::Setup {
                team_size,
                passcode,
            } => {
                if *app_state.current() == ServerState::Idle {
                    info!("inserting game state");
                    commands.insert_resource(GameState {
                        team_size: *team_size,
                        passcode: *passcode,
                        characters: HashMap::with_capacity(*team_size),
                    });
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
