use bevy::{math::Vec2, prelude::*};
use heron::Velocity;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::CharacterEntityCommand;
use crate::{
    character::{CharacterMarker, CharacterSpeedMultiplier},
    effects::MovementInterruptBundle,
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
