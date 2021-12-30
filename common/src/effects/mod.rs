//! A categorization of the various effects which can be applied to characters.
mod buff;
mod control;
mod damage;
mod heal;
mod interrupts;
mod target;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

pub use buff::*;
pub use control::*;
pub use damage::*;
pub use heal::*;
pub use interrupts::*;
pub use target::*;

#[derive(Default)]
pub struct EffectMarker;

pub struct EffectPlugin<T> {
    state: T,
}

impl<T> EffectPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

pub const EFFECTS_LABEL: &str = "effects";

impl<T> Plugin for EffectPlugin<T>
where
    T: Sync + Send + Debug + Clone + Copy + Eq + Hash + 'static,
{
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::on_update(self.state)
            .label(EFFECTS_LABEL)
            .with_system(damage_effect_apply.system())
            .with_system(health_effect_apply.system())
            .with_system(interrupt_effect_apply.system())
            .with_system(control_effect_apply.system());
        app.add_system_set(system_set);
    }
}
