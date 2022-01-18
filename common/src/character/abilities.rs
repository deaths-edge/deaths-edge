use std::{array::IntoIter, iter::Map};

use bevy::{
    ecs::query::{Fetch, FilterFetch, WorldQuery},
    prelude::*,
};

use crate::abilities::AbilityId;

#[derive(Debug, Clone, Copy, Component)]
pub struct Abilities(pub [AbilityId; 8]);

impl IntoIterator for Abilities {
    type Item = AbilityId;

    type IntoIter = IntoIter<AbilityId, 8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Abilities {
    pub fn iter_over<'world, 'state, Q, F>(
        &'state self,
        query: &'state Query<'world, 'state, Q, F>,
    ) -> impl Iterator<Item = <<Q as WorldQuery>::ReadOnlyFetch as Fetch<'world, 'state>>::Item>
    where
        Q: WorldQuery,
        F: WorldQuery,
        F::Fetch: FilterFetch,
    {
        self.0
            .iter()
            .map(|AbilityId(id)| id)
            .cloned()
            .filter_map(|id| query.get(id).ok())
    }

    // pub fn iter_mut_over<Q, F>(&self, query: &mut Query<Q, F>) -> impl Iterator<Item = QueryItem<Q>>
    // where
    //     Q: WorldQuery,
    //     F: WorldQuery,
    //     F::Fetch: FilterFetch,
    // {
    //     let query = &mut query;
    //     let x = self
    //         .0
    //         .iter()
    //         .map(|AbilityId(id)| id)
    //         .cloned()
    //         .map(|id| query.get_mut(id))
    //         .filter_map(Result::ok);

    //     x
    // }
}
