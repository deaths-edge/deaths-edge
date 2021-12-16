use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use heron::{rapier_plugin::PhysicsWorld, Velocity};
use serde::{Deserialize, Serialize};

use crate::{effects::MovementInterruptBundle, spells::instances::fireball_action};

use super::{
    CharacterCastState, CharacterClass, CharacterMarker, CharacterSpeedMultiplier, CharacterTarget,
    LastCastInstant,
};

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum MotionDirection {
    Left,
    Forward,
    Right,
    Backward,
    LeftForward,
    LeftBackward,
    RightForward,
    RightBackward,
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct Motion(pub Option<MotionDirection>);

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
    mut events: EventReader<CharacterCommand<Action>>,

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
            CharacterClass::Mars => {}
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
                Action::Action2 => todo!(),
                Action::Action3 => todo!(),
                Action::Action4 => todo!(),
                Action::Action5 => todo!(),
                Action::Action6 => todo!(),
                Action::Action7 => todo!(),
                Action::Action8 => todo!(),
            },
            CharacterClass::Heka => {}
            CharacterClass::Pluto => {}
            CharacterClass::Mammon => {}
        }
    }
}

/// Receives [`Motion`] input and accelerates character in said direction.
pub fn character_movement(
    mut motion_events: EventReader<CharacterCommand<Motion>>,

    // CharacterIndex query
    mut character_query: Query<
        (
            Entity,
            &CharacterSpeedMultiplier,
            &mut Transform,
            &mut Velocity,
        ),
        With<CharacterMarker>,
    >,

    mut commands: Commands,
) {
    const FORWARD_SPEED: f32 = 1.0;
    const STRAFE_SPEED: f32 = 0.8;
    const BACKPEDDLE_SPEED: f32 = 0.6;

    for command in motion_events.iter() {
        let (character_entity, speed_multiplier, transform, mut velocity) = character_query
            .get_mut(command.id())
            .expect("failed to find character");

        // Construct direction
        let mut direction = match command.command().0 {
            None => Vec2::ZERO,
            Some(MotionDirection::Left) => Vec2::new(-STRAFE_SPEED, 0.),
            Some(MotionDirection::LeftForward) => Vec2::new(-STRAFE_SPEED, FORWARD_SPEED),
            Some(MotionDirection::Forward) => Vec2::new(0., FORWARD_SPEED),
            Some(MotionDirection::RightForward) => Vec2::new(STRAFE_SPEED, FORWARD_SPEED),
            Some(MotionDirection::Right) => Vec2::new(STRAFE_SPEED, 0.),
            Some(MotionDirection::RightBackward) => Vec2::new(STRAFE_SPEED, -BACKPEDDLE_SPEED),
            Some(MotionDirection::Backward) => Vec2::new(0., -BACKPEDDLE_SPEED),
            Some(MotionDirection::LeftBackward) => Vec2::new(-STRAFE_SPEED, -FORWARD_SPEED),
        };

        // TODO: Constify this
        if direction != Vec2::ZERO {
            // Normalize
            let mag = direction.length().max(1.);
            direction /= mag;

            commands
                .spawn()
                .insert_bundle(MovementInterruptBundle::new(character_entity));
        }

        let direction = transform.rotation * (direction.extend(0.));

        // Assign velocity
        *velocity = Velocity::from(direction * speed_multiplier.speed());
    }
}

pub struct CharacterCommand<Command> {
    id: Entity,
    command: Command,
}

impl<Command> CharacterCommand<Command> {
    pub fn new(id: Entity, command: Command) -> Self {
        Self { id, command }
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    pub fn command(&self) -> &Command {
        &self.command
    }
}

pub struct GameCommand<Command> {
    command: Command,
}

pub struct CharacterCommandPlugin<T> {
    state: T,
}

impl<T> CharacterCommandPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for CharacterCommandPlugin<T>
where
    T: Send + Sync + 'static + Debug + Eq + Hash + Clone + Copy,
{
    fn build(&self, app: &mut AppBuilder) {
        let movement = SystemSet::on_update(self.state)
            .with_system(character_movement.system())
            .with_system(character_action.system());
        app.add_event::<CharacterCommand<Motion>>()
            .add_event::<CharacterCommand<Action>>()
            .add_system_set(movement);
    }
}
