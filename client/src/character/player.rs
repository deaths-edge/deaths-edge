use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use super::*;

use common::character::{CharacterEntityAction, CharacterIndex, CharacterMarker, SelectTarget};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerState {
    Waiting,
    Spawned,
    Dead,
}

pub struct PlayerMarker;

pub fn player_select(
    mut target_reader: EventReader<CharacterEntityAction<SelectTarget>>,
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

            if let Some(target_index) = target_command.action().0 {
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

pub const PLAYER_ACTIONS_LABEL: &str = "player-abilitys";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let character_abilities = SystemSet::on_update(PlayerState::Spawned)
            .label(PLAYER_ACTIONS_LABEL)
            // TODO: Ordering
            .with_system(player_select.system());
        app.add_state(PlayerState::Waiting)
            .add_system_set(character_abilities);
    }
}
