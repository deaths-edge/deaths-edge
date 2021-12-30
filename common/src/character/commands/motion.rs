use bevy::{math::Vec2, prelude::*};
use heron::Velocity;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::CharacterEntityCommand;
use crate::{
    character::{CharacterMarker, CharacterSpeedMultiplier},
    effects::{EffectTarget, MovementInterruptBundle},
};

const FORWARD_SPEED: f32 = 1.0;
const STRAFE_SPEED: f32 = 0.8;
const BACKPEDDLE_SPEED: f32 = 0.6;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum ParallelMotion {
    Forward,
    Backward,
}

impl Into<Vec2> for ParallelMotion {
    fn into(self) -> Vec2 {
        match self {
            Self::Forward => Vec2::new(0., FORWARD_SPEED),
            Self::Backward => Vec2::new(0., -BACKPEDDLE_SPEED),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum NormalMotion {
    Left,
    Right,
}

impl Into<Vec2> for NormalMotion {
    fn into(self) -> Vec2 {
        match self {
            Self::Left => Vec2::new(-STRAFE_SPEED, 0.),
            Self::Right => Vec2::new(STRAFE_SPEED, 0.),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub struct Motion {
    pub parallel: Option<ParallelMotion>,
    pub normal: Option<NormalMotion>,
}

impl Into<Vec2> for Motion {
    fn into(self) -> Vec2 {
        let parallel: Vec2 = self.parallel.map(Into::into).unwrap_or_default();
        let normal: Vec2 = self.normal.map(Into::into).unwrap_or_default();
        parallel + normal
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
    for command in motion_events.iter() {
        let (character_entity, speed_multiplier, transform, mut velocity) = character_query
            .get_mut(command.id())
            .expect("failed to find character");

        // Construct direction
        let mut direction: Vec2 = command.command().clone().into();

        // TODO: Constify this
        if direction != Vec2::ZERO {
            // Normalize
            let mag = direction.length().max(1.);
            direction /= mag;

            commands
                .spawn()
                .insert_bundle(MovementInterruptBundle::new(EffectTarget(character_entity)));
        }

        let direction = transform.rotation * (direction.extend(0.));

        // Assign velocity
        *velocity = Velocity::from(direction * speed_multiplier.speed());

        info!(message = "motion", ?velocity);
    }
}
