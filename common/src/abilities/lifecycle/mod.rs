mod cast;
mod instant;
mod status;

use std::{fmt::Debug, hash::Hash, time::Duration};

use bevy::{prelude::*, utils::Instant};

pub use cast::*;
pub use instant::*;
pub use status::*;

#[derive(Debug, Clone, Component)]
pub struct TotalDuration(pub Duration);

#[derive(Debug, Clone, Component)]
pub struct ProgressDuration(pub Duration);

#[derive(Debug, Component)]
pub struct Start(pub Instant);

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
            .with_system(cast_anchor);
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .after("anchor-casts")
            .with_system(cast_complete_spawn)
            .with_system(cast_despawn)
            .with_system(cast_complete)
            .with_system(cast_movement_interrupt)
            .with_system(instant_effect_despawn);
        app.add_system_set(set).add_system_set(anchor);
    }
}
