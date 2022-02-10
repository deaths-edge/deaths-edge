use std::net::SocketAddr;

use bevy::prelude::*;

use crate::{
    abilities::{AbilityPlugin, AbilityState},
    character::{CharacterPlugin, CharacterState},
    game_camera::{CameraState, GameCameraPlugin},
    input_mapping::InputMapPlugin,
    // music::SplashMusicPlugin,
    network::{ArenaServer, NetworkingState},
    spawning::{SpawnPlugin, SpawnState},
    ui::{hud::HudState, mouse::WorldMouseState, splash::SplashUIPlugin, UIPlugin},
    GameState,
};

use common::{heron::PhysicsPlugin, network::server::ArenaSetup, state::ArenaState};

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

    mut game_state: ResMut<State<GameState>>,
    mut spawning_state: ResMut<State<SpawnState>>,
    mut character_state: ResMut<State<CharacterState>>,
    mut ability_state: ResMut<State<AbilityState>>,
    mut network_state: ResMut<State<NetworkingState>>,
    mut arena_state: ResMut<State<ArenaState>>,
    mut camera_state: ResMut<State<CameraState>>,
    mut mouse_state: ResMut<State<WorldMouseState>>,
    mut hud_state: ResMut<State<HudState>>,

    mut commands: Commands,
) {
    const STATE_TRANSITION_FAILED: &str = "state transition failed";
    if let Some(event) = transition_events.iter().next() {
        info!(state_transition = ?event);
        match event {
            StateTransition::MainLobby => {
                game_state
                    .set(GameState::MainLobby)
                    .expect(STATE_TRANSITION_FAILED);
                spawning_state
                    .set(SpawnState::Active)
                    .expect(STATE_TRANSITION_FAILED);
                character_state
                    .set(CharacterState::Active)
                    .expect(STATE_TRANSITION_FAILED);
                ability_state
                    .set(AbilityState::Active)
                    .expect(STATE_TRANSITION_FAILED);
                camera_state
                    .set(CameraState::Active)
                    .expect(STATE_TRANSITION_FAILED);
                mouse_state
                    .set(WorldMouseState::Active)
                    .expect(STATE_TRANSITION_FAILED);
                hud_state
                    .set(HudState::Active)
                    .expect(STATE_TRANSITION_FAILED);
            }
            StateTransition::Connect { server } => {
                game_state
                    .set(GameState::Connecting)
                    .expect("state transition failed");
                spawning_state
                    .set(SpawnState::Inactive)
                    .expect("state transition failed");
                network_state
                    .set(NetworkingState::Active)
                    .expect("state transition failed");
                character_state
                    .set(CharacterState::Inactive)
                    .expect("state transition failed");
                commands.insert_resource(ArenaServer::new(*server))
            }
            StateTransition::Connected { setup } => {
                game_state
                    .set(GameState::Arena)
                    .expect("state transition failed");
                spawning_state
                    .set(SpawnState::Active)
                    .expect("state transition failed");
                arena_state
                    .set(ArenaState::Waiting)
                    .expect("state transition failed");
                character_state
                    .set(CharacterState::Active)
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
