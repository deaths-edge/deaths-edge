use bevy::prelude::*;

use common::{
    character::{CharacterEntityCommand, CharacterIndex, CharacterMarker},
    network::CharacterNetworkCommand,
};

pub fn network_to_entity_command<T>(
    query: Query<(Entity, &CharacterIndex), With<CharacterMarker>>,
    mut character_network_reader: EventReader<CharacterNetworkCommand<T>>,
    mut character_entity_writer: EventWriter<CharacterEntityCommand<T>>,
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
                .map(|(id, _)| CharacterEntityCommand::new(id, network_command.command.clone()))
        });
    character_entity_writer.send_batch(commands);
}
