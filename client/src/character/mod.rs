mod player;
mod reconcile;

use bevy::prelude::*;

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
    pub fn new(common: &CharacterBundle) -> Self {
        let size = common.class().size();

        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: common.class().color(),
                    custom_size: Some(Vec2::new(size.width, size.width)),
                    ..Default::default()
                },
                ..Default::default()
            },
            selected: Selected::default(),
        }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        let reconcile = SystemSet::on_update(ClientState::Arena)
            .label(RECONCILE_LABEL)
            // NETWORK_HANDLE_LABEL writes Reconcile events
            .after(NETWORK_HANDLE_LABEL)
            .with_system(reconcile);
        app.add_system_set(reconcile)
            .add_event::<Reconcile>()
            .add_plugin(PlayerPlugin)
            .add_plugin(CommonCharacterPlugin {
                state: ClientState::Arena,
            });
    }
}
