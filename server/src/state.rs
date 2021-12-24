use bevy::prelude::*;

use common::{
    character::CastingPlugin, effects::EffectPlugin, heron::PhysicsPlugin, spells::SpellPlugin,
    state::ArenaState,
};
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ServerState;

use crate::network::NETWORK_HANDLE_LABEL;

pub const STATE_TRANSITION_LABEL: &str = "state-transition";

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let state_transitions = SystemSet::new()
            .label(STATE_TRANSITION_LABEL)
            // TODO: Ordering
            .after(NETWORK_HANDLE_LABEL)
            .with_system(state_transitions.system());
        app.add_event::<StateTransition>()
            .add_state(ServerState)
            .add_event::<ArenaState>()
            .add_system_set(state_transitions);
    }
}

#[derive(Debug)]
pub enum StateTransition {}

fn state_transitions(
    mut transition_events: EventReader<StateTransition>,

    mut spawning_state: ResMut<State<ArenaState>>,

    mut commands: Commands,
) {
    if let Some(event) = transition_events.iter().next() {}
}

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CastingPlugin::new(ServerState))
            .add_plugin(SpellPlugin::new(ServerState))
            .add_plugin(EffectPlugin::new(ServerState))
            .add_plugin(PhysicsPlugin::default());
    }
}
