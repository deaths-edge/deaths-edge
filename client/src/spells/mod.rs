mod instances;
mod materials;

use bevy::prelude::*;

use common::spells::SpellPlugin as CommonSpellPlugin;

pub use instances::*;
pub use materials::*;

use crate::state::ClientState;

pub struct SpellPlugin;

pub const SPELLS_LABEL: &str = "spells";

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpellMaterials>()
            .add_plugin(CommonSpellPlugin::new(ClientState::Arena));
    }
}
