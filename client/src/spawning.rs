use std::{fmt::Debug, hash::Hash};

use bevy::prelude::*;

use common::{network::server::SpawnCharacter, spawning::spawn_character_base, state::ArenaState};

use crate::{
    character::{CharacterBundle, CharacterMaterials, PlayerBundle},
    ui::nameplate::{setup_nameplate, NameplateMaterials},
};

/// While [`ArenaState::Waiting`] run [`spawn_characters`].
pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spawner =
            SystemSet::on_update(ArenaState::Waiting).with_system(spawn_characters.system());

        app.add_event::<SpawnCharacter>().add_system_set(spawner);
    }
}

/// Listen for [`SpawnCharacter`] event, adding characters
pub fn spawn_characters(
    time: Res<Time>,
    character_materials: Res<CharacterMaterials>,
    nameplate_materials: Res<NameplateMaterials>,
    mut spawn_events: EventReader<SpawnCharacter>,
    mut commands: Commands,
) {
    spawn_character_base(&time, &mut spawn_events, |common_bundle, spawn_event| {
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
    });
}
