use bevy::{ecs::system::EntityCommands, prelude::*};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    abilities::{
        AbilityMarker, CastBundle, CharacterId, InstantBundle, InstantEffectsMarker,
        UseObstructions,
    },
    character::{Cast, CastState, CharacterMarker, OptionalTarget},
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
// TODO: Split this into two systems? Instant/CastBundle
pub fn character_ability(
    time: Res<Time>,

    // Ability events
    mut events: EventReader<CharacterEntityAction<Ability>>,

    mut character_query: Query<(Entity, &OptionalTarget, &mut CastState), With<CharacterMarker>>,
    ability_query: Query<
        (
            &CharacterId,
            &UseObstructions,
            Option<&InstantBundle>,
            Option<&CastBundle>,
        ),
        With<AbilityMarker>,
    >,

    mut commands: Commands,
) {
    let now = time.last_update().expect("failed to find last update");
    for action in events.iter() {
        let (character_id, opt_target, mut cast_state) = character_query
            .get_mut(action.id())
            .expect("character not found");

        // Find ability
        // TODO: Shortcut this search?
        let (_, obstructions, instant_bundle, cast_bundle) = ability_query
            .iter()
            .find(|(source, _, _, _)| source.0 == character_id)
            .expect("casted by unknown source");

        if !obstructions.0.is_empty() {
            warn!(message = "cannot use ability", ?obstructions);
            continue;
        }

        let snapshot = |mut entity_commands: EntityCommands| {
            if let Some(target) = opt_target.0 {
                entity_commands.insert(target);
            }

            entity_commands.id()
        };

        if let Some(instant_bundle_fn) = instant_bundle {
            let mut entity_commands = commands.spawn();

            entity_commands
                .insert(InstantEffectsMarker)
                .insert_bundle(instant_bundle_fn.0());

            snapshot(entity_commands);
        }

        if let Some(cast_bundle_fn) = cast_bundle {
            let mut entity_commands = commands.spawn();

            entity_commands
                .insert(InstantEffectsMarker)
                .insert_bundle(cast_bundle_fn.0());

            let cast_id = snapshot(entity_commands);

            let cast = Cast {
                start: now,
                cast_id,
            };
            cast_state.0 = Some(cast);
        }
    }
}
