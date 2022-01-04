use std::time::Duration;

use bevy::prelude::*;
use heron::Velocity;

use crate::character::{CastState, CharacterEntityAction, CharacterMarker, Motion};

use super::{
    AbilityInstance, AbilityMarker, AbilitySource, Casting, Failed, InstantInterrupt, Obstruction,
    UseObstructions,
};

pub struct RequiresStationary;

pub fn check_required_stationary(
    mut ability_query: Query<
        (&AbilitySource, &mut UseObstructions),
        (With<AbilityMarker>, With<RequiresStationary>),
    >,
    character_query: Query<&Velocity, (With<CharacterMarker>, Changed<Velocity>)>,
) {
    for (source, mut obstructions) in ability_query.iter_mut() {
        if let Ok(velocity) = character_query.get(source.0) {
            if velocity.linear == Vec3::ZERO {
                obstructions.0.remove(&Obstruction::NonStationary);
            } else {
                obstructions.0.insert(Obstruction::NonStationary);
            }
        }
    }
}

pub fn motion_interrupt(
    mut motion: EventReader<CharacterEntityAction<Motion>>,

    mut commands: Commands,
) {
    // commands
    //     .spawn()
    //     .insert(InstantInterrupt(Duration::default()))
}
