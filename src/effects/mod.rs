mod damage;
mod heal;
mod interrupt;
mod target;

use bevy::prelude::*;

pub use damage::*;
pub use heal::*;
pub use interrupt::*;
pub use target::*;
pub use target::*;

use crate::state::AppState;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let system_set = SystemSet::on_update(AppState::Arena)
            .label("effects")
            .with_system(damage_effect_apply.system())
            .with_system(health_effect_apply.system())
            .with_system(interrupt_effect_apply.system());
        app.add_system_set(system_set);
    }
}

#[derive(Default)]
pub struct EffectMarker;
