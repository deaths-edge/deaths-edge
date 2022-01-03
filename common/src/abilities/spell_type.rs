use bevy::prelude::*;

use crate::character::{CharacterMarker, Interrupts};

use super::{AbilityMarker, AbilitySource, Obstruction, UseObstructions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellType {
    Fire,
    Frost,
    Nature,
    Holy,
    Unholy,
    Arcane,
    Physical,
}

pub fn check_lock(
    mut ability_query: Query<
        (&AbilitySource, &SpellType, &mut UseObstructions),
        With<AbilityMarker>,
    >,
    character_query: Query<&Interrupts, (With<CharacterMarker>, Changed<Interrupts>)>,
) {
    for (source, spell_type, mut obstructions) in ability_query.iter_mut() {
        if let Ok(interrupts) = character_query.get(source.0) {
            let is_locked = interrupts.is_locked(&spell_type);

            if is_locked {
                obstructions.0.insert(Obstruction::Locked);
            } else {
                obstructions.0.remove(&Obstruction::Locked);
            }
        }
    }
}
