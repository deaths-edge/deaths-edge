use bevy::prelude::*;

use common::{
    character::{
        mars::Mars, medea::Medea, CharacterIndex, CharacterMarker, Class, ClassTrait, Team,
    },
    network::server::{DespawnCharacter, SpawnCharacter},
};

use crate::{
    character::{mars::ClientMars, medea::ClientMedea, PlayerMarker, PlayerState},
    network::NETWORK_HANDLE_LABEL,
    ui::hud::nameplate::{setup_nameplate, NameplateMarker, NameplateParent},
    GameState,
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
            let _ = player_state.overwrite_set(PlayerState::Spawned);
            entity_commands.insert(PlayerMarker).id()
        } else {
            info!("spawned character");
            entity_commands.id()
        };
        setup_nameplate(id, &mut commands);
    }
}

fn despawn_characters(
    mut despawn_reader: EventReader<DespawnCharacter>,
    mut player_state: ResMut<State<PlayerState>>,

    character_query: Query<(Entity, &CharacterIndex, With<PlayerMarker>), With<CharacterMarker>>,
    nameplate_query: Query<(Entity, &NameplateParent), With<NameplateMarker>>,

    mut commands: Commands,
) {
    for despawn in despawn_reader.iter() {
        info!("despawn found");
        let (id, _, is_player) = character_query
            .iter()
            .find(|(_, index, _)| despawn.index == **index)
            .expect("can't find character");

        if is_player {
            info!("is player");
            // TODO: This seems like a bug?
            // let _ = player_state.overwrite_set(PlayerState::Waiting);
        }

        // Remove character
        commands.entity(id).despawn();

        // Remove nameplate
        let (nameplate_id, _) = nameplate_query
            .iter()
            .find(|(_, parent)| parent.0 == id)
            .expect("failed to find nameplate");
        commands.entity(nameplate_id).despawn_recursive();
    }
}

// TODO: Replace this?
fn spawn_lobby_player(
    player_query: Query<&CharacterIndex, With<PlayerMarker>>,

    mut spawn_writer: EventWriter<SpawnCharacter>,
    mut despawn_writer: EventWriter<DespawnCharacter>,

    selected_char: Res<Class>,
    mut last_char: Local<Option<Class>>,
) {
    if Some(*selected_char) == *last_char {
        return;
    }

    if let Ok(&index) = player_query.get_single() {
        despawn_writer.send(DespawnCharacter { index })
    }

    spawn_writer.send(SpawnCharacter {
        index: CharacterIndex(0),
        class: *selected_char,
        player: true,
        position: Vec2::ZERO,
        rotation: 0.0,
        team: Team::Blue,
    });

    *last_char = Some(*selected_char);
}

/// While [`ArenaState::Waiting`] run [`spawn_characters`].
pub struct SpawnPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum SpawnState {
    Active,
    Inactive,
}

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        let spawner = SystemSet::on_update(SpawnState::Active)
            .label(SPAWN_CHARACTER_LABEL)
            // NETWORK_HANDLE_LABEL writes SpawnCharacter events.
            .after(NETWORK_HANDLE_LABEL)
            .with_system(spawn_characters);
        let despawner = SystemSet::on_update(SpawnState::Active)
            // NETWORK_HANDLE_LABEL writes DespawnCharacter events.
            .after(NETWORK_HANDLE_LABEL)
            .before(SPAWN_CHARACTER_LABEL)
            .with_system(despawn_characters);
        let lobby_spawn =
            SystemSet::on_update(GameState::MainLobby).with_system(spawn_lobby_player);

        app.insert_resource(Class::Mars)
            .add_state(SpawnState::Inactive)
            .add_event::<SpawnCharacter>()
            .add_event::<DespawnCharacter>()
            .add_system_set(spawner)
            .add_system_set(despawner)
            .add_system_set(lobby_spawn);
    }
}
