pub mod effects;
mod info;
pub mod instances;
pub mod lifecycle;
pub mod magic_school;
pub mod obstructions;
mod target;

pub use target::*;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use effects::EffectPlugin;
use lifecycle::*;
use obstructions::*;

#[derive(Default, Debug, Component)]
pub struct AbilityMarker;

#[derive(Debug, Clone, Copy, Component)]
pub struct AbilityId(pub Entity);

pub struct AbilityPlugin<T> {
    state: T,
}

impl<T> AbilityPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbilityLabels {
    Checks,
    Effects,
    Lifecycle,
}

impl SystemLabel for AbilityLabels {
    fn dyn_clone(&self) -> Box<dyn SystemLabel> {
        Box::new(self.clone())
    }
}

pub fn progress_durations(time: Res<Time>, mut query: Query<&mut ProgressDuration>) {
    for mut progress in query.iter_mut() {
        progress.0 += time.delta();
    }
}

impl<T> Plugin for AbilityPlugin<T>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Hash + Eq,
{
    fn build(&self, app: &mut App) {
        let effects = EffectPlugin {
            state: self.state,
            label: AbilityLabels::Effects,
        };

        let obstructions = ObstructionPlugin {
            state: self.state,
            label: AbilityLabels::Checks,
        };

        let lifecycle = LifecyclePlugin {
            state: self.state,
            label: AbilityLabels::Lifecycle,
        };

        app.add_plugin(lifecycle)
            .add_plugin(obstructions)
            .add_plugin(effects)
            .add_system(progress_durations);
    }
}
