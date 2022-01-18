use bevy::prelude::*;

use common::abilities::AbilityPlugin as CommonAbilityPlugin;

use crate::state::ClientState;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CommonAbilityPlugin::new(ClientState::Arena));
    }
}
