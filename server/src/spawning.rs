use bevy::prelude::*;

use common::{
    character::{CharacterBundle as CommonCharacterBundle, CharacterIndex, CharacterMarker},
    game::GameRoster,
    network::{
        server::{ServerMessage, SpawnCharacter},
        NetworkSendEvent, Packet, Packetting,
    },
};

use crate::{
    character::{CharacterBundle, ClientAddress},
    state::ServerState,
};

// TODO: Add waiting state
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawner =
            SystemSet::on_update(ServerState::Running).with_system(spawn_characters.system());

        app.add_system_set(spawner);
    }
}

/// Spawn characters local to the server
// pub fn spawn_characters(
//     time: Res<Time>,
//     mut spawn_reader: EventReader<SpawnCharacter>,
//     mut commands: Commands,
// ) {
//     spawn_character_base(&time, &mut spawn_reader, |common_bundle, spawn_event| {
//         let position = spawn_event.position();
//         let transform = Transform::from_xyz(position.x, position.y, 0.);

//         let character_bundle = CharacterBundle::new(transform, common_bundle);
//         commands.spawn_bundle(character_bundle);
//     });
// }

pub fn spawn_characters(
    time: Res<Time>,

    mut next_index: Local<CharacterIndex>,
    mut game_roster: ResMut<GameRoster>,

    character_address_query: Query<&ClientAddress, With<CharacterMarker>>,
    // character_query: Query<&ClientAddress, With<CharacterMarker>>,
    mut network_writer: EventWriter<NetworkSendEvent<ServerMessage>>,

    mut commands: Commands,
) {
    if game_roster.is_changed() {
        info!("game roster changed");

        for (new_address, permit) in game_roster.drain() {
            let position = Vec2::new(0., 0.); // TODO

            let common_bundle = CommonCharacterBundle::new(*next_index, permit.class, &time);
            let transform = Transform::from_xyz(position.x, position.y, 0.);

            let character_bundle =
                CharacterBundle::new(transform, common_bundle, ClientAddress(new_address));

            commands.spawn_bundle(character_bundle);
            next_index.increment();

            // TODO: Send all existing characters to new character

            // Send spawn to all existing characters
            let network_spawn_events = character_address_query.iter().map(|address| {
                let player = **address == new_address;
                let message = ServerMessage::SpawnCharacter(SpawnCharacter::new(
                    *next_index,
                    permit.class,
                    player,
                    position,
                ));
                NetworkSendEvent::new(message, **address, Packetting::Unreliable)
            });
            network_writer.send_batch(network_spawn_events);
        }
    }
}
