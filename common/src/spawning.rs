use bevy::prelude::*;

use crate::{character::CharacterBundle, network::server::SpawnCharacter};

pub fn spawn_character_base<F>(
    time: &Time,
    spawn_reader: &mut EventReader<SpawnCharacter>,
    mut f: F,
) where
    F: FnMut(CharacterBundle, &SpawnCharacter),
{
    for spawn_event in spawn_reader.iter() {
        let common = CharacterBundle::new(spawn_event.index(), spawn_event.class(), time);
        f(common, spawn_event);
    }
}
