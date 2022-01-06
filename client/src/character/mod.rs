mod materials;
mod player;
mod reconcile;

use bevy::prelude::*;

pub use materials::*;
pub use player::*;
pub use reconcile::*;

use common::{
    character::{CharacterBundle, CharacterPlugin as CommonCharacterPlugin},
    network::server::Reconcile,
};

use crate::{network::NETWORK_HANDLE_LABEL, state::ClientState, ui::selected::Selected};

#[derive(Bundle)]
pub struct ClientCharacterBundle {
    #[bundle]
    sprite: SpriteBundle,
    selected: Selected,
}

impl ClientCharacterBundle {
    pub fn new(common: &CharacterBundle, materials: &CharacterMaterials) -> Self {
        let size = common.class().size();

        Self {
            sprite: SpriteBundle {
                material: materials.handle(common.class()).clone(),
                sprite: Sprite::new(Vec2::new(size.width, size.width)),
                ..Default::default()
            },
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
            .add_plugin(CharacterMaterialPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CommonCharacterPlugin {
                state: ClientState::Arena,
            });
    }
}
