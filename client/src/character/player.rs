use std::{fmt::Debug, hash::Hash};

use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::*;
use crate::input_mapping::SelectClick;

use common::{character::CharacterTarget, network::server::SpawnCharacter};

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
            .with_system(player_char_select.system());
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

/// Receives a [`SelectClick`] event and selects a character.
pub fn player_char_select(
    mut select_clicks: EventReader<SelectClick>,
    mut character_query: QuerySet<(
        Query<(Entity, &mut Selected)>,
        Query<(Entity, &Transform, &Sprite, &mut Selected)>,
        Query<&mut CharacterTarget, With<PlayerMarker>>,
    )>,
) {
    const SELECT_SIZE: (f32, f32) = (30., 30.);

    // Get selected click
    let select_click = if let Some(some) = select_clicks.iter().last() {
        some
    } else {
        return;
    };

    // Find and set selection
    let selected_entity_opt = character_query
        .q1_mut()
        .iter_mut()
        .find(|(_, char_transform, char_sprite, _)| {
            collide(
                select_click.mouse_position.extend(0.),
                SELECT_SIZE.into(),
                char_transform.translation,
                char_sprite.size,
            )
            .is_some()
        })
        .map(|(entity, _, _, selected)| (entity, selected))
        .map(|(entity, mut selected)| {
            // Set selection
            *selected = Selected::Selected;

            entity
        });

    // Set character selection
    if let Ok(mut character_target) = character_query.q2_mut().single_mut() {
        if let Some(index) = selected_entity_opt {
            tracing::info!(message = "selected character", ?index);
            character_target.set_entity(index);
        } else {
            character_target.deselect();
        }
    };

    // Deselect everything else
    for (_, mut selected) in character_query
        .q0_mut()
        .iter_mut()
        .filter(|(entity, _)| Some(*entity) != selected_entity_opt)
    {
        *selected = Selected::Unselected;
    }
}
