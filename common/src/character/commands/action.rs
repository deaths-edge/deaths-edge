use bevy::{core::Time, prelude::*};
use heron::rapier_plugin::PhysicsWorld;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    character::{
        CharacterCastState, CharacterClass, CharacterMarker, CharacterTarget, LastCastInstant,
    },
    spells::instances::{fireball_action, spear_action},
};

use super::CharacterEntityCommand;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    Action1,
    Action2,
    Action3,
    Action4,
    Action5,
    Action6,
    Action7,
    Action8,
}

/// Receives an [`Action`] and performs the associated action.
pub fn character_action(
    time: Res<Time>,
    physics_world: PhysicsWorld,

    // Action events
    mut events: EventReader<CharacterEntityCommand<Action>>,

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
    for command in events.iter() {
        let (
            character_entity,
            character_transform,
            character_class,
            last_cast_instant,
            mut character_cast_state,
            character_target,
        ) = character_query
            .get_mut(command.id())
            .expect("character not found");

        let action = command.command();
        match character_class {
            CharacterClass::Mars => match action {
                Action::Action1 => {
                    let result = spear_action(
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
            CharacterClass::Medea => match action {
                Action::Action1 => {
                    let result = fireball_action(
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
