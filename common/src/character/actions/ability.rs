use bevy::{ecs::system::EntityCommands, prelude::*};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    abilities::{
        lifecycle::{CastBundle, InstantBundle},
        obstructions::UseObstructions,
        AbilityMarker, Source,
    },
    character::{Abilities, CharacterMarker, OptionalTarget},
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

impl Ability {
    pub fn as_index(&self) -> usize {
        use Ability::*;
        match self {
            Ability1 => 0,
            Ability2 => 1,
            Ability3 => 2,
            Ability4 => 3,
            Ability5 => 4,
            Ability6 => 5,
            Ability7 => 6,
            Ability8 => 7,
        }
    }
}

/// Receives an [`Ability`] and performs the associated ability.
// TODO: Split this into two systems? Instant/CastBundle
pub fn character_ability(
    // Ability events
    mut events: EventReader<CharacterEntityAction<Ability>>,

    mut character_query: Query<(Entity, &Abilities, &OptionalTarget), With<CharacterMarker>>,
    ability_query: Query<
        (
            &UseObstructions,
            Option<&InstantBundle>,
            Option<&CastBundle>,
        ),
        With<AbilityMarker>,
    >,

    mut commands: Commands,
) {
    for action in events.iter() {
        let (character_id, abilities, opt_target) = character_query
            .get_mut(action.id())
            .expect("character not found");

        let action = action.action;
        let ability_id = &abilities.0[action.as_index()];

        let (obstructions, instant_bundle, cast_bundle) = ability_query
            .get(ability_id.0)
            .expect("cannot find ability");

        if !obstructions.0.is_empty() {
            warn!(message = "cannot use ability", ?obstructions);
            continue;
        }

        let snapshot = |mut entity_commands: EntityCommands| {
            if let Some(target) = opt_target.0 {
                entity_commands.insert(target);
            }

            entity_commands
                .insert(Source(character_id))
                .insert(*ability_id)
                .id()
        };

        if let Some(instant_bundle_fn) = instant_bundle {
            info!("spawning instant bundle");
            let mut entity_commands = commands.spawn();
            instant_bundle_fn.0.apply(&mut entity_commands);

            snapshot(entity_commands);
            info!("spawned instant bundle");
        }

        if let Some(cast_bundle_fn) = cast_bundle {
            let mut entity_commands = commands.spawn();
            cast_bundle_fn.0.apply(&mut entity_commands);

            snapshot(entity_commands);
        }
    }
}
