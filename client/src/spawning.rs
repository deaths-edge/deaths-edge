use bevy::prelude::*;

use common::{
    character::CharacterBundle as CommonCharacterBundle, network::server::SpawnCharacter,
    state::ArenaState,
};

use crate::{
    character::{CharacterBundle, CharacterMaterials, PlayerBundle},
    network::NETWORK_HANDLE_LABEL,
    ui::nameplate::{setup_nameplate, NameplateMaterials},
};

pub const SPAWN_CHARACTER_LABEL: &str = "spawn-characters";

/// Listen for [`SpawnCharacter`] event, adding characters
pub fn spawn_characters(
    time: Res<Time>,
    character_materials: Res<CharacterMaterials>,
    nameplate_materials: Res<NameplateMaterials>,
    mut spawn_reader: EventReader<SpawnCharacter>,
    mut commands: Commands,
) {
    for spawn_event in spawn_reader.iter() {
        let common_bundle =
            CommonCharacterBundle::new(spawn_event.index(), spawn_event.class(), &time);
        let position = spawn_event.position();
        let transform = Transform::from_xyz(position.x, position.y, 0.);

        let character_bundle = CharacterBundle::new(transform, common_bundle, &character_materials);
        let id = if spawn_event.player() {
            let player_bundle = PlayerBundle::new(character_bundle);
            commands.spawn_bundle(player_bundle).id()
        } else {
            commands.spawn_bundle(character_bundle).id()
        };
        setup_nameplate(id, &nameplate_materials, &mut commands);
    }
}

/// While [`ArenaState::Waiting`] run [`spawn_characters`].
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawner = SystemSet::on_update(ArenaState::Waiting)
            .label(SPAWN_CHARACTER_LABEL)
            .after(NETWORK_HANDLE_LABEL)
            .with_system(spawn_characters.system());

        app.add_event::<SpawnCharacter>().add_system_set(spawner);
    }
}
