use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use crate::character::{CharacterMarker, Interrupt};

use super::{AbilityMarker, CharacterId, Obstruction, UseObstructions};

#[derive(Debug, Default, Component)]
pub struct Fire;

#[derive(Debug, Default, Component)]
pub struct Frost;

#[derive(Debug, Default, Component)]
pub struct Nature;

pub fn check_lock<School: Component>(
    mut ability_query: Query<(&CharacterId, &School, &mut UseObstructions), With<AbilityMarker>>,
    character_query: Query<(), (With<CharacterMarker>, With<Interrupt<School>>)>,
) {
    for (source, spell_type, mut obstructions) in ability_query.iter_mut() {
        if character_query.get(source.0).is_ok() {
            obstructions.0.insert(Obstruction::Locked);
        }
    }
}

pub struct SchoolPlugin<T> {
    pub state: T,
}

impl<T> Plugin for SchoolPlugin<T>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Hash + Eq,
{
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(self.state)
            .with_system(check_lock::<Fire>)
            .with_system(check_lock::<Frost>)
            .with_system(check_lock::<Nature>);
        app.add_system_set(set);
    }
}
