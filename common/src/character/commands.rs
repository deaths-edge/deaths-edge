use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use heron::{rapier_plugin::PhysicsWorld, Velocity};
use serde::{Deserialize, Serialize};

use crate::{
    effects::MovementInterruptBundle, network::NETWORK_POLL_LABEL,
    spells::instances::fireball_action,
};

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

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FocalAngle(pub f32);

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
    mut motion_events: EventReader<CharacterEntityCommand<Motion>>,

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

        info!(message = "motion", ?velocity);
    }
}

/// Receives [`FocalAngle`] event and rotates character in that direction.
pub fn character_focal_rotate(
    mut character_query: Query<&mut Transform, With<CharacterMarker>>,
    mut events: EventReader<CharacterEntityCommand<FocalAngle>>,
) {
    if let Some(command) = events.iter().last() {
        info!(message = "rotating", angle = %command.command.0);
        let mut transform = character_query
            .get_mut(command.id)
            .expect("player not found");

        transform.rotation = Quat::from_rotation_z(command.command.0);
    }
}

pub const CHARACTER_COMMANDS: &str = "character-commands";

/// A character command, addressed by [`Entity`].
pub struct CharacterEntityCommand<Command> {
    id: Entity,
    command: Command,
}

impl<Command> CharacterEntityCommand<Command> {
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

pub struct CharacterEntityCommandPlugin<T> {
    state: T,
}

impl<T> CharacterEntityCommandPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

impl<T> Plugin for CharacterEntityCommandPlugin<T>
where
    T: Send + Sync + 'static + Debug + Eq + Hash + Clone + Copy,
{
    fn build(&self, app: &mut AppBuilder) {
        let movement = SystemSet::on_update(self.state)
            .label(CHARACTER_COMMANDS)
            .after(NETWORK_POLL_LABEL)
            .with_system(character_movement.system())
            .with_system(character_action.system())
            .with_system(character_focal_rotate.system());
        app.add_event::<CharacterEntityCommand<Motion>>()
            .add_event::<CharacterEntityCommand<Action>>()
            .add_event::<CharacterEntityCommand<FocalAngle>>()
            .add_system_set(movement);
    }
}
