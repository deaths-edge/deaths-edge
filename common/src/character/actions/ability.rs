use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    abilities::{instances::OnPress, obstructions::UseObstructions, AbilityMarker},
    character::{Abilities, CharacterMarker},
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
pub fn character_ability(
    // Ability events
    mut events: EventReader<CharacterEntityAction<Ability>>,

    mut character_query: Query<(Entity, &Abilities), With<CharacterMarker>>,
    ability_query: Query<(&UseObstructions, Option<&OnPress>), With<AbilityMarker>>,

    mut commands: Commands,
) {
    for action in events.iter() {
        let (character_id, abilities) = character_query
            .get_mut(action.id())
            .expect("character not found");

        let action = action.action;
        let ability_id = &abilities.0[action.as_index()];

        let (obstructions, on_press_opt) = ability_query
            .get(ability_id.0)
            .expect("cannot find ability");

        if !obstructions.0.is_empty() {
            warn!(message = "cannot use ability", ?obstructions);
            continue;
        }

        if let Some(on_press) = on_press_opt {
            info!("spawning instant bundle");
            let mut entity_commands = commands.spawn();
            on_press.0.apply(character_id, &mut entity_commands);

            info!("spawned instant bundle");
        }
    }
}
