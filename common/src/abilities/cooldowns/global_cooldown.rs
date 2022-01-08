use bevy::prelude::*;

use crate::{
    abilities::{
        AbilityId, AbilityInstanceMarker, AbilityMarker, CastType, CharacterId, Complete,
        Obstruction, UseObstructions,
    },
    character::{CharacterMarker, LastCastInstant, GLOBAL_COOLDOWN},
};

/// Ability obeys global cooldown.
#[derive(Default, Debug)]
pub struct GlobalCooldown;

pub fn check_global_cooldown(
    time: Res<Time>,
    mut ability_query: Query<
        (&CharacterId, &mut UseObstructions),
        (With<AbilityMarker>, With<GlobalCooldown>),
    >,
    character_query: Query<&LastCastInstant, With<CharacterMarker>>,
) {
    let last_update = time.last_update().expect("cannot find last update");

    for (source, mut obstructions) in ability_query.iter_mut() {
        let last_cast = character_query
            .get(source.0)
            .expect("missing ability source");
        if last_cast.0 + GLOBAL_COOLDOWN < last_update {
            obstructions.0.remove(&Obstruction::GlobalCooldown);
        } else {
            obstructions.0.insert(Obstruction::GlobalCooldown);
        }
    }
}

pub fn apply_global_cooldown(
    time: Res<Time>,

    instance_query: Query<&AbilityId, (With<Complete>, With<AbilityInstanceMarker>)>,
    ability_query: Query<
        (Entity, &CharacterId, &CastType),
        (With<AbilityMarker>, With<GlobalCooldown>),
    >,
    mut character_query: Query<&mut LastCastInstant, With<CharacterMarker>>,
) {
    let now = time.last_update().expect("failed to find last update");

    for instance_id in instance_query.iter() {
        let (_, source, cast_type) = ability_query
            .iter()
            .find(|(id, _, _)| *id == instance_id.0)
            .expect("failed to find ability");

        let mut last_cast_instant = character_query
            .get_mut(source.0)
            .expect("missing ability source");

        match cast_type {
            CastType::Instant => {
                last_cast_instant.0 = now;
            }
            CastType::Cast(duration) => {
                last_cast_instant.0 = now - *duration;
            }
            CastType::Channel(_) => todo!(),
        }
    }
}