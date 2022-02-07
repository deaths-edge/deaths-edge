use bevy::prelude::*;

use crate::{
    abilities::{AbilityMarker, Source},
    character::{CastId, CharacterMarker},
};

use super::{Obstruction, UseObstructions};

#[derive(Debug, Component)]
pub struct CantWhileCasting;

pub fn check_while_casting(
    character_query: Query<(), (With<CharacterMarker>, With<CastId>)>,
    mut ability_query: Query<
        (&Source, &mut UseObstructions),
        (With<AbilityMarker>, With<CantWhileCasting>),
    >,
) {
    for (Source(source), mut obstructions) in ability_query.iter_mut() {
        let is_casting = character_query.get(*source).is_ok();

        if is_casting {
            obstructions.0.insert(Obstruction::Casting);
        } else {
            obstructions.0.remove(&Obstruction::Casting);
        }
    }
}
