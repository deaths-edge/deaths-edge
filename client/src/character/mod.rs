mod classes;
mod player;
mod reconcile;

use bevy::prelude::*;

pub use classes::*;
pub use player::*;
pub use reconcile::*;

use common::{character::CharacterPlugin as CommonCharacterPlugin, network::server::Reconcile};

use crate::{network::NETWORK_HANDLE_LABEL, GameState};

#[derive(Bundle)]
pub struct ClientCharacterBundle {
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CharacterState {
    Active,
    Inactive,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Only reconcile in arena?
        let reconcile = SystemSet::on_update(GameState::Arena)
            .label(RECONCILE_LABEL)
            // NETWORK_HANDLE_LABEL writes Reconcile events
            .after(NETWORK_HANDLE_LABEL)
            .with_system(reconcile);
        app.add_state(CharacterState::Inactive)
            .add_system_set(reconcile)
            .add_event::<Reconcile>()
            .add_plugin(PlayerPlugin)
            .add_plugin(CommonCharacterPlugin {
                state: CharacterState::Active,
            });
    }
}
