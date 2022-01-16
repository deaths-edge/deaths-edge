use std::iter::once;

use bevy::prelude::*;

use common::{
    abilities::spawn_class_abilities,
    character::{CharacterBundle, CharacterIndex, CharacterMarker, Class, Team},
    game::GameRoster,
    network::{
        server::{GameAction, ServerMessage, SpawnCharacter},
        NetworkResource,
    },
    state::ArenaState,
};

use crate::{
    character::{ClientAddress, ServerCharacterBundle},
    network::NETWORK_HANDLE_LABEL,
};

pub const SPAWNER_LABEL: &str = "spawner";

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        let spawner = SystemSet::on_update(ArenaState::Waiting)
            .label(SPAWNER_LABEL)
            // NETWORK_HANDLE_LABEL sends [`SpawnCharacter`] events
            .after(NETWORK_HANDLE_LABEL)
            .with_system(spawn_characters);

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
        (&CharacterIndex, &Class, &Team, &Transform),
        With<CharacterMarker>,
    >,
    mut net: ResMut<NetworkResource>,

    mut commands: Commands,
) {
    if game_roster.is_changed() {
        for (new_address, permit) in game_roster.drain() {
            let position = Vec2::new(0., 0.); // TODO

            let transform = Transform::from_xyz(position.x, position.y, 0.);
            let common_character_bundle = CharacterBundle::new(
                *next_index,
                transform,
                permit.class,
                permit.team,
                time.startup(),
            );
            let server_character_bundle = ServerCharacterBundle {
                address: ClientAddress(new_address),
            };

            // Send all existing characters to new character
            for (index, class, team, transform) in character_existing_query.iter() {
                let message = SpawnCharacter {
                    index: *index,
                    class: *class,
                    player: false,
                    team: *team,
                    position: transform.translation.truncate(),
                    rotation: transform.rotation.z,
                };
                let message = ServerMessage::GameAction(GameAction::SpawnCharacter(message));
                net.send_message(new_address, message)
                    .expect("failed to send SpawnCharacter to new character");
            }

            let id = commands
                .spawn_bundle(common_character_bundle)
                .insert_bundle(server_character_bundle)
                .id();
            spawn_class_abilities(id, &mut commands);

            // Send spawn to all existing characters and new character
            let iter = character_address_query
                .iter()
                .map(|address| address.0)
                .chain(once(new_address));
            for address in iter {
                let player = address == new_address;
                let message = SpawnCharacter {
                    index: *next_index,
                    class: permit.class,
                    player,
                    position,
                    team: permit.team,
                    rotation: 0.,
                };
                let message = ServerMessage::GameAction(GameAction::SpawnCharacter(message));

                net.send_message(address, message)
                    .expect("failed to send SpawnCharacter to existing characters");
            }
            next_index.increment();
        }
    }
}
