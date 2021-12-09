use bevy::prelude::*;

use common::character::{CharacterBundle as CommonCharacterBundle, CharacterClass};

use crate::{
    character::{CharacterBundle, CharacterMaterials, PlayerBundle},
    state::ClientState,
    ui::nameplate::setup_nameplate,
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let char_spawn = SystemSet::on_enter(ClientState::Arena)
            .with_system(spawn_player.system().chain(setup_nameplate.system()))
            .with_system(spawn_char_1.system().chain(setup_nameplate.system()));

        app.add_system_set(char_spawn);
    }
}

pub fn spawn_player(
    time: Res<Time>,
    materials: Res<CharacterMaterials>,
    mut commands: Commands,
) -> Entity {
    let common = CommonCharacterBundle::new(0.into(), CharacterClass::Medea, &time);
    let character_bundle =
        CharacterBundle::new(Transform::from_xyz(50., 50., 0.), common, &materials);
    let player_bundle = PlayerBundle::new(character_bundle);
    commands.spawn_bundle(player_bundle).id()
}

pub fn spawn_char_1(
    time: Res<Time>,
    materials: Res<CharacterMaterials>,
    mut commands: Commands,
) -> Entity {
    let common = CommonCharacterBundle::new(1.into(), CharacterClass::Heka, &time);
    let character_bundle =
        CharacterBundle::new(Transform::from_xyz(-50., -50., 0.), common, &materials);
    commands.spawn_bundle(character_bundle).id()
}
