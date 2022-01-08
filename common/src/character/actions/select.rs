use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::character::{CharacterIndex, CharacterMarker, OptionalTarget};

use super::CharacterEntityAction;

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct SelectTarget(pub Option<CharacterIndex>);

/// Receives an [`OptionalTarget`] and performs targeting.
pub fn character_target(
    // OptionalTarget events
    mut events: EventReader<CharacterEntityAction<SelectTarget>>,

    mut target_query: Query<&mut OptionalTarget, With<CharacterMarker>>,
    index_query: Query<(Entity, &CharacterIndex), With<CharacterMarker>>,
) {
    for action in events.iter() {
        let mut character_target = target_query
            .get_mut(action.id())
            .expect("character not found");

        if let Some(target_index) = action.action().0 {
            let target_entity = index_query
                .iter()
                .find(|(_, index)| **index == target_index)
                .map(|(entity, _)| entity)
                .expect("failed to find target");
            *character_target = OptionalTarget(Some(target_entity));
        } else {
            *character_target = OptionalTarget(None);
        }
    }
}
