mod cast;
mod instant;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

pub use cast::*;
pub use instant::*;

pub struct LifecyclePlugin<T, L> {
    pub state: T,
    pub label: L,
}

impl<T, L> Plugin for LifecyclePlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,
{
    fn build(&self, app: &mut App) {
        let anchor = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .label("anchor-casts")
            .with_system(anchor_cast);
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .after("anchor-casts")
            .with_system(spawn_complete_cast)
            .with_system(despawn_cast)
            .with_system(cast_complete)
            .with_system(cast_movement_interrupt);
        app.add_system_set(set).add_system_set(anchor);
    }
}
