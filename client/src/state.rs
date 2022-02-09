use std::net::SocketAddr;

use bevy::prelude::*;

use crate::{
    abilities::AbilityPlugin,
    character::CharacterPlugin,
    game_camera::GameCameraPlugin,
    input_mapping::InputMapPlugin,
    // music::SplashMusicPlugin,
    network::{ArenaServer, NetworkingState},
    spawning::{SpawnPlugin, SpawnState},
    ui::{splash::SplashUIPlugin, UIPlugin},
};

use common::{heron::PhysicsPlugin, network::server::ArenaSetup, state::ArenaState};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
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
    fn build(&self, app: &mut App) {
        let state_transitions = SystemSet::new()
            .label(STATE_TRANSITIONS_LABEL)
            // TODO: Ordering
            .with_system(state_transitions);
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

/// Listen for [`StateTransition`]s and perform abilities.
fn state_transitions(
    mut transition_events: EventReader<StateTransition>,

    mut ui_state: ResMut<State<GameState>>,
    mut spawning_state: ResMut<State<SpawnState>>,
    mut network_state: ResMut<State<NetworkingState>>,
    mut arena_state: ResMut<State<ArenaState>>,

    mut commands: Commands,
) {
    if let Some(event) = transition_events.iter().next() {
        info!(state_transition = ?event);
        match event {
            StateTransition::MainLobby => {
                ui_state
                    .set(GameState::MainLobby)
                    .expect("state transition failed");
                spawning_state
                    .set(SpawnState::Active)
                    .expect("state transition failed");
            }
            StateTransition::Connect { server } => {
                ui_state
                    .set(GameState::Connecting)
                    .expect("state transition failed");
                spawning_state
                    .set(SpawnState::Unactive)
                    .expect("state transition failed");
                network_state
                    .set(NetworkingState::Active)
                    .expect("state transition failed");
                commands.insert_resource(ArenaServer::new(*server))
            }
            StateTransition::Connected { setup } => {
                ui_state
                    .set(GameState::Arena)
                    .expect("state transition failed");
                spawning_state
                    .set(SpawnState::Active)
                    .expect("state transition failed");
                arena_state
                    .set(ArenaState::Waiting)
                    .expect("state transition failed");
                setup.map.spawn_environment(&mut commands);
            }
        }
    }
}

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SplashUIPlugin)
        // .add_plugin(SplashMusicPlugin)
        ;
    }
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(ArenaState::Inactive)
            .add_plugin(CharacterPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(SpawnPlugin)
            .add_plugin(InputMapPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(GameCameraPlugin);
    }
}
