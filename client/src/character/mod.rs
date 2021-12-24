mod materials;
mod player;
mod reconcile;

use bevy::prelude::*;

pub use materials::*;
pub use player::*;
pub use reconcile::*;

use common::{
    character::{
        CastingPlugin, CharacterBundle as CommonCharacterBundle, CharacterEntityCommandPlugin,
    },
    network::server::Reconcile,
};

use crate::{network::NETWORK_HANDLE_LABEL, state::ClientState, ui::selected::Selected};

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    common: CommonCharacterBundle,
    selected: Selected,
}

impl CharacterBundle {
    pub fn new(
        transform: Transform,
        common: CommonCharacterBundle,
        materials: &CharacterMaterials,
    ) -> Self {
        let size = common.class().size();

        Self {
            sprite: SpriteBundle {
                material: materials.handle(common.class()).clone(),
                transform,
                sprite: Sprite::new(Vec2::new(size.width, size.width)),
                ..Default::default()
            },
            common,
            selected: Selected::default(),
        }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let reconcile = SystemSet::on_update(ClientState::Arena)
            .label(RECONCILE_LABEL)
            // NETWORK_HANDLE_LABEL writes Reconcile events
            .after(NETWORK_HANDLE_LABEL)
            .with_system(reconcile.system());
        app.add_system_set(reconcile)
            .add_event::<Reconcile>()
            .add_plugin(CharacterEntityCommandPlugin::new(ClientState::Arena))
            .add_plugin(CharacterMaterialPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CastingPlugin::new(ClientState::Arena));
    }
}
