mod damage;
mod heal;
mod interrupts;
mod target;

use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

pub use damage::*;
pub use heal::*;
pub use interrupts::*;
pub use target::*;
pub use target::*;

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
            .label("effects")
            .with_system(damage_effect_apply.system())
            .with_system(health_effect_apply.system())
            .with_system(interrupt_effect_apply.system());
        app.add_system_set(system_set);
    }
}

#[derive(Default)]
pub struct EffectMarker;
