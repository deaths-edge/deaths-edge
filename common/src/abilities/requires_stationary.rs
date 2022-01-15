use bevy::prelude::*;
use heron::Velocity;

use crate::character::{CastState, CharacterEntityAction, CharacterMarker, Motion};

use super::{
    AbilityId, AbilityInstanceMarker, AbilityMarker, Casting, CharacterId, Failed, Obstruction,
    UseObstructions,
};

/// Requires that target is stationary while casting.
#[derive(Default, Debug, Component)]
pub struct RequiresStationary;

pub fn check_required_stationary(
    mut ability_query: Query<
        (&CharacterId, &mut UseObstructions),
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
    mut motion_events: EventReader<CharacterEntityAction<Motion>>,

    cast_instance_query: Query<(Entity, &AbilityId), (With<Casting>, With<AbilityInstanceMarker>)>,
    cast_ability_query: Query<(), (With<AbilityMarker>, With<RequiresStationary>)>,

    mut character_query: Query<&mut CastState, With<CharacterMarker>>,

    mut commands: Commands,
) {
    for motion in motion_events.iter() {
        if !motion.action.is_stationary() {
            let mut cast_state = character_query
                .get_mut(motion.id)
                .expect("failed to find character");
            let instance_id_opt = if let Some(cast) = &cast_state.0 {
                let (instance_id, ability_id) = cast_instance_query
                    .get(cast.instance_id)
                    .expect("failed to find ability instance");
                if cast_ability_query.get(ability_id.0).is_ok() {
                    Some(instance_id)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(instance_id) = instance_id_opt {
                commands
                    .entity(instance_id)
                    .remove::<Casting>()
                    .insert(Failed);

                cast_state.0 = None;
            }
        }
    }
}
