use bevy::prelude::*;

use common::{
    character::{mars::Mars, medea::Medea, Class, ClassTrait},
    network::server::SpawnCharacter,
    state::ArenaState,
};

use crate::{
    character::{mars::ClientMars, medea::ClientMedea, PlayerMarker, PlayerState},
    network::NETWORK_HANDLE_LABEL,
    ui::nameplate::setup_nameplate,
};

pub const SPAWN_CHARACTER_LABEL: &str = "spawn-characters";

/// Listen for [`SpawnCharacter`] event, adding characters
pub fn spawn_characters(
    mut spawn_reader: EventReader<SpawnCharacter>,

    mut player_state: ResMut<State<PlayerState>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_reader.iter() {
        let position = spawn_event.position;
        let transform = Transform::from_xyz(position.x, position.y, 0.);

        let mut entity_commands = match spawn_event.class {
            Class::Medea => {
                let character_id = Medea::spawn(
                    spawn_event.index,
                    spawn_event.team,
                    transform,
                    &mut commands,
                );
                let mut entity_commands = commands.entity(character_id);
                ClientMedea::insert_bundle(&mut entity_commands);
                entity_commands
            }
            Class::Mars => {
                let character_id = Mars::spawn(
                    spawn_event.index,
                    spawn_event.team,
                    transform,
                    &mut commands,
                );
                let mut entity_commands = commands.entity(character_id);
                ClientMars::insert_bundle(&mut entity_commands);
                entity_commands
            }
            Class::Pluto => todo!(),
            Class::Mammon => todo!(),
            Class::Nergal => todo!(),
            Class::Janus => todo!(),
            Class::Borvo => todo!(),
            Class::Heka => todo!(),
            Class::Rhea => todo!(),
        };

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
