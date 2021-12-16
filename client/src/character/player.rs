use std::{fmt::Debug, hash::Hash};

use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::*;
use crate::input_mapping::{FocalHold, SelectClick};

use common::character::CharacterTarget;

pub struct PlayerPlugin<T> {
    state: T,
}

impl<T> PlayerPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for PlayerPlugin<T>
where
    T: Sync + Send + Debug + Clone + Copy + Eq + Hash + 'static,
{
    fn build(&self, app: &mut AppBuilder) {
        let character_motion = SystemSet::on_update(self.state)
            .label("character-motion")
            .with_system(player_focal_rotate.system());
        let character_actions = SystemSet::on_update(self.state)
            .label("character-actions")
            .with_system(player_char_select.system());
        app.add_system_set(character_motion)
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

/// Receives [`FocalHold`] event and rotates character in that direction.
pub fn player_focal_rotate(
    mut character_query: Query<&mut Transform, With<PlayerMarker>>,
    mut events: EventReader<FocalHold>,
) {
    let mut transform = character_query.single_mut().expect("player not found");

    const MINIMUM_FOCAL_LENGTH: f32 = 200.;

    if let Some(event) = events.iter().last() {
        let translation = Vec2::from(transform.translation);

        let diff = event.mouse_position - translation;
        let distance = diff.length();
        let adjustment = distance.min(MINIMUM_FOCAL_LENGTH);
        let new_diff = diff * adjustment / distance;
        // let new_diff = diff;

        let angle = Vec2::new(0., 1.).angle_between(new_diff);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}
