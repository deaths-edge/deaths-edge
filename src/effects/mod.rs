mod damage;
mod heal;
mod interupt;
mod target;

use std::time::Duration;

use bevy::prelude::*;

pub use damage::*;
pub use heal::*;
pub use interupt::*;
pub use target::*;
pub use target::*;

pub struct EffectPlugin;

const EFFECT_SET: &str = "effects";

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::new()
            .label(EFFECT_SET)
            .with_system(damage_effect_apply.system())
            .with_system(health_effect_apply.system())
            .with_system(interupt_effect_apply.system());
        app.add_system_set(system_set);
    }
}

#[derive(Default)]
pub struct EffectMarker;
