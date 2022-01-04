use bevy::{core::Time, prelude::*};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    abilities::{AbilityInstance, AbilityMarker, AbilitySource, CastType, UseObstructions},
    character::{Cast, CastState, CharacterMarker, LastCastInstant},
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

/// Receives an [`Ability`] and performs the associated ability.
pub fn character_ability(
    time: Res<Time>,

    // Ability events
    mut events: EventReader<CharacterEntityAction<Ability>>,

    mut character_query: Query<(Entity, &mut CastState), With<CharacterMarker>>,
    ability_query: Query<
        (Entity, &AbilitySource, &CastType, &UseObstructions),
        With<AbilityMarker>,
    >,

    mut commands: Commands,
) {
    for action in events.iter() {
        let (character_id, mut cast) = character_query
            .get_mut(action.id())
            .expect("character not found");

        // Find ability
        // TODO: Shortcut this search?
        let (ability_id, _, cast_type, obstructions) = ability_query
            .iter()
            .find(|(_, source, _, _)| source.0 == character_id)
            .expect("casted by unknown source");

        if !obstructions.0.is_empty() {
            warn!(message = "cannot use ability", ?obstructions);
            continue;
        }

        // Create instance of ability
        match cast_type {
            CastType::Instant => {
                commands.spawn().insert(AbilityInstance(ability_id));
            }
            CastType::Cast(_) => {
                // cast.0 = Some(Cast {
                //     ability_id,
                //     start: now,
                // });
            }
            CastType::Channel(_) => todo!(),
        }
    }
}
