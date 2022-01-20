pub mod effects;
pub mod instances;
pub mod lifecycle;
mod magic_school;
pub mod obstructions;
mod target;

pub use magic_school::*;
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
            .add_plugin(effects);
    }
}
