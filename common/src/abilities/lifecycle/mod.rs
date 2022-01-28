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
        let instant_plugin = InstantPlugin {
            state: self.state,
            label: self.label.clone(),
        };
        let cast_plugin = CastPlugin {
            state: self.state,
            label: self.label.clone(),
        };
        let status_plugin = StatusPlugin {
            state: self.state,
            label: self.label.clone(),
        };
        app.add_plugin(instant_plugin)
            .add_plugin(cast_plugin)
            .add_plugin(status_plugin);
    }
}
