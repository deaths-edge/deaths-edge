use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use super::{
    obstructions::{Obstruction, UseObstructions},
    AbilityId, AbilityMarker,
};
use crate::character::{Abilities, CharacterMarker, Interrupted};

#[derive(Debug, Default, Clone, Component)]
pub struct Fire;

#[derive(Debug, Default, Clone, Component)]
pub struct Frost;

#[derive(Debug, Default, Clone, Component)]
pub struct Nature;

#[derive(Debug, Default, Clone, Component)]
pub struct Interruptable;

pub fn check_lock<School: Component>(
    character_query: Query<&Abilities, (With<CharacterMarker>, With<Interrupted<School>>)>,
    mut ability_query: Query<&mut UseObstructions, (With<AbilityMarker>, With<School>)>,
) {
    for abilities in character_query.iter() {
        for AbilityId(ability_id) in *abilities {
            if let Ok(mut obstructions) = ability_query.get_mut(ability_id) {
                obstructions.0.insert(Obstruction::Locked);
            }
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
