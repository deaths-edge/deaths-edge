use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    abilities::{
        AbilityId, AbilityInstanceMarker, AbilityMarker, CharacterId, DeleteObstructions,
        Preparing, UseObstructions,
    },
    character::CharacterMarker,
};

use super::CharacterEntityAction;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Ability {
    Ability1,
    Ability2,
    Ability3,
    Ability4,
    Ability5,
    Ability6,
    Ability7,
    Ability8,
}

#[derive(Debug, Bundle)]
pub struct BaseAbilityInstance {
    marker: AbilityInstanceMarker,
    ability_id: AbilityId,
    delete_obstructions: DeleteObstructions,
}

/// Receives an [`Ability`] and performs the associated ability.
pub fn character_ability(
    // Ability events
    mut events: EventReader<CharacterEntityAction<Ability>>,

    mut character_query: Query<Entity, With<CharacterMarker>>,
    ability_query: Query<(Entity, &CharacterId, &UseObstructions), With<AbilityMarker>>,

    mut commands: Commands,
) {
    for action in events.iter() {
        let character_id = character_query
            .get_mut(action.id())
            .expect("character not found");

        // Find ability
        // TODO: Shortcut this search?
        let (ability_id, _, obstructions) = ability_query
            .iter()
            .find(|(_, source, _)| source.0 == character_id)
            .expect("casted by unknown source");

        if !obstructions.0.is_empty() {
            warn!(message = "cannot use ability", ?obstructions);
            continue;
        }

        // Create instance of ability
        commands
            .spawn()
            .insert_bundle(BaseAbilityInstance {
                marker: AbilityInstanceMarker,
                ability_id: AbilityId(ability_id),
                delete_obstructions: DeleteObstructions::default(),
            })
            .insert(Preparing);
    }
}
