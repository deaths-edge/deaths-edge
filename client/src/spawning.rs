use bevy::prelude::*;

use common::{
    abilities::spawn_class_abilities, character::CharacterBundle, network::server::SpawnCharacter,
    state::ArenaState,
};

use crate::{
    character::{ClientCharacterBundle, PlayerMarker, PlayerState},
    network::NETWORK_HANDLE_LABEL,
    ui::nameplate::setup_nameplate,
};

pub const SPAWN_CHARACTER_LABEL: &str = "spawn-characters";

/// Listen for [`SpawnCharacter`] event, adding characters
pub fn spawn_characters(
    time: Res<Time>,

    mut spawn_reader: EventReader<SpawnCharacter>,

    mut player_state: ResMut<State<PlayerState>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_reader.iter() {
        let position = spawn_event.position;
        let transform = Transform::from_xyz(position.x, position.y, 0.);

        let common_character_bundle = CharacterBundle::new(
            spawn_event.index,
            transform,
            spawn_event.class,
            spawn_event.team,
            time.startup(),
        );
        let client_character_bundle = ClientCharacterBundle::new(&common_character_bundle);

        let mut entity_commands = commands.spawn_bundle(client_character_bundle);
        entity_commands.insert_bundle(common_character_bundle);

        let id = if spawn_event.player {
            info!("spawned player");
            player_state
                .set(PlayerState::Spawned)
                .expect("this can't happen twice");
            entity_commands.insert(PlayerMarker).id()
        } else {
            info!("spawned character");
            entity_commands.id()
        };
        setup_nameplate(id, &mut commands);
        spawn_class_abilities(id, &mut commands);
    }
}

/// While [`ArenaState::Waiting`] run [`spawn_characters`].
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        let spawner = SystemSet::on_update(ArenaState::Waiting)
            .label(SPAWN_CHARACTER_LABEL)
            // NETWORK_HANDLE_LABEL writes SpawnCharacter events.
            .after(NETWORK_HANDLE_LABEL)
            .with_system(spawn_characters);

        app.add_event::<SpawnCharacter>().add_system_set(spawner);
    }
}
