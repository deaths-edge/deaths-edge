use bevy::{core::Time, prelude::*};
use heron::rapier_plugin::PhysicsWorld;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    abilities::{
        AbilityInstance, AbilityMarker, AbilitySource, CastType, RequiresTarget, UseObstructions,
    },
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

    mut character_query: Query<
        (Entity, &mut CastState, &mut LastCastInstant),
        With<CharacterMarker>,
    >,
    ability_query: Query<
        (
            Entity,
            &AbilitySource,
            &CastType,
            Option<&RequiresTarget>,
            &UseObstructions,
        ),
        With<AbilityMarker>,
    >,

    mut commands: Commands,
) {
    let now = time.last_update().expect("failed to find last update");

    for action in events.iter() {
        let (character_id, mut cast, mut last_cast_instant) = character_query
            .get_mut(action.id())
            .expect("character not found");

        // Find ability
        // TODO: Shortcut this search?
        let (ability_id, _, cast_type, requires_target, obstructions) = ability_query
            .iter()
            .find(|(_, source, _, _, _)| source.0 == character_id)
            .expect("casted by unknown source");

        if !obstructions.0.is_empty() {
            warn!(message = "cannot use ability", ?obstructions);
            continue;
        }

        // Create instance of ability
        match cast_type {
            CastType::Instant => {
                // Update last cast instant
                last_cast_instant.0 = now;

                // requires_target

                let entity_commands = commands.spawn().insert(AbilityInstance(ability_id));

                if let Some(requires_target) = requires_target {
                    // entity_commands.insert()
                }
            }
            CastType::Cast(_) => {
                cast.0 = Some(Cast {
                    ability_id,
                    start: now,
                });
            }
            CastType::Channel(_) => todo!(),
        }
    }
}
