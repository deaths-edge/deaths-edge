use std::iter::once;

use bevy::prelude::*;

use common::{
    character::{
        CharacterBundle as CommonCharacterBundle, CharacterClass, CharacterIndex, CharacterMarker,
    },
    game::GameRoster,
    network::{
        server::{GameAction, ServerMessage, SpawnCharacter},
        NetworkResource,
    },
    state::ArenaState,
};

use crate::{
    character::{CharacterBundle, ClientAddress},
    network::NETWORK_HANDLE_LABEL,
};

pub const SPAWNER_LABEL: &str = "spawner";

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawner = SystemSet::on_update(ArenaState::Waiting)
            .label(SPAWNER_LABEL)
            // NETWORK_HANDLE_LABEL sends [`SpawnCharacter`] events
            .after(NETWORK_HANDLE_LABEL)
            .with_system(spawn_characters.system());

        // TODO: Don't start active
        app.add_state(ArenaState::Waiting).add_system_set(spawner);
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
    mut net: ResMut<NetworkResource>,

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
            for (index, class, transform) in character_existing_query.iter() {
                let message = SpawnCharacter::new(
                    *index,
                    *class,
                    false,
                    transform.translation.truncate(),
                    transform.rotation.z,
                );
                let message = ServerMessage::GameAction(GameAction::SpawnCharacter(message));
                net.send_message(new_address, message)
                    .expect("failed to send SpawnCharacter to new character");
            }

            commands.spawn_bundle(character_bundle);

            // Send spawn to all existing characters and new character
            let iter = character_address_query
                .iter()
                .map(|address| **address)
                .chain(once(new_address));
            for address in iter {
                let player = address == new_address;
                let message = SpawnCharacter::new(*next_index, permit.class, player, position, 0.);
                let message = ServerMessage::GameAction(GameAction::SpawnCharacter(message));

                net.send_message(address, message)
                    .expect("failed to send SpawnCharacter to existing characters");
            }
            next_index.increment();
        }
    }
}
