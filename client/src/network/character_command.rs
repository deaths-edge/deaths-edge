use std::marker::PhantomData;

use bevy::prelude::*;

use common::{
    character::{CharacterEntityAction, CharacterIndex, CharacterMarker, CHARACTER_COMMANDS},
    network::{client::ClientMessage, CharacterNetworkAction},
};

use crate::{character::PlayerState, input_mapping::INPUT_TO_CHARACTER_LABEL, state::ClientState};

use super::{player_input_to_network, NETWORK_HANDLE_LABEL};

pub fn network_to_entity_command<T>(
    query: Query<(Entity, &CharacterIndex), With<CharacterMarker>>,
    mut character_network_reader: EventReader<CharacterNetworkAction<T>>,
    mut character_entity_writer: EventWriter<CharacterEntityAction<T>>,
) where
    T: Send + Sync + 'static,
    T: Clone,
{
    let commands = character_network_reader
        .iter()
        .filter_map(|network_command| {
            query
                .iter()
                .find(|(_, index)| **index == network_command.index)
                .map(|(id, _)| CharacterEntityAction::new(id, network_command.action.clone()))
        });
    character_entity_writer.send_batch(commands);
}

pub struct CharacterNetworkActionPlugin<T> {
    _command: PhantomData<T>,
}

impl<T> CharacterNetworkActionPlugin<T> {
    pub fn new() -> Self {
        Self {
            _command: PhantomData,
        }
    }
}

pub const CHARACTER_NETWORK_COMMAND_LABEL: &str = "broadcast-inputs";

pub const NETWORK_TO_ENTITY_LABEL: &str = "network-to-entity";

impl<T> Plugin for CharacterNetworkActionPlugin<T>
where
    T: Send + Sync + 'static,
    T: Clone,
    T: Into<ClientMessage>,
{
    fn build(&self, app: &mut AppBuilder) {
        let broadcast_inputs = SystemSet::on_update(PlayerState::Spawned)
            .label(CHARACTER_NETWORK_COMMAND_LABEL)
            // INPUT_TO_CHARACTER_LABEL sends PlayerInputAction<Value> events
            .after(INPUT_TO_CHARACTER_LABEL)
            .with_system(player_input_to_network::<T>.system());

        let network_to_entity = SystemSet::on_update(ClientState::Arena)
            .label(NETWORK_TO_ENTITY_LABEL)
            // NETWORK_HANDLE_LABEL sends CharacterNetworkAction<Value> events
            .after(NETWORK_HANDLE_LABEL)
            // CHARACTER_COMMANDS reads CharacterEntityAction<Value> events
            .before(CHARACTER_COMMANDS)
            .with_system(network_to_entity_command::<T>.system());

        app.add_event::<CharacterNetworkAction<T>>()
            .add_system_set(broadcast_inputs)
            .add_system_set(network_to_entity);
    }
}
