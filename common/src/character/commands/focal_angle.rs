use bevy::prelude::*;
use heron::Velocity;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::CharacterEntityCommand;
use crate::character::CharacterMarker;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FocalAngle(pub f32);

impl FocalAngle {
    pub fn almost_eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() * 512. < std::f32::consts::PI
    }
}

/// Receives [`FocalAngle`] event and rotates character in that direction.
pub fn character_focal_rotate(
    mut character_query: Query<(&mut Transform, &mut Velocity), With<CharacterMarker>>,
    mut events: EventReader<CharacterEntityCommand<FocalAngle>>,
) {
    if let Some(command) = events.iter().last() {
        info!(message = "rotating", angle = %command.command.0);
        let (mut transform, mut velocity) = character_query
            .get_mut(command.id)
            .expect("player not found");

        let new_rotation = Quat::from_rotation_z(command.command.0);
        let rotation_delta = transform.rotation.inverse() * new_rotation;

        transform.rotation = new_rotation;

        velocity.linear = rotation_delta * velocity.linear;
    }
}
