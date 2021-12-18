use std::iter::once;

use bevy::prelude::*;

use common::{
    character::{
        CharacterBundle as CommonCharacterBundle, CharacterClass, CharacterIndex, CharacterMarker,
    },
    game::GameRoster,
    network::{
        server::{GameCommand, ServerMessage, SpawnCharacter},
        NetworkSendEvent, Packetting,
    },
    state::SpawningState,
};

use crate::{
    character::{CharacterBundle, ClientAddress},
    network::{NETWORK_HANDLE_LABEL, NETWORK_SEND_LABEL},
};

pub const SPAWNER_LABEL: &str = "spawner";

// TODO: Add waiting state
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawner = SystemSet::on_update(SpawningState::Active)
            .label(SPAWNER_LABEL)
            .after(NETWORK_HANDLE_LABEL)
            .before(NETWORK_SEND_LABEL)
            .with_system(spawn_characters.system());

        // TODO: Don't start active
        app.add_state(SpawningState::Deactive)
            .add_system_set(spawner);
    }
}

pub fn spawn_characters(
    time: Res<Time>,

    mut next_index: Local<CharacterIndex>,
    mut game_roster: ResMut<GameRoster>,

    character_address_query: Query<&ClientAddress, With<CharacterMarker>>,
    character_existing_query: Query<
        (&CharacterIndex, &CharacterClass, &Transform),
        With<CharacterMarker>,
    >,
    mut network_writer: EventWriter<NetworkSendEvent<ServerMessage>>,

    mut commands: Commands,
) {
    if game_roster.is_changed() {
        for (new_address, permit) in game_roster.drain() {
            let position = Vec2::new(0., 0.); // TODO

            let common_bundle = CommonCharacterBundle::new(*next_index, permit.class, &time);
            let transform = Transform::from_xyz(position.x, position.y, 0.);
            let character_bundle =
                CharacterBundle::new(transform, common_bundle, ClientAddress(new_address));

            // Send all existing characters to new character
            let network_spawn_events =
                character_existing_query
                    .iter()
                    .map(|(index, class, transform)| {
                        let message = SpawnCharacter::new(
                            *index,
                            *class,
                            false,
                            transform.translation.truncate(),
                            transform.rotation.z,
                        );
                        let message =
                            ServerMessage::GameCommand(GameCommand::SpawnCharacter(message));
                        NetworkSendEvent::new(message, new_address, Packetting::ReliableUnordered)
                    });
            network_writer.send_batch(network_spawn_events);

            commands.spawn_bundle(character_bundle);

            // Send spawn to all existing characters and new character
            let network_spawn_events = character_address_query
                .iter()
                .map(|address| **address)
                .chain(once(new_address))
                .map(|address| {
                    let player = address == new_address;
                    let message =
                        SpawnCharacter::new(*next_index, permit.class, player, position, 0.);
                    let message = ServerMessage::GameCommand(GameCommand::SpawnCharacter(message));

                    NetworkSendEvent::new(message, address, Packetting::ReliableUnordered)
                });
            network_writer.send_batch(network_spawn_events);
            next_index.increment();
        }
    }
}
