use bevy::prelude::*;

use common::abilities::AbilityPlugin as CommonAbilityPlugin;

pub struct AbilityPlugin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AbilityState {
    Active,
    Inactive,
}

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AbilityState::Inactive)
            .add_plugin(CommonAbilityPlugin::new(AbilityState::Active));
    }
}
