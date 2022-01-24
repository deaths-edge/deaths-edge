use bevy::prelude::*;

use crate::{
    abilities::{AbilityMarker, Source},
    character::{CastState, CharacterMarker},
};

use super::{Obstruction, UseObstructions};

#[derive(Debug, Component)]
pub struct CantWhileCasting;

pub fn check_while_casting(
    character_query: Query<&CastState, With<CharacterMarker>>,
    mut ability_query: Query<
        (&Source, &mut UseObstructions),
        (With<AbilityMarker>, With<CantWhileCasting>),
    >,
) {
    for (Source(source), mut obstructions) in ability_query.iter_mut() {
        let cast_state = character_query
            .get(*source)
            .expect("failed to find character");

        if cast_state.0.is_some() {
            obstructions.0.insert(Obstruction::Casting);
        } else {
            obstructions.0.remove(&Obstruction::Casting);
        }
    }
}
