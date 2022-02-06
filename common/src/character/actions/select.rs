use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    abilities::Target,
    character::{CharacterIndex, CharacterMarker},
};

use super::CharacterEntityAction;

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize, Serialize, Component)]
pub struct SelectTarget(pub Option<CharacterIndex>);

/// Receives an [`OptionalTarget`] and performs targeting.
pub fn character_target(
    // OptionalTarget events
    mut events: EventReader<CharacterEntityAction<SelectTarget>>,

    mut target_query: Query<Entity, With<CharacterMarker>>,
    index_query: Query<(Entity, &CharacterIndex), With<CharacterMarker>>,

    mut commands: Commands,
) {
    for action in events.iter() {
        let character_target = target_query
            .get_mut(action.id())
            .expect("character not found");

        let mut entity_commands = commands.entity(character_target);
        if let &SelectTarget(Some(target_index)) = action.action() {
            // TODO: Alert if this fails
            let target = index_query
                .iter()
                .find(|(_, index)| **index == target_index)
                .map(|(entity, _)| entity)
                .expect("failed to find index");

            entity_commands.insert(Target(target));
        } else {
            entity_commands.remove::<Target>();
        }
    }
}
