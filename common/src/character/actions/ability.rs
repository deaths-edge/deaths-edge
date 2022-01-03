use bevy::{core::Time, prelude::*};
use heron::rapier_plugin::PhysicsWorld;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::character::{CastState, CharacterMarker, Class, LastCastInstant, Target};

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
            &Class,
            &LastCastInstant,
            &mut CastState,
            &Target,
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
            Class::Mars => match ability {
                Ability::Ability1 => {}
                _ => todo!(),
            },
            Class::Medea => match ability {
                Ability::Ability1 => {}
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
