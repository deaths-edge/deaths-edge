use bevy::{core::Time, prelude::*};
use heron::rapier_plugin::PhysicsWorld;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    character::{
        CharacterCastState, CharacterClass, CharacterMarker, CharacterTarget, LastCastInstant,
    },
    spells::instances::{fireball_ability, spear_ability},
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
    physics_world: PhysicsWorld,

    // Ability events
    mut events: EventReader<CharacterEntityAction<Ability>>,

    mut character_query: Query<
        (
            Entity,
            &Transform,
            &CharacterClass,
            &LastCastInstant,
            &mut CharacterCastState,
            &CharacterTarget,
        ),
        With<CharacterMarker>,
    >,

    target_query: Query<&Transform, With<CharacterMarker>>,
) {
    for action in events.iter() {
        let (
            character_entity,
            character_transform,
            character_class,
            last_cast_instant,
            mut character_cast_state,
            character_target,
        ) = character_query
            .get_mut(action.id())
            .expect("character not found");

        let ability = action.action();
        match character_class {
            CharacterClass::Mars => match ability {
                Ability::Ability1 => {
                    let result = spear_ability(
                        &time,
                        &physics_world,
                        last_cast_instant,
                        character_entity,
                        character_transform,
                        character_target,
                        &mut character_cast_state,
                        &target_query,
                    );

                    if let Err(error) = result {
                        warn!(message = "failed to cast fireball", %error)
                    }
                }
                _ => todo!(),
            },
            CharacterClass::Medea => match ability {
                Ability::Ability1 => {
                    let result = fireball_ability(
                        &time,
                        &physics_world,
                        last_cast_instant,
                        character_entity,
                        character_transform,
                        character_target,
                        &mut character_cast_state,
                        &target_query,
                    );

                    if let Err(error) = result {
                        warn!(message = "failed to cast fireball", %error)
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
