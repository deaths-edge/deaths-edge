use std::net::SocketAddr;

use bevy::prelude::*;

use crate::{
    character::CharacterPlugin,
    game_camera::GameCameraPlugin,
    input_mapping::InputMapPlugin,
    music::SplashMusicPlugin,
    network::GameServer,
    spawning::SpawnPlugin,
    ui::{splash::SplashUIPlugin, UIPlugins},
};

use common::{
    effects::EffectPlugin, environment::EnvironmentPlugin, heron::PhysicsPlugin,
    spells::SpellPlugin, state::ArenaState,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    /// Splash screen
    Splash,
    /// Main lobby
    MainLobby,
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
        app.add_event::<StateTransitionEvent>()
            .add_system_set(state_transitions);
    }
}

#[derive(Debug)]
pub enum StateTransitionEvent {
    ToArena { server: SocketAddr },
}

/// Listen for [`StateTransitionEvent`]s and perform actions.
fn state_transitions(
    mut commands: Commands,
    mut transition_events: EventReader<StateTransitionEvent>,
    mut app_state: ResMut<State<ClientState>>,
    arena_state: ResMut<State<ArenaState>>,
) {
    if let Some(event) = transition_events.iter().next() {
        info!(state_transition = ?event);
        match event {
            StateTransitionEvent::ToArena { server } => {
                app_state
                    .set(ClientState::Arena)
                    .expect("state transition failed");
                commands.insert_resource(GameServer::new(*server))
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
        app.add_state(ArenaState::Waiting)
            .add_plugin(CharacterPlugin)
            .add_plugins(UIPlugins)
            .add_plugin(SpawnPlugin)
            .add_plugin(InputMapPlugin)
            .add_plugin(SpellPlugin::new(ClientState::Arena))
            .add_plugin(EffectPlugin::new(ClientState::Arena))
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(GameCameraPlugin)
            .add_plugin(EnvironmentPlugin::new(ClientState::Arena));
    }
}
