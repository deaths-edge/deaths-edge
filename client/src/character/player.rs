use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use super::*;

use common::{
    character::{CharacterEntityCommand, CharacterIndex, CharacterMarker, Target},
    network::server::SpawnCharacter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Waiting,
    Spawned,
    Dead,
}

/// Switch from waiting to spawned
pub fn spawn_state(
    mut spawn_reader: EventReader<SpawnCharacter>,
    mut player_state: ResMut<State<PlayerState>>,
) {
    for spawn in spawn_reader.iter() {
        if spawn.player() {
            player_state
                .set(PlayerState::Spawned)
                .expect("this can't happen twice")
        }
    }
}

pub const CHARACTER_ACTIONS: &str = "character-actions";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let player_state_transition =
            SystemSet::on_update(PlayerState::Waiting).with_system(spawn_state.system());
        let character_actions = SystemSet::on_update(PlayerState::Spawned)
            .label(CHARACTER_ACTIONS)
            // TODO: Ordering
            .with_system(player_select.system());
        app.add_state(PlayerState::Waiting)
            .add_system_set(player_state_transition)
            .add_system_set(character_actions);
    }
}

pub struct PlayerMarker;

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    #[bundle]
    character_bundle: CharacterBundle,
}

impl PlayerBundle {
    pub fn new(character_bundle: CharacterBundle) -> Self {
        Self {
            player_marker: PlayerMarker,
            character_bundle,
        }
    }
}

pub fn player_select(
    mut target_reader: EventReader<CharacterEntityCommand<Target>>,
    player_query: Query<(), With<PlayerMarker>>,
    mut character_query: QuerySet<(
        Query<(Entity, &mut Selected)>,
        Query<(&CharacterIndex, &mut Selected), With<CharacterMarker>>,
    )>,
) {
    for target_command in target_reader.iter() {
        let is_player = player_query.get(target_command.id()).is_ok();

        if is_player {
            // Deselect everything
            for (_, mut selected) in character_query.q0_mut().iter_mut() {
                *selected = Selected::Unselected;
            }

            if let Some(target_index) = target_command.command().0 {
                let (_, mut selected) = character_query
                    .q1_mut()
                    .iter_mut()
                    .find(|(index, _)| **index == target_index)
                    .expect("failed to find selection");
                *selected = Selected::Selected;
            }
        }
    }
}
