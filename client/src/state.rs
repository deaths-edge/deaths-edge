use std::net::SocketAddr;

use bevy::prelude::*;

use crate::{
    character::CharacterPlugin,
    game_camera::GameCameraPlugin,
    input_mapping::InputMapPlugin,
    music::SplashMusicPlugin,
    network::{GameServer, NetworkingState},
    spawning::SpawnPlugin,
    spells::SpellPlugin,
    ui::{splash::SplashUIPlugin, UIPlugin},
};

use common::{
    effects::EffectPlugin, heron::PhysicsPlugin, network::server::ArenaSetup, state::ArenaState,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    /// Splash screen
    Splash,
    /// Main lobby
    MainLobby,
    /// Connecting
    Connecting,
    /// In arena
    Arena,
}

pub const STATE_TRANSITIONS_LABEL: &str = "state-transitions";

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state_transitions = SystemSet::new()
            .label(STATE_TRANSITIONS_LABEL)
            // TODO: Ordering
            .with_system(state_transitions.system());
        app.add_event::<StateTransition>()
            .add_system_set(state_transitions);
    }
}

#[derive(Debug)]
pub enum StateTransition {
    MainLobby,
    Connect { server: SocketAddr },
    Connected { setup: ArenaSetup },
}

/// Listen for [`StateTransition`]s and perform abilitys.
fn state_transitions(
    mut transition_events: EventReader<StateTransition>,

    mut app_state: ResMut<State<ClientState>>,
    mut network_state: ResMut<State<NetworkingState>>,
    mut arena_state: ResMut<State<ArenaState>>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if let Some(event) = transition_events.iter().next() {
        info!(state_transition = ?event);
        match event {
            StateTransition::MainLobby => {
                app_state
                    .set(ClientState::MainLobby)
                    .expect("state transition failed");
            }
            StateTransition::Connect { server } => {
                app_state
                    .set(ClientState::Connecting)
                    .expect("state transition failed");
                network_state
                    .set(NetworkingState::Active)
                    .expect("state transition failed");
                commands.insert_resource(GameServer::new(*server))
            }
            StateTransition::Connected { setup } => {
                app_state
                    .set(ClientState::Arena)
                    .expect("state transition failed");
                arena_state
                    .set(ArenaState::Waiting)
                    .expect("state transition failed");
                setup.map.spawn_environment(&mut commands, &mut materials);
            }
        }
    }
}

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(SplashUIPlugin).add_plugin(SplashMusicPlugin);
    }
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(ArenaState::Inactive)
            .add_plugin(CharacterPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(SpawnPlugin)
            .add_plugin(InputMapPlugin)
            .add_plugin(SpellPlugin)
            .add_plugin(EffectPlugin::new(ClientState::Arena))
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(GameCameraPlugin)
            // .add_plugin(EnvironmentPlugin::new(ClientState::Arena))
            ;
    }
}
